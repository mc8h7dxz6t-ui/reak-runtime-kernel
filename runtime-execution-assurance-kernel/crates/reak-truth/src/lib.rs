//! IF-TRU-01 — runtime truth conclusions from reconciliation (not recovery or progression).

mod derive_logic;
mod engine;
mod error;
mod model;

pub use derive_logic::{derive_conclusion as compute_conclusion, input_digest_hex};
pub use engine::TruthEngine;
pub use error::TruthError;
pub use model::{
    AdmissibilitySet, NonEstablishedReason, TruthConclusion, TruthDerivationInput, TruthHistoryEntry,
    TruthRecord, TruthReplayMetadata,
};

pub const INTERFACE: reak_types::InterfaceId = reak_types::InterfaceId::v1_0_0("IF-TRU-01");
