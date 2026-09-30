use crate::error::PolicyContextError;
use crate::model::{PolicyEpochRef, PolicySnapshotHandle};
use parking_lot::RwLock;
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordEnvelope, RecordKind,
};
use reak_types::{EpochId, RecordSequence, StreamId, TenantId};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const POLICY_STREAM: u64 = 0x0050_4F4C;

#[derive(Clone, Serialize, Deserialize)]
struct EpochAnnouncement {
    epoch_id: String,
    rules_blob: Vec<u8>,
}

use serde::{Deserialize, Serialize};

/// Registers epochs and pins snapshots for decision records.
pub struct PolicyContext {
    store: MemoryDurableRecordStore,
    epochs: RwLock<HashMap<String, [u8; 32]>>,
    tenant_id: TenantId,
}

impl PolicyContext {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            store: MemoryDurableRecordStore::new(),
            epochs: RwLock::new(HashMap::new()),
            tenant_id,
        }
    }

    pub fn store(&self) -> &MemoryDurableRecordStore {
        &self.store
    }

    fn hash_rules(rules: &[u8]) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(rules);
        h.finalize().into()
    }

    /// Publish a new epoch (append-only announcement).
    pub fn register_epoch(&self, epoch_id: EpochId, rules_blob: Vec<u8>) -> Result<(), PolicyContextError> {
        if rules_blob.is_empty() {
            return Err(PolicyContextError::Serialization);
        }
        let hash = Self::hash_rules(&rules_blob);
        let ann = EpochAnnouncement {
            epoch_id: epoch_id.as_str().to_string(),
            rules_blob,
        };
        let payload = serde_json::to_vec(&ann).map_err(|_| PolicyContextError::Serialization)?;
        self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(POLICY_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        self.epochs.write().insert(epoch_id.as_str().to_string(), hash);
        Ok(())
    }

    pub fn resolve_epoch(&self, reference: &PolicyEpochRef) -> Result<PolicySnapshotHandle, PolicyContextError> {
        let map = self.epochs.read();
        let expected = map
            .get(reference.epoch_id.as_str())
            .ok_or(PolicyContextError::UnknownEpoch)?;
        if *expected != reference.content_hash {
            return Err(PolicyContextError::HashMismatch);
        }
        Ok(PolicySnapshotHandle {
            epoch_id: reference.epoch_id.clone(),
            content_hash: reference.content_hash,
            pinned_at_sequence: RecordSequence::ZERO,
        })
    }

    pub fn pin_epoch(&self, reference: PolicyEpochRef) -> Result<PolicySnapshotHandle, PolicyContextError> {
        let handle = self.resolve_epoch(&reference)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(POLICY_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload: serde_json::to_vec(&reference).map_err(|_| PolicyContextError::Serialization)?,
        })?;
        Ok(PolicySnapshotHandle {
            epoch_id: handle.epoch_id,
            content_hash: handle.content_hash,
            pinned_at_sequence: ack.sequence,
        })
    }
}
