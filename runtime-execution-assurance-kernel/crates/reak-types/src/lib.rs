//! Shared domain identifiers and contract versions (IF-* metadata).
//! No runtime logic — types only.

mod ids;
mod contract;

pub use contract::{InterfaceId, INTERFACE_DR_01, INTERFACE_EXP_01, INTERFACE_AUTH_01, INTERFACE_POL_01, INTERFACE_REG_01, INTERFACE_RPL_01, INTERFACE_UES_01};
pub use ids::{
    ArtifactId, EpochId, IdError, RecordHash, RecordSequence, StageId, StreamId, TenantId,
};

/// Maximum payload size per record (1 MiB). Hostile input bound.
pub const MAX_RECORD_PAYLOAD_BYTES: usize = 1_048_576;

/// Maximum tenant / stream id length.
pub const MAX_ID_UTF8_BYTES: usize = 256;
