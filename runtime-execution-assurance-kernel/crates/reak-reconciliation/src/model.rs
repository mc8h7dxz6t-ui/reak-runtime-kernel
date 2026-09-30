use reak_commitment::CommitmentDigest;
use reak_dispatch::DispatchOutcome;
use reak_observation::ObservedClassification;
use reak_types::{RecordHash, RecordSequence};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentReference {
    pub commitment_id: String,
    pub commitment_digest: CommitmentDigest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyReference {
    pub epoch_id: String,
    pub content_hash: RecordHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UesReference {
    pub reconciliation_stage_remaining: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationInput {
    pub commitment_ref: CommitmentReference,
    pub dispatch_ticket_id: String,
    pub dispatch_history: Vec<reak_dispatch::DispatchHistoryEntry>,
    pub observation_history: Vec<reak_observation::ObservationRecord>,
    pub policy_ref: PolicyReference,
    pub ues_ref: UesReference,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReconciliationOutcome {
    ReconciledSuccess,
    ReconciledFailure,
    ReconciledUnknown,
    ObservationConflict,
    EvidenceInsufficient,
    DispatchNotObserved,
    UnexpectedObservation,
    DuplicateObservation,
    MultipleValidObservations,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationReplayMetadata {
    pub durable_sequence: RecordSequence,
    pub prev_chain_hash: RecordHash,
    pub record_chain_hash: RecordHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconciliationRecord {
    pub reconciliation_id: String,
    pub commitment_ref: CommitmentReference,
    pub dispatch_ticket_id: String,
    pub policy_ref: PolicyReference,
    pub ues_ref: UesReference,
    pub outcome: ReconciliationOutcome,
    pub dispatch_outcome: Option<DispatchOutcome>,
    pub observation_ids: Vec<String>,
    pub observation_classes: Vec<(String, ObservedClassification)>,
    pub input_digest_hex: String,
    pub replay: ReconciliationReplayMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReconciliationHistoryEntry {
    Reconciled(ReconciliationRecord),
}
