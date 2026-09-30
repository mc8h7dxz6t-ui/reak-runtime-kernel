use crate::error::ObservationError;
use crate::model::{
    ObservationHistoryEntry, ObservationInput, ObservationRecord, ObservationReplayMetadata,
    PendingObservation,
};
use crate::validate::{record_pending, validate_input};
use parking_lot::{Mutex, RwLock};
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordCursor, RecordEnvelope, RecordKind,
};
use reak_replay::ReplayEngine;
use reak_types::{RecordHash, RecordSequence, StreamId, TenantId};
use std::collections::HashMap;

const OBS_STREAM: u64 = 0x004F_4253;

pub struct ObservationEngine {
    tenant_id: TenantId,
    store: MemoryDurableRecordStore,
    by_ticket: RwLock<HashMap<String, Vec<String>>>,
    by_id: RwLock<HashMap<String, ObservationRecord>>,
    source_events: RwLock<HashMap<(String, String), String>>,
    append_serial: Mutex<()>,
}

impl ObservationEngine {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            store: MemoryDurableRecordStore::new(),
            by_ticket: RwLock::new(HashMap::new()),
            by_id: RwLock::new(HashMap::new()),
            source_events: RwLock::new(HashMap::new()),
            append_serial: Mutex::new(()),
        }
    }

    /// IF-OBS-01: validate input and build pending observation (no durable write).
    pub fn record(&self, input: ObservationInput) -> Result<PendingObservation, ObservationError> {
        record_pending(input)
    }

    /// IF-OBS-01: append exactly one immutable observation record.
    pub fn append(
        &self,
        pending: PendingObservation,
        appended_at_unix_ms: u64,
    ) -> Result<ObservationRecord, ObservationError> {
        let _guard = self.append_serial.lock();
        let input = ObservationInput {
            reference: pending.reference.clone(),
            classification: pending.classification,
            raw_payload: pending.raw_payload.clone(),
            metadata: pending.metadata.clone(),
        };
        validate_input(&input)?;

        let key = (
            pending.reference.observation_source.clone(),
            pending.reference.source_event_id.clone(),
        );
        if self.source_events.read().contains_key(&key) {
            return Err(ObservationError::ReplayRejected);
        }

        let provisional_id = format!(
            "obs_{}_{}",
            pending.reference.dispatch_ticket_id,
            appended_at_unix_ms
        );

        let record = ObservationRecord {
            observation_id: provisional_id,
            reference: pending.reference,
            classification: pending.classification,
            raw_payload: pending.raw_payload,
            metadata: pending.metadata,
            content_digest_hex: pending.content_digest_hex,
            appended_at_unix_ms,
            replay: ObservationReplayMetadata {
                durable_sequence: RecordSequence::ZERO,
                prev_chain_hash: RecordHash::ZERO,
                record_chain_hash: RecordHash::ZERO,
            },
        };

        let replay = self.append_entry(&record)?;
        let observation_id = format!(
            "obs_{}_{}",
            record.reference.dispatch_ticket_id,
            replay.durable_sequence.raw()
        );
        let record = ObservationRecord {
            observation_id,
            replay,
            ..record
        };

        self.source_events
            .write()
            .insert(key, record.observation_id.clone());
        self.by_id
            .write()
            .insert(record.observation_id.clone(), record.clone());
        self.by_ticket
            .write()
            .entry(record.reference.dispatch_ticket_id.clone())
            .or_default()
            .push(record.observation_id.clone());

        Ok(record)
    }

    fn append_entry(
        &self,
        record: &ObservationRecord,
    ) -> Result<ObservationReplayMetadata, ObservationError> {
        let entry = ObservationHistoryEntry::Appended(record.clone());
        let payload = serde_json::to_vec(&entry).map_err(|_| ObservationError::Serialization)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(OBS_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        let stored = self
            .store
            .read_from(
                &self.tenant_id,
                StreamId::new(OBS_STREAM),
                RecordCursor {
                    sequence: ack.sequence,
                },
                1,
            )?
            .into_iter()
            .next()
            .ok_or(ObservationError::ReplayInconsistent)?;
        Ok(ObservationReplayMetadata {
            durable_sequence: ack.sequence,
            prev_chain_hash: stored.prev_hash,
            record_chain_hash: stored.record_hash,
        })
    }

    /// IF-OBS-01: verify durable observation stream.
    pub fn verify(&self) -> Result<(), ObservationError> {
        match ReplayEngine::verify_history(&self.store, &self.tenant_id, StreamId::new(OBS_STREAM)) {
            Ok(_) => Ok(()),
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => Ok(()),
            Err(e) => Err(ObservationError::Replay(e)),
        }
    }

    pub fn observations_for_ticket(&self, ticket_id: &str) -> Vec<ObservationRecord> {
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

    /// IF-OBS-01: rebuild indexes from log; no reconciliation.
    pub fn recover_from_log(&self) -> Result<usize, ObservationError> {
        let report = match ReplayEngine::verify_history(
            &self.store,
            &self.tenant_id,
            StreamId::new(OBS_STREAM),
        ) {
            Ok(r) => r,
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => {
                *self.by_ticket.write() = HashMap::new();
                *self.by_id.write() = HashMap::new();
                *self.source_events.write() = HashMap::new();
                return Ok(0);
            }
            Err(e) => return Err(ObservationError::Replay(e)),
        };

        let mut by_ticket: HashMap<String, Vec<String>> = HashMap::new();
        let mut by_id: HashMap<String, ObservationRecord> = HashMap::new();
        let mut source_events: HashMap<(String, String), String> = HashMap::new();
        let mut count = 0usize;

        for stored in &report.snapshot {
            let entry: ObservationHistoryEntry = serde_json::from_slice(&stored.envelope.payload)
                .map_err(|_| ObservationError::Serialization)?;
            let ObservationHistoryEntry::Appended(record) = entry;
            let key = (
                record.reference.observation_source.clone(),
                record.reference.source_event_id.clone(),
            );
            source_events.insert(key, record.observation_id.clone());
            by_id.insert(record.observation_id.clone(), record.clone());
            by_ticket
                .entry(record.reference.dispatch_ticket_id.clone())
                .or_default()
                .push(record.observation_id.clone());
            count += 1;
        }

        *self.by_ticket.write() = by_ticket;
        *self.by_id.write() = by_id;
        *self.source_events.write() = source_events;
        Ok(count)
    }
}
