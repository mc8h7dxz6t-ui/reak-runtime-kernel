use reak_recovery::{PolicySnapshotRef, RecoveryIntentAuthorization, RecoveryRecord};
use reak_truth::TruthRecord;
use reak_types::{RecordHash, RecordSequence};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressionInput {
    pub truth: TruthRecord,
    pub recovery: RecoveryRecord,
    pub authorization: RecoveryIntentAuthorization,
    pub policy: PolicySnapshotRef,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressionState {
    Terminal,
    Await,
    RetryPermitted,
    CompensationPermitted,
    HumanApprovalRequired,
    Abort,
    NoFurtherAction,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressionReplayMetadata {
    pub durable_sequence: RecordSequence,
    pub prev_chain_hash: RecordHash,
    pub record_chain_hash: RecordHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressionRecord {
    pub progression_id: String,
    pub truth_id: String,
    pub recovery_id: String,
    pub dispatch_ticket_id: String,
    pub state: ProgressionState,
    pub input_digest_hex: String,
    pub replay: ProgressionReplayMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressionHistoryEntry {
    Determined(ProgressionRecord),
}
