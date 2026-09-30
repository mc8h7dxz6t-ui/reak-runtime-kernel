use crate::error::ReconciliationError;
use crate::model::{ReconciliationHistoryEntry, ReconciliationInput, ReconciliationRecord, ReconciliationReplayMetadata};
use crate::reconcile_logic::{build_record, compute_outcome, input_digest_hex, validate_input};
use parking_lot::{Mutex, RwLock};
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordCursor, RecordEnvelope, RecordKind,
};
use reak_replay::ReplayEngine;
use reak_types::{StreamId, TenantId};
use std::collections::HashMap;

const REC_STREAM: u64 = 0x0052_4543;

pub struct ReconciliationEngine {
    tenant_id: TenantId,
    store: MemoryDurableRecordStore,
    by_ticket: RwLock<HashMap<String, Vec<String>>>,
    by_id: RwLock<HashMap<String, ReconciliationRecord>>,
    by_digest: RwLock<HashMap<String, String>>,
    reconcile_serial: Mutex<()>,
}

impl ReconciliationEngine {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            store: MemoryDurableRecordStore::new(),
            by_ticket: RwLock::new(HashMap::new()),
            by_id: RwLock::new(HashMap::new()),
            by_digest: RwLock::new(HashMap::new()),
            reconcile_serial: Mutex::new(()),
        }
    }

    /// IF-REC-01: deterministic compare of histories; append immutable reconciliation record.
    pub fn reconcile(&self, input: ReconciliationInput) -> Result<ReconciliationRecord, ReconciliationError> {
        let _guard = self.reconcile_serial.lock();
        validate_input(&input)?;
        let digest = input_digest_hex(&input);
        if self.by_digest.read().contains_key(&digest) {
            return Err(ReconciliationError::DuplicateReconciliation);
        }

        let outcome = compute_outcome(&input)?;
        let provisional = build_record(
            &input,
            outcome,
            ReconciliationReplayMetadata {
                durable_sequence: reak_types::RecordSequence::ZERO,
                prev_chain_hash: reak_types::RecordHash::ZERO,
                record_chain_hash: reak_types::RecordHash::ZERO,
            },
            String::new(),
        );

        let replay = self.append_entry(&provisional)?;
        let reconciliation_id = format!(
            "rec_{}_{}",
            input.dispatch_ticket_id,
            replay.durable_sequence.raw()
        );
        let record = build_record(&input, outcome, replay, reconciliation_id);

        self.by_digest.write().insert(digest, record.reconciliation_id.clone());
        self.by_id
            .write()
            .insert(record.reconciliation_id.clone(), record.clone());
        self.by_ticket
            .write()
            .entry(record.dispatch_ticket_id.clone())
            .or_default()
            .push(record.reconciliation_id.clone());

        Ok(record)
    }

    fn append_entry(
        &self,
        record: &ReconciliationRecord,
    ) -> Result<ReconciliationReplayMetadata, ReconciliationError> {
        let entry = ReconciliationHistoryEntry::Reconciled(record.clone());
        let payload = serde_json::to_vec(&entry).map_err(|_| ReconciliationError::Serialization)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(REC_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        let stored = self
            .store
            .read_from(
                &self.tenant_id,
                StreamId::new(REC_STREAM),
                RecordCursor {
                    sequence: ack.sequence,
                },
                1,
            )?
            .into_iter()
            .next()
            .ok_or(ReconciliationError::ReplayInconsistent)?;
        Ok(ReconciliationReplayMetadata {
            durable_sequence: ack.sequence,
            prev_chain_hash: stored.prev_hash,
            record_chain_hash: stored.record_hash,
        })
    }

    pub fn verify(&self) -> Result<(), ReconciliationError> {
        match ReplayEngine::verify_history(&self.store, &self.tenant_id, StreamId::new(REC_STREAM)) {
            Ok(_) => Ok(()),
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => Ok(()),
            Err(e) => Err(ReconciliationError::Replay(e)),
        }
    }

    pub fn records_for_ticket(&self, ticket_id: &str) -> Vec<ReconciliationRecord> {
        let ids = self
            .by_ticket
            .read()
            .get(ticket_id)
            .cloned()
            .unwrap_or_default();
        let map = self.by_id.read();
        ids.iter()
            .filter_map(|id| map.get(id).cloned())
            .collect()
    }

    pub fn recover_from_log(&self) -> Result<usize, ReconciliationError> {
        let report = match ReplayEngine::verify_history(
            &self.store,
            &self.tenant_id,
            StreamId::new(REC_STREAM),
        ) {
            Ok(r) => r,
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => {
                *self.by_ticket.write() = HashMap::new();
                *self.by_id.write() = HashMap::new();
                *self.by_digest.write() = HashMap::new();
                return Ok(0);
            }
            Err(e) => return Err(ReconciliationError::Replay(e)),
        };

        let mut by_ticket: HashMap<String, Vec<String>> = HashMap::new();
        let mut by_id: HashMap<String, ReconciliationRecord> = HashMap::new();
        let mut by_digest: HashMap<String, String> = HashMap::new();
        let mut count = 0usize;

        for stored in &report.snapshot {
            let entry: ReconciliationHistoryEntry = serde_json::from_slice(&stored.envelope.payload)
                .map_err(|_| ReconciliationError::Serialization)?;
            let ReconciliationHistoryEntry::Reconciled(record) = entry;
            by_digest.insert(record.input_digest_hex.clone(), record.reconciliation_id.clone());
            by_id.insert(record.reconciliation_id.clone(), record.clone());
            by_ticket
                .entry(record.dispatch_ticket_id.clone())
                .or_default()
                .push(record.reconciliation_id.clone());
            count += 1;
        }

        *self.by_ticket.write() = by_ticket;
        *self.by_id.write() = by_id;
        *self.by_digest.write() = by_digest;
        Ok(count)
    }
}
