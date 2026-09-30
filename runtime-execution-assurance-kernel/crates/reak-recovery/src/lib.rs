//! IF-RCV-01 — recovery strategy from immutable truth; never dispatches or re-derives truth.

mod engine;
mod error;
mod model;
mod strategy;

pub use engine::RecoveryEngine;
pub use error::RecoveryError;
pub use model::{
    PolicySnapshotRef, RecoveryHistoryEntry, RecoveryInput, RecoveryIntentAuthorization,
    RecoveryRecord, RecoveryReplayMetadata, RecoveryStrategy,
};
pub use strategy::select_strategy;

pub const INTERFACE: reak_types::InterfaceId = reak_types::InterfaceId::v1_0_0("IF-RCV-01");
