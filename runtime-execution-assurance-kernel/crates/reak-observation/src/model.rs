use reak_types::{RecordHash, RecordSequence};
use serde::{Deserialize, Serialize};

/// What the outside world reported — classification is supplied by the caller; never inferred here.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservedClassification {
    ObservedSuccess,
    ObservedFailure,
    ObservedUnknown,
    ObservedTimeout,
    ObservedCancelled,
    ObservationUnavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationReference {
    pub dispatch_ticket_id: String,
    pub provider_id: String,
    pub provider_operation_id: String,
    pub correlation_ids: Vec<String>,
    pub observed_at_unix_ms: u64,
    pub observation_source: String,
    /// Adapter-supplied idempotency key; duplicate `(source, event_id)` is rejected (replay).
    pub source_event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationMetadata {
    pub labels: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationInput {
    pub reference: ObservationReference,
    pub classification: ObservedClassification,
    pub raw_payload: Vec<u8>,
    pub metadata: ObservationMetadata,
}

/// Validated, not yet durable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingObservation {
    pub reference: ObservationReference,
    pub classification: ObservedClassification,
    pub raw_payload: Vec<u8>,
    pub metadata: ObservationMetadata,
    pub content_digest_hex: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationReplayMetadata {
    pub durable_sequence: RecordSequence,
    pub prev_chain_hash: RecordHash,
    pub record_chain_hash: RecordHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationRecord {
    pub observation_id: String,
    pub reference: ObservationReference,
    pub classification: ObservedClassification,
    pub raw_payload: Vec<u8>,
    pub metadata: ObservationMetadata,
    pub content_digest_hex: String,
    pub appended_at_unix_ms: u64,
    pub replay: ObservationReplayMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationHistoryEntry {
    Appended(ObservationRecord),
}
