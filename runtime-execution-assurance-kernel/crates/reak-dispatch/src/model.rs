use reak_commitment::CommitmentDigest;
use reak_types::{RecordHash, RecordSequence};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchSpec {
    pub commitment_id: String,
    pub commitment_digest: CommitmentDigest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchTicket {
    pub ticket_id: String,
    pub commitment_id: String,
    pub attempt_number: u32,
    pub operation_lineage: String,
    pub operation_generation: u64,
    pub state: DispatchState,
    pub issued_at_unix_ms: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispatchState {
    TicketIssued,
    Released,
    AwaitingAck,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispatchOutcome {
    Success,
    Unknown,
    Failed,
    Cancelled,
    Timeout,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchReplayMetadata {
    pub durable_sequence: RecordSequence,
    pub prev_chain_hash: RecordHash,
    pub record_chain_hash: RecordHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchAttemptRecord {
    pub ticket_id: String,
    pub attempt_number: u32,
    pub released_at_unix_ms: u64,
    pub replay: DispatchReplayMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchOutcomeRecord {
    pub ticket_id: String,
    pub outcome: DispatchOutcome,
    pub classified_at_unix_ms: u64,
    pub replay: DispatchReplayMetadata,
    pub recovery_metadata: Option<String>,
    pub cancellation_reason: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispatchHistoryEntry {
    TicketIssued(DispatchTicket),
    Released(DispatchAttemptRecord),
    Outcome(DispatchOutcomeRecord),
}
