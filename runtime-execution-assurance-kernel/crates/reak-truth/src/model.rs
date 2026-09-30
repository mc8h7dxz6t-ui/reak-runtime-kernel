use reak_reconciliation::ReconciliationRecord;
use reak_types::{RecordHash, RecordSequence};
use serde::{Deserialize, Serialize};

/// Caller-supplied admissibility snapshot (IF-EVD-01 hooks are out of kernel scope for Phase 7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmissibilitySet {
    pub catalogue_handles: Vec<String>,
    pub all_required_admissible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonEstablishedReason {
    ReconciliationUnknown,
    ObservationConflict,
    EvidenceInsufficient,
    DispatchNotObserved,
    UnexpectedObservation,
    DuplicateObservation,
    MultipleValidObservations,
    AdmissibilityInsufficient,
    ReconciliationNotEstablishing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TruthConclusion {
    EstablishedSuccess,
    EstablishedFailure,
    NonEstablished,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruthDerivationInput {
    pub reconciliation: ReconciliationRecord,
    pub admissibility: AdmissibilitySet,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruthReplayMetadata {
    pub durable_sequence: RecordSequence,
    pub prev_chain_hash: RecordHash,
    pub record_chain_hash: RecordHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruthRecord {
    pub truth_id: String,
    pub reconciliation_id: String,
    pub dispatch_ticket_id: String,
    pub conclusion: TruthConclusion,
    pub non_established_reason: Option<NonEstablishedReason>,
    pub input_digest_hex: String,
    pub replay: TruthReplayMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TruthHistoryEntry {
    Conclusion(TruthRecord),
}
