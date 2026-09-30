use reak_policy_context::PolicySnapshotHandle;
use reak_types::{RecordHash, RecordSequence};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationIdentity {
    pub lineage_id: String,
    pub generation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaneReference {
    pub registry_pointer: String,
    pub record_sequence: RecordSequence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentIntent {
    pub operation: OperationIdentity,
    pub principal_id: String,
    pub scope_class_id: String,
    pub provider_id: String,
    pub intent_bytes: Vec<u8>,
    pub truth_ref: PlaneReference,
    pub recovery_ref: PlaneReference,
    pub progression_ref: PlaneReference,
    pub exposure_units: u64,
    pub ues_eliminable_units: u64,
    pub policy: PolicySnapshotHandle,
    pub wall_time_unix_ms: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CommitmentStatus {
    Bound,
    Voided,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentDigest(pub [u8; 32]);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityLink {
    pub grant_id: String,
    pub issued_at_sequence: RecordSequence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReservationLink {
    pub reservation_id: String,
    pub units_reserved: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMetadata {
    pub durable_sequence: RecordSequence,
    pub prev_chain_hash: RecordHash,
    pub record_chain_hash: RecordHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentRecord {
    pub commitment_id: String,
    pub status: CommitmentStatus,
    pub operation: OperationIdentity,
    pub operation_digest: CommitmentDigest,
    pub authority_link: AuthorityLink,
    pub reservation_link: ReservationLink,
    pub policy_epoch: String,
    pub policy_content_hash: [u8; 32],
    pub provider_id: String,
    pub truth_ref: PlaneReference,
    pub recovery_ref: PlaneReference,
    pub progression_ref: PlaneReference,
    pub ues_remaining_after_bind: u64,
    pub wall_time_unix_ms: u64,
    pub replay: ReplayMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitmentBinding {
    pub record: CommitmentRecord,
    pub digest: CommitmentDigest,
}
