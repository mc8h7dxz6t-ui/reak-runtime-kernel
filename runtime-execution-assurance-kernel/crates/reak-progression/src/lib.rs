//! IF-PRG-01 — final runtime decision: next admissible step from truth + recovery (no dispatch).

mod classify;
mod engine;
mod error;
mod model;

pub use classify::{classify_next_state, input_digest_hex};
pub use engine::ProgressionEngine;
pub use error::ProgressionError;
pub use model::{
    ProgressionHistoryEntry, ProgressionInput, ProgressionRecord, ProgressionReplayMetadata,
    ProgressionState,
};

pub const INTERFACE: reak_types::InterfaceId = reak_types::InterfaceId::v1_0_0("IF-PRG-01");
