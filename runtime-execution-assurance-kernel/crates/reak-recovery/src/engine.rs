use crate::error::RecoveryError;
use crate::model::{
    RecoveryHistoryEntry, RecoveryInput, RecoveryIntentAuthorization, RecoveryRecord,
    RecoveryReplayMetadata,
};
use crate::strategy::select_strategy;
use parking_lot::{Mutex, RwLock};
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordCursor, RecordEnvelope, RecordKind,
};
use reak_replay::ReplayEngine;
use reak_types::{RecordHash, StreamId, TenantId};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const RCV_STREAM: u64 = 0x0052_4356;

pub struct RecoveryEngine {
    tenant_id: TenantId,
    store: MemoryDurableRecordStore,
    by_truth: RwLock<HashMap<String, String>>,
    by_id: RwLock<HashMap<String, RecoveryRecord>>,
    authorized: RwLock<HashMap<String, ()>>,
    by_digest: RwLock<HashMap<String, String>>,
    serial: Mutex<()>,
}

impl RecoveryEngine {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            store: MemoryDurableRecordStore::new(),
            by_truth: RwLock::new(HashMap::new()),
            by_id: RwLock::new(HashMap::new()),
            authorized: RwLock::new(HashMap::new()),
            by_digest: RwLock::new(HashMap::new()),
            serial: Mutex::new(()),
        }
    }

    fn input_digest(input: &RecoveryInput) -> String {
        let payload = serde_json::to_vec(input).unwrap_or_default();
        hex::encode(Sha256::digest(&payload))
    }

    fn validate_input(input: &RecoveryInput) -> Result<(), RecoveryError> {
        if input.truth.truth_id.is_empty()
            || input.truth.dispatch_ticket_id.is_empty()
            || input.policy.epoch_id.is_empty()
            || input.policy.content_hash == RecordHash::ZERO
        {
            return Err(RecoveryError::InvalidInput);
        }
        Ok(())
    }

    /// IF-RCV-01: propose recovery strategy from truth; append immutable record.
    pub fn propose_recovery(&self, input: RecoveryInput) -> Result<RecoveryRecord, RecoveryError> {
        let _guard = self.serial.lock();
        Self::validate_input(&input)?;
        let digest = Self::input_digest(&input);
        if self.by_digest.read().contains_key(&digest)
            || self.by_truth.read().contains_key(&input.truth.truth_id)
        {
            return Err(RecoveryError::DuplicateRecovery);
        }

        let strategy = select_strategy(&input.truth);
        let provisional = RecoveryRecord {
            recovery_id: String::new(),
            truth_id: input.truth.truth_id.clone(),
            dispatch_ticket_id: input.truth.dispatch_ticket_id.clone(),
            strategy,
            authorizes_execution: false,
            intent_authorized: false,
            input_digest_hex: digest,
            replay: RecoveryReplayMetadata {
                durable_sequence: reak_types::RecordSequence::ZERO,
                prev_chain_hash: RecordHash::ZERO,
                record_chain_hash: RecordHash::ZERO,
            },
        };

        let replay = self.append_entry(&RecoveryHistoryEntry::Proposed(provisional.clone()))?;
        let recovery_id = format!(
            "rcv_{}_{}",
            input.truth.dispatch_ticket_id,
            replay.durable_sequence.raw()
        );
        let record = RecoveryRecord {
            recovery_id,
            replay,
            ..provisional
        };

        self.by_digest
            .write()
            .insert(record.input_digest_hex.clone(), record.recovery_id.clone());
        self.by_truth
            .write()
            .insert(record.truth_id.clone(), record.recovery_id.clone());
        self.by_id
            .write()
            .insert(record.recovery_id.clone(), record.clone());

        Ok(record)
    }

    /// IF-RCV-01: authorize intent for downstream progression (never dispatches).
    pub fn authorize_recovery_intent(
        &self,
        recovery_id: &str,
    ) -> Result<RecoveryIntentAuthorization, RecoveryError> {
        let _guard = self.serial.lock();
        let record = self
            .by_id
            .read()
            .get(recovery_id)
            .cloned()
            .ok_or(RecoveryError::NotFound)?;
        if self.authorized.read().contains_key(recovery_id) {
            return Err(RecoveryError::AlreadyAuthorized);
        }

        let auth = RecoveryIntentAuthorization {
            recovery_id: recovery_id.to_string(),
            truth_id: record.truth_id.clone(),
            authorizes_execution: false,
            replay: RecoveryReplayMetadata {
                durable_sequence: reak_types::RecordSequence::ZERO,
                prev_chain_hash: RecordHash::ZERO,
                record_chain_hash: RecordHash::ZERO,
            },
        };
        let replay =
            self.append_entry(&RecoveryHistoryEntry::IntentAuthorized(auth.clone()))?;
        let auth = RecoveryIntentAuthorization { replay, ..auth };

        self.authorized.write().insert(recovery_id.to_string(), ());
        let mut updated = record;
        updated.intent_authorized = true;
        self.by_id.write().insert(recovery_id.to_string(), updated);

        Ok(auth)
    }

    fn append_entry(
        &self,
        entry: &RecoveryHistoryEntry,
    ) -> Result<RecoveryReplayMetadata, RecoveryError> {
        let payload = serde_json::to_vec(entry).map_err(|_| RecoveryError::Serialization)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(RCV_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        let stored = self
            .store
            .read_from(
                &self.tenant_id,
                StreamId::new(RCV_STREAM),
                RecordCursor {
                    sequence: ack.sequence,
                },
                1,
            )?
            .into_iter()
            .next()
            .ok_or(RecoveryError::ReplayInconsistent)?;
        Ok(RecoveryReplayMetadata {
            durable_sequence: ack.sequence,
            prev_chain_hash: stored.prev_hash,
            record_chain_hash: stored.record_hash,
        })
    }

    pub fn verify(&self) -> Result<(), RecoveryError> {
        match ReplayEngine::verify_history(&self.store, &self.tenant_id, StreamId::new(RCV_STREAM)) {
            Ok(_) => Ok(()),
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => Ok(()),
            Err(e) => Err(RecoveryError::Replay(e)),
        }
    }

    pub fn recover_from_log(&self) -> Result<usize, RecoveryError> {
        let report = match ReplayEngine::verify_history(
            &self.store,
            &self.tenant_id,
            StreamId::new(RCV_STREAM),
        ) {
            Ok(r) => r,
            Err(reak_replay::ReplayError::DurableRecord(
                reak_durable_record::DurableRecordError::StreamNotFound,
            )) => {
                *self.by_truth.write() = HashMap::new();
                *self.by_id.write() = HashMap::new();
                *self.authorized.write() = HashMap::new();
                *self.by_digest.write() = HashMap::new();
                return Ok(0);
            }
            Err(e) => return Err(RecoveryError::Replay(e)),
        };

        let mut by_truth: HashMap<String, String> = HashMap::new();
        let mut by_id: HashMap<String, RecoveryRecord> = HashMap::new();
        let mut authorized: HashMap<String, ()> = HashMap::new();
        let mut by_digest: HashMap<String, String> = HashMap::new();
        let mut count = 0usize;

        for stored in &report.snapshot {
            let entry: RecoveryHistoryEntry = serde_json::from_slice(&stored.envelope.payload)
                .map_err(|_| RecoveryError::Serialization)?;
            match entry {
                RecoveryHistoryEntry::Proposed(record) => {
                    by_digest.insert(record.input_digest_hex.clone(), record.recovery_id.clone());
                    by_truth.insert(record.truth_id.clone(), record.recovery_id.clone());
                    by_id.insert(record.recovery_id.clone(), record);
                }
                RecoveryHistoryEntry::IntentAuthorized(auth) => {
                    authorized.insert(auth.recovery_id.clone(), ());
                    if let Some(rec) = by_id.get_mut(&auth.recovery_id) {
                        rec.intent_authorized = true;
                    }
                }
            }
            count += 1;
        }

        *self.by_truth.write() = by_truth;
        *self.by_id.write() = by_id;
        *self.authorized.write() = authorized;
        *self.by_digest.write() = by_digest;
        Ok(count)
    }
}
