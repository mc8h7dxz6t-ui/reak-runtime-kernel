use crate::classify::{classify_next_state, input_digest_hex, validate_input};
use crate::error::ProgressionError;
use crate::model::{
    ProgressionHistoryEntry, ProgressionInput, ProgressionRecord, ProgressionReplayMetadata,
    ProgressionState,
};
use parking_lot::{Mutex, RwLock};
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordCursor, RecordEnvelope, RecordKind,
};
use reak_replay::ReplayEngine;
use reak_types::{StreamId, TenantId};
use std::collections::HashMap;

const PRG_STREAM: u64 = 0x0050_5247;

pub struct ProgressionEngine {
    tenant_id: TenantId,
    store: MemoryDurableRecordStore,
    by_recovery: RwLock<HashMap<String, String>>,
    by_id: RwLock<HashMap<String, ProgressionRecord>>,
    by_digest: RwLock<HashMap<String, String>>,
    serial: Mutex<()>,
}

impl ProgressionEngine {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            store: MemoryDurableRecordStore::new(),
            by_recovery: RwLock::new(HashMap::new()),
            by_id: RwLock::new(HashMap::new()),
            by_digest: RwLock::new(HashMap::new()),
            serial: Mutex::new(()),
        }
    }

    /// IF-PRG-01: determine next admissible step and append immutable record.
    pub fn determine_next_step(
        &self,
        input: ProgressionInput,
    ) -> Result<ProgressionRecord, ProgressionError> {
        let _guard = self.serial.lock();
        validate_input(&input)?;
        let digest = input_digest_hex(&input);
        if self.by_digest.read().contains_key(&digest)
            || self.by_recovery.read().contains_key(&input.recovery.recovery_id)
        {
            return Err(ProgressionError::DuplicateProgression);
        }

        let state = classify_next_state(&input)?;
        let provisional = ProgressionRecord {
            progression_id: String::new(),
            truth_id: input.truth.truth_id.clone(),
            recovery_id: input.recovery.recovery_id.clone(),
            dispatch_ticket_id: input.recovery.dispatch_ticket_id.clone(),
            state,
            input_digest_hex: digest,
            replay: ProgressionReplayMetadata {
                durable_sequence: reak_types::RecordSequence::ZERO,
                prev_chain_hash: reak_types::RecordHash::ZERO,
                record_chain_hash: reak_types::RecordHash::ZERO,
            },
        };

        let replay = self.append_entry(&provisional)?;
        let progression_id = format!(
            "prg_{}_{}",
            input.recovery.dispatch_ticket_id,
            replay.durable_sequence.raw()
        );
        let record = ProgressionRecord {
            progression_id,
            replay,
            ..provisional
        };

        self.by_digest
            .write()
            .insert(record.input_digest_hex.clone(), record.progression_id.clone());
        self.by_recovery
            .write()
            .insert(record.recovery_id.clone(), record.progression_id.clone());
        self.by_id
            .write()
            .insert(record.progression_id.clone(), record.clone());

        Ok(record)
    }

    fn append_entry(
        &self,
        record: &ProgressionRecord,
    ) -> Result<ProgressionReplayMetadata, ProgressionError> {
        let entry = ProgressionHistoryEntry::Determined(record.clone());
        let payload = serde_json::to_vec(&entry).map_err(|_| ProgressionError::Serialization)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(PRG_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        let stored = self
            .store
            .read_from(
                &self.tenant_id,
                StreamId::new(PRG_STREAM),
                RecordCursor {
                    sequence: ack.sequence,
                },
                1,
            )?
            .into_iter()
            .next()
            .ok_or(ProgressionError::ReplayInconsistent)?;
        Ok(ProgressionReplayMetadata {
            durable_sequence: ack.sequence,
            prev_chain_hash: stored.prev_hash,
            record_chain_hash: stored.record_hash,
        })
    }

    pub fn verify(&self) -> Result<(), ProgressionError> {
        match ReplayEngine::verify_history(&self.store, &self.tenant_id, StreamId::new(PRG_STREAM)) {
            Ok(_) => Ok(()),
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => Ok(()),
            Err(e) => Err(ProgressionError::Replay(e)),
        }
    }

    pub fn recover_from_log(&self) -> Result<usize, ProgressionError> {
        let report = match ReplayEngine::verify_history(
            &self.store,
            &self.tenant_id,
            StreamId::new(PRG_STREAM),
        ) {
            Ok(r) => r,
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => {
                *self.by_recovery.write() = HashMap::new();
                *self.by_id.write() = HashMap::new();
                *self.by_digest.write() = HashMap::new();
                return Ok(0);
            }
            Err(e) => return Err(ProgressionError::Replay(e)),
        };

        let mut by_recovery: HashMap<String, String> = HashMap::new();
        let mut by_id: HashMap<String, ProgressionRecord> = HashMap::new();
        let mut by_digest: HashMap<String, String> = HashMap::new();
        let mut count = 0usize;

        for stored in &report.snapshot {
            let entry: ProgressionHistoryEntry = serde_json::from_slice(&stored.envelope.payload)
                .map_err(|_| ProgressionError::Serialization)?;
            let ProgressionHistoryEntry::Determined(record) = entry;
            by_digest.insert(record.input_digest_hex.clone(), record.progression_id.clone());
            by_recovery.insert(record.recovery_id.clone(), record.progression_id.clone());
            by_id.insert(record.progression_id.clone(), record.clone());
            count += 1;
        }

        *self.by_recovery.write() = by_recovery;
        *self.by_id.write() = by_id;
        *self.by_digest.write() = by_digest;
        Ok(count)
    }

    /// Exposed for tests — pure classification without append.
    pub fn classify_only(&self, input: &ProgressionInput) -> Result<ProgressionState, ProgressionError> {
        classify_next_state(input)
    }
}
