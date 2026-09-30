use crate::error::AuthorityError;
use crate::model::{AuthorityRecord, AuthorityRequest, ScopeClass};
use parking_lot::RwLock;
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordEnvelope, RecordKind,
};
use reak_policy_context::{PolicyContext, PolicyEpochRef};
use reak_types::{RecordSequence, StreamId, TenantId};
use std::collections::HashMap;

const AUTH_STREAM: u64 = 0x0041_5554;

pub struct AuthorityService {
    tenant_id: TenantId,
    policy: PolicyContext,
    store: MemoryDurableRecordStore,
    active: RwLock<HashMap<String, AuthorityRecord>>,
    revoked: RwLock<HashMap<String, ()>>,
}

impl AuthorityService {
    pub fn new(tenant_id: TenantId, policy: PolicyContext) -> Self {
        Self {
            tenant_id,
            policy,
            store: MemoryDurableRecordStore::new(),
            active: RwLock::new(HashMap::new()),
            revoked: RwLock::new(HashMap::new()),
        }
    }

    pub fn grant_attempt(
        &self,
        request: AuthorityRequest,
    ) -> Result<AuthorityRecord, AuthorityError> {
        if request.principal_id.is_empty() || request.scope.class_id.is_empty() {
            return Err(AuthorityError::Denied);
        }
        let _pin = self.policy.resolve_epoch(&PolicyEpochRef {
            epoch_id: request.policy.epoch_id.clone(),
            content_hash: request.policy.content_hash,
        })?;

        let grant_id = format!("{}:{}", request.principal_id, request.scope.class_id);
        if self.revoked.read().contains_key(&grant_id) {
            return Err(AuthorityError::Denied);
        }

        let record = AuthorityRecord {
            grant_id: grant_id.clone(),
            principal_id: request.principal_id,
            scope: request.scope,
            policy_epoch: request.policy.epoch_id.as_str().to_string(),
            issued_at_sequence: RecordSequence::ZERO,
        };
        let payload =
            serde_json::to_vec(&record).map_err(|_| AuthorityError::Serialization)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(AUTH_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        let record = AuthorityRecord {
            issued_at_sequence: ack.sequence,
            ..record
        };
        self.active.write().insert(grant_id, record.clone());
        Ok(record)
    }

    pub fn revoke(&self, grant_id: &str) -> Result<(), AuthorityError> {
        if !self.active.read().contains_key(grant_id) {
            return Err(AuthorityError::NotActive);
        }
        self.active.write().remove(grant_id);
        self.revoked.write().insert(grant_id.to_string(), ());
        Ok(())
    }

    pub fn verify_scope(
        &self,
        principal_id: &str,
        scope: &ScopeClass,
    ) -> Result<AuthorityRecord, AuthorityError> {
        let grant_id = format!("{}:{}", principal_id, scope.class_id);
        if self.revoked.read().contains_key(&grant_id) {
            return Err(AuthorityError::NotActive);
        }
        self.active
            .read()
            .get(&grant_id)
            .cloned()
            .ok_or(AuthorityError::NotActive)
    }
}
