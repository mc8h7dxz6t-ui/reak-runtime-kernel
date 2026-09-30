//! IF-REC-01 — reconcile commitment, dispatch, and observation histories (not Truth).

mod engine;
mod error;
mod model;
mod reconcile_logic;

pub use engine::ReconciliationEngine;
pub use error::ReconciliationError;
pub use model::{
    CommitmentReference, PolicyReference, ReconciliationHistoryEntry, ReconciliationInput,
    ReconciliationOutcome, ReconciliationRecord, ReconciliationReplayMetadata, UesReference,
};
pub use reconcile_logic::{compute_outcome, input_digest_hex};

pub const INTERFACE: reak_types::InterfaceId = reak_types::InterfaceId::v1_0_0("IF-REC-01");
