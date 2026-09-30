//! IF-POL-01 — policy epoch resolution and pinning.

mod engine;
mod error;
mod model;

pub use engine::PolicyContext;
pub use error::PolicyContextError;
pub use model::{PolicyEpochRef, PolicySnapshotHandle};

pub const INTERFACE: reak_types::InterfaceId = reak_types::INTERFACE_POL_01;
