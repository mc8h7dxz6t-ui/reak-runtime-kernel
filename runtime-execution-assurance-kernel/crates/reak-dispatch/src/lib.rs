//! IF-DSP-01 — sole owner of outbound dispatch attempts (no provider I/O).

mod engine;
mod error;
mod model;
pub mod state_machine;
mod verify;

pub use engine::DispatchEngine;
pub use error::DispatchError;
pub use model::{
    DispatchAttemptRecord, DispatchHistoryEntry, DispatchOutcome, DispatchOutcomeRecord,
    DispatchReplayMetadata, DispatchSpec, DispatchState, DispatchTicket,
};

pub const INTERFACE: reak_types::InterfaceId = reak_types::InterfaceId::v1_0_0("IF-DSP-01");
