use reak_truth::TruthRecord;
use reak_types::{RecordHash, RecordSequence};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySnapshotRef {
    pub epoch_id: String,
    pub content_hash: RecordHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryInput {
    pub truth: TruthRecord,
    pub policy: PolicySnapshotRef,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    NoAction,
    Retry,
    Compensate,
    Escalate,
    Await,
    Abort,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryReplayMetadata {
    pub durable_sequence: RecordSequence,
    pub prev_chain_hash: RecordHash,
    pub record_chain_hash: RecordHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRecord {
    pub recovery_id: String,
    pub truth_id: String,
    pub dispatch_ticket_id: String,
    pub strategy: RecoveryStrategy,
    /// IF-RCV-01 invariant: recovery never dispatches directly.
    pub authorizes_execution: bool,
    pub intent_authorized: bool,
    pub input_digest_hex: String,
    pub replay: RecoveryReplayMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryIntentAuthorization {
    pub recovery_id: String,
    pub truth_id: String,
    pub authorizes_execution: bool,
    pub replay: RecoveryReplayMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryHistoryEntry {
    Proposed(RecoveryRecord),
    IntentAuthorized(RecoveryIntentAuthorization),
}
