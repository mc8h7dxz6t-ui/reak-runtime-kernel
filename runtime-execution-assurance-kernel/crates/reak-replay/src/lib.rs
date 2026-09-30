//! IF-RPL-01 — verify and reconstruct; **must not** drive future dispatch.

mod engine;
mod error;
mod model;

pub use engine::ReplayEngine;
pub use error::ReplayError;
pub use model::ReplayReport;

pub const INTERFACE: reak_types::InterfaceId = reak_types::INTERFACE_RPL_01;
