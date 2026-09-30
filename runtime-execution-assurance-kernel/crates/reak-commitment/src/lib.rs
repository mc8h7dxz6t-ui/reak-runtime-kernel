//! IF-CMT-01 — Runtime commitment engine (bind-before-effect).
//!
//! Does not dispatch, observe, reconcile, or call external systems.

mod digest;
mod engine;
mod error;
mod model;
mod state_machine;

pub use engine::CommitmentEngine;
pub use error::CommitmentError;
pub use model::{
    AuthorityLink, CommitmentBinding, CommitmentDigest, CommitmentIntent, CommitmentRecord,
    CommitmentStatus, OperationIdentity, PlaneReference, ReplayMetadata, ReservationLink,
};

pub const INTERFACE: reak_types::InterfaceId = reak_types::InterfaceId::v1_0_0("IF-CMT-01");

/// UES stage id for commitment binds (foundation stage namespace).
pub const COMMITMENT_UES_STAGE: reak_types::StageId = reak_types::StageId::new(10);
