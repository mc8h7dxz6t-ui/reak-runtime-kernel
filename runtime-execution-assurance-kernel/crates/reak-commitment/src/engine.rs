use crate::digest::{
    commitment_digest, commitment_id_from_digest, operation_digest, validate_intent_refs,
};
use crate::error::CommitmentError;
use crate::model::{
    AuthorityLink, CommitmentBinding, CommitmentDigest, CommitmentIntent, CommitmentRecord,
    CommitmentStatus, ReservationLink, ReplayMetadata,
};
use crate::state_machine;
use parking_lot::{Mutex, RwLock};
use reak_authority::ScopeClass;
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordCursor, RecordEnvelope, RecordKind,
};
use reak_exposure::{ExposureReservation, ExposureService};
use reak_policy_context::{PolicyContext, PolicyEpochRef};
use reak_replay::ReplayEngine;
use reak_types::{RecordHash, RecordSequence, StreamId, TenantId};
use reak_ues::{UncertaintyDelta, UncertaintyLedger};
use std::collections::HashMap;

const CMT_STREAM: u64 = 0x0043_4D54;

pub struct CommitmentEngine {
    tenant_id: TenantId,
    store: MemoryDurableRecordStore,
    by_id: RwLock<HashMap<String, CommitmentRecord>>,
    by_operation: RwLock<HashMap<String, String>>,
    voided: RwLock<HashMap<String, ()>>,
    bind_serial: Mutex<()>,
}

impl CommitmentEngine {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            store: MemoryDurableRecordStore::new(),
            by_id: RwLock::new(HashMap::new()),
            by_operation: RwLock::new(HashMap::new()),
            voided: RwLock::new(HashMap::new()),
            bind_serial: Mutex::new(()),
        }
    }

    pub fn durable_store(&self) -> &MemoryDurableRecordStore {
        &self.store
    }

    fn operation_key(op: &crate::model::OperationIdentity) -> String {
        format!("{}:{}", op.lineage_id, op.generation)
    }

    pub fn bind(
        &self,
        exposure: &ExposureService,
        policy: &PolicyContext,
        ues: &UncertaintyLedger,
        intent: CommitmentIntent,
    ) -> Result<CommitmentBinding, CommitmentError> {
        let _bind_guard = self.bind_serial.lock();
        let authority = exposure.authority();
        if !validate_intent_refs(&intent) {
            return Err(CommitmentError::InvalidIntent);
        }

        let op_key = Self::operation_key(&intent.operation);
        if self.by_operation.read().contains_key(&op_key) {
            return Err(CommitmentError::DuplicateOperation);
        }

        let scope = ScopeClass {
            class_id: intent.scope_class_id.clone(),
        };
        let auth_record = authority
            .verify_scope(&intent.principal_id, &scope)
            .map_err(|e| match e {
                reak_authority::AuthorityError::NotActive | reak_authority::AuthorityError::Denied => {
                    CommitmentError::AuthorityRevoked
                }
                other => CommitmentError::Authority(other),
            })?;

        let pinned = policy
            .resolve_epoch(&PolicyEpochRef {
                epoch_id: intent.policy.epoch_id.clone(),
                content_hash: intent.policy.content_hash,
            })
            .map_err(|e| match e {
                reak_policy_context::PolicyContextError::HashMismatch
                | reak_policy_context::PolicyContextError::UnknownEpoch => {
                    CommitmentError::EpochMismatch
                }
                other => CommitmentError::Policy(other),
            })?;
        if pinned.epoch_id != intent.policy.epoch_id
            || pinned.content_hash != intent.policy.content_hash
        {
            return Err(CommitmentError::EpochMismatch);
        }

        let reservation = exposure
            .reserve(ExposureReservation {
                principal_id: intent.principal_id.clone(),
                scope,
                units: intent.exposure_units,
            })
            .map_err(CommitmentError::Exposure)?;

        let ues_receipt = ues
            .consume_budget(
                crate::COMMITMENT_UES_STAGE,
                UncertaintyDelta {
                    eliminable_units: intent.ues_eliminable_units,
                },
            )
            .map_err(CommitmentError::UesExhausted)?;

        let op_dig = operation_digest(&intent.operation, &intent.intent_bytes);
        let authority_link = AuthorityLink {
            grant_id: auth_record.grant_id.clone(),
            issued_at_sequence: auth_record.issued_at_sequence,
        };
        let reservation_link = ReservationLink {
            reservation_id: reservation.reservation_id.clone(),
            units_reserved: reservation.units_reserved,
        };

        let digest_bytes = commitment_digest(
            &op_dig,
            &authority_link,
            &reservation_link,
            intent.policy.epoch_id.as_str(),
            &intent.policy.content_hash,
            &intent.provider_id,
            &intent.truth_ref,
            &intent.recovery_ref,
            &intent.progression_ref,
            intent.wall_time_unix_ms,
        );
        let digest = CommitmentDigest(digest_bytes);
        let commitment_id = commitment_id_from_digest(&digest_bytes);

        let prev_hash = self
            .store
            .read_from(
                &self.tenant_id,
                StreamId::new(CMT_STREAM),
                RecordCursor {
                    sequence: RecordSequence::ZERO,
                },
                usize::MAX,
            )
            .ok()
            .and_then(|v| v.last().map(|r| r.record_hash))
            .unwrap_or(RecordHash::ZERO);

        let record = CommitmentRecord {
            commitment_id: commitment_id.clone(),
            status: CommitmentStatus::Bound,
            operation: intent.operation.clone(),
            operation_digest: CommitmentDigest(op_dig),
            authority_link,
            reservation_link,
            policy_epoch: intent.policy.epoch_id.as_str().to_string(),
            policy_content_hash: intent.policy.content_hash,
            provider_id: intent.provider_id.clone(),
            truth_ref: intent.truth_ref.clone(),
            recovery_ref: intent.recovery_ref.clone(),
            progression_ref: intent.progression_ref.clone(),
            ues_remaining_after_bind: ues_receipt.remaining,
            wall_time_unix_ms: intent.wall_time_unix_ms,
            replay: ReplayMetadata {
                durable_sequence: RecordSequence::ZERO,
                prev_chain_hash: prev_hash,
                record_chain_hash: RecordHash::ZERO,
            },
        };

        let payload =
            serde_json::to_vec(&record).map_err(|_| CommitmentError::Serialization)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(CMT_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;

        self.store.verify_stream(&self.tenant_id, StreamId::new(CMT_STREAM))?;

        let stored = self
            .store
            .read_from(
                &self.tenant_id,
                StreamId::new(CMT_STREAM),
                RecordCursor {
                    sequence: ack.sequence,
                },
                1,
            )?
            .into_iter()
            .next()
            .ok_or(CommitmentError::ReplayInconsistent)?;

        let record = CommitmentRecord {
            replay: ReplayMetadata {
                durable_sequence: ack.sequence,
                prev_chain_hash: stored.prev_hash,
                record_chain_hash: stored.record_hash,
            },
            ..record
        };

        if record.operation_digest.0 != op_dig {
            return Err(CommitmentError::DigestMismatch);
        }

        self.by_id.write().insert(commitment_id.clone(), record.clone());
        self.by_operation.write().insert(op_key, commitment_id.clone());

        Ok(CommitmentBinding {
            record,
            digest,
        })
    }

    pub fn verify_binding(&self, commitment_id: &str, digest: &CommitmentDigest) -> Result<CommitmentRecord, CommitmentError> {
        if self.voided.read().contains_key(commitment_id) {
            return Err(CommitmentError::Voided);
        }
        let record = self
            .by_id
            .read()
            .get(commitment_id)
            .cloned()
            .ok_or(CommitmentError::NotFound)?;

        if record.status == CommitmentStatus::Voided {
            return Err(CommitmentError::Voided);
        }

        let recomputed = commitment_digest(
            &record.operation_digest.0,
            &record.authority_link,
            &record.reservation_link,
            &record.policy_epoch,
            &record.policy_content_hash,
            &record.provider_id,
            &record.truth_ref,
            &record.recovery_ref,
            &record.progression_ref,
            record.wall_time_unix_ms,
        );
        if recomputed != digest.0 {
            return Err(CommitmentError::DigestMismatch);
        }

        ReplayEngine::verify_history(
            &self.store,
            &self.tenant_id,
            StreamId::new(CMT_STREAM),
        )
        .map_err(|_| CommitmentError::ReplayInconsistent)?;

        Ok(record)
    }

    pub fn void_binding(&self, commitment_id: &str) -> Result<(), CommitmentError> {
        let record = self
            .by_id
            .read()
            .get(commitment_id)
            .cloned()
            .ok_or(CommitmentError::NotFound)?;
        if record.status == CommitmentStatus::Voided || self.voided.read().contains_key(commitment_id) {
            return Err(CommitmentError::AlreadyVoided);
        }
        if !state_machine::can_transition(record.status, CommitmentStatus::Voided) {
            return Err(CommitmentError::InvalidIntent);
        }

        let voided = CommitmentRecord {
            status: CommitmentStatus::Voided,
            ..record
        };
        let payload =
            serde_json::to_vec(&voided).map_err(|_| CommitmentError::Serialization)?;
        self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(CMT_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;

        self.voided.write().insert(commitment_id.to_string(), ());
        self.by_id.write().insert(commitment_id.to_string(), voided);
        Ok(())
    }

    /// Rebuild in-memory indexes from append-only commitment stream (crash recovery).
    pub fn recover_from_log(&self) -> Result<usize, CommitmentError> {
        let report = ReplayEngine::verify_history(
            &self.store,
            &self.tenant_id,
            StreamId::new(CMT_STREAM),
        )
        .map_err(|_| CommitmentError::ReplayInconsistent)?;

        let mut count = 0;
        let mut by_id = HashMap::new();
        let mut by_op = HashMap::new();
        let mut voided = HashMap::new();

        for stored in &report.snapshot {
            let record: CommitmentRecord = serde_json::from_slice(&stored.envelope.payload)
                .map_err(|_| CommitmentError::Serialization)?;
            by_id.insert(record.commitment_id.clone(), record.clone());
            if record.status == CommitmentStatus::Bound {
                by_op.insert(Self::operation_key(&record.operation), record.commitment_id.clone());
            }
            if record.status == CommitmentStatus::Voided {
                voided.insert(record.commitment_id.clone(), ());
            }
            count += 1;
        }

        *self.by_id.write() = by_id;
        *self.by_operation.write() = by_op;
        *self.voided.write() = voided;
        Ok(count)
    }
}
