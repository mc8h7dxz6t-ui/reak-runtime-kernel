use crate::derive_logic::{build_record, derive_conclusion, input_digest_hex, preserve_non_established, validate_input};
use crate::error::TruthError;
use crate::model::{
    NonEstablishedReason, TruthDerivationInput, TruthHistoryEntry, TruthRecord, TruthReplayMetadata,
};
use parking_lot::{Mutex, RwLock};
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordCursor, RecordEnvelope, RecordKind,
};
use reak_replay::ReplayEngine;
use reak_types::{StreamId, TenantId};
use std::collections::HashMap;

const TRU_STREAM: u64 = 0x0054_5255;

pub struct TruthEngine {
    tenant_id: TenantId,
    store: MemoryDurableRecordStore,
    by_reconciliation: RwLock<HashMap<String, String>>,
    by_ticket: RwLock<HashMap<String, Vec<String>>>,
    by_id: RwLock<HashMap<String, TruthRecord>>,
    by_digest: RwLock<HashMap<String, String>>,
    derive_serial: Mutex<()>,
}

impl TruthEngine {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            store: MemoryDurableRecordStore::new(),
            by_reconciliation: RwLock::new(HashMap::new()),
            by_ticket: RwLock::new(HashMap::new()),
            by_id: RwLock::new(HashMap::new()),
            by_digest: RwLock::new(HashMap::new()),
            derive_serial: Mutex::new(()),
        }
    }

    /// IF-TRU-01: derive and append immutable truth record.
    pub fn derive_conclusion(&self, input: TruthDerivationInput) -> Result<TruthRecord, TruthError> {
        self.append_derivation(input, false, None)
    }

    /// IF-TRU-01: record explicit non-established conclusion (no upgrade).
    pub fn preserve_non_established(
        &self,
        input: TruthDerivationInput,
        reason: NonEstablishedReason,
    ) -> Result<TruthRecord, TruthError> {
        self.append_derivation(input, true, Some(reason))
    }

    fn append_derivation(
        &self,
        input: TruthDerivationInput,
        preserve: bool,
        preserve_reason: Option<NonEstablishedReason>,
    ) -> Result<TruthRecord, TruthError> {
        let _guard = self.derive_serial.lock();
        validate_input(&input)?;
        let digest = input_digest_hex(&input);
        if self.by_digest.read().contains_key(&digest) {
            return Err(TruthError::DuplicateTruth);
        }
        if self
            .by_reconciliation
            .read()
            .contains_key(&input.reconciliation.reconciliation_id)
        {
            return Err(TruthError::DuplicateTruth);
        }

        let (conclusion, reason) = if preserve {
            preserve_non_established(&input, preserve_reason.unwrap())?
        } else {
            derive_conclusion(&input)?
        };

        let provisional = build_record(
            &input,
            conclusion,
            reason,
            TruthReplayMetadata {
                durable_sequence: reak_types::RecordSequence::ZERO,
                prev_chain_hash: reak_types::RecordHash::ZERO,
                record_chain_hash: reak_types::RecordHash::ZERO,
            },
            String::new(),
        );
        let replay = self.append_entry(&provisional)?;
        let truth_id = format!(
            "tru_{}_{}",
            input.reconciliation.dispatch_ticket_id,
            replay.durable_sequence.raw()
        );
        let record = build_record(&input, conclusion, reason, replay, truth_id);

        self.by_digest.write().insert(digest, record.truth_id.clone());
        self.by_reconciliation
            .write()
            .insert(record.reconciliation_id.clone(), record.truth_id.clone());
        self.by_id
            .write()
            .insert(record.truth_id.clone(), record.clone());
        self.by_ticket
            .write()
            .entry(record.dispatch_ticket_id.clone())
            .or_default()
            .push(record.truth_id.clone());

        Ok(record)
    }

    fn append_entry(&self, record: &TruthRecord) -> Result<TruthReplayMetadata, TruthError> {
        let entry = TruthHistoryEntry::Conclusion(record.clone());
        let payload = serde_json::to_vec(&entry).map_err(|_| TruthError::Serialization)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(TRU_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        let stored = self
            .store
            .read_from(
                &self.tenant_id,
                StreamId::new(TRU_STREAM),
                RecordCursor {
                    sequence: ack.sequence,
                },
                1,
            )?
            .into_iter()
            .next()
            .ok_or(TruthError::ReplayInconsistent)?;
        Ok(TruthReplayMetadata {
            durable_sequence: ack.sequence,
            prev_chain_hash: stored.prev_hash,
            record_chain_hash: stored.record_hash,
        })
    }

    pub fn verify(&self) -> Result<(), TruthError> {
        match ReplayEngine::verify_history(&self.store, &self.tenant_id, StreamId::new(TRU_STREAM)) {
            Ok(_) => Ok(()),
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => Ok(()),
            Err(e) => Err(TruthError::Replay(e)),
        }
    }

    pub fn recover_from_log(&self) -> Result<usize, TruthError> {
        let report = match ReplayEngine::verify_history(
            &self.store,
            &self.tenant_id,
            StreamId::new(TRU_STREAM),
        ) {
            Ok(r) => r,
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => {
                *self.by_reconciliation.write() = HashMap::new();
                *self.by_ticket.write() = HashMap::new();
                *self.by_id.write() = HashMap::new();
                *self.by_digest.write() = HashMap::new();
                return Ok(0);
            }
            Err(e) => return Err(TruthError::Replay(e)),
        };

        let mut by_reconciliation: HashMap<String, String> = HashMap::new();
        let mut by_ticket: HashMap<String, Vec<String>> = HashMap::new();
        let mut by_id: HashMap<String, TruthRecord> = HashMap::new();
        let mut by_digest: HashMap<String, String> = HashMap::new();
        let mut count = 0usize;

        for stored in &report.snapshot {
            let entry: TruthHistoryEntry = serde_json::from_slice(&stored.envelope.payload)
                .map_err(|_| TruthError::Serialization)?;
            let TruthHistoryEntry::Conclusion(record) = entry;
            by_digest.insert(record.input_digest_hex.clone(), record.truth_id.clone());
            by_reconciliation.insert(record.reconciliation_id.clone(), record.truth_id.clone());
            by_id.insert(record.truth_id.clone(), record.clone());
            by_ticket
                .entry(record.dispatch_ticket_id.clone())
                .or_default()
                .push(record.truth_id.clone());
            count += 1;
        }

        *self.by_reconciliation.write() = by_reconciliation;
        *self.by_ticket.write() = by_ticket;
        *self.by_id.write() = by_id;
        *self.by_digest.write() = by_digest;
        Ok(count)
    }
}
