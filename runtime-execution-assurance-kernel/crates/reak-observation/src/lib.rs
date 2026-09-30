//! IF-OBS-01 — capture external observations only (no dispatch, truth, or reconciliation).

mod engine;
mod error;
mod model;
pub mod state_machine;
mod validate;

pub use engine::ObservationEngine;
pub use error::ObservationError;
pub use model::{
    ObservationHistoryEntry, ObservationInput, ObservationMetadata, ObservationRecord,
    ObservationReference, ObservationReplayMetadata, ObservedClassification, PendingObservation,
};

pub const INTERFACE: reak_types::InterfaceId = reak_types::InterfaceId::v1_0_0("IF-OBS-01");
