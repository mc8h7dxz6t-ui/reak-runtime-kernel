use reak_types::IdError;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DurableRecordError {
    #[error("invalid identifier: {0}")]
    InvalidId(#[from] IdError),
    #[error("payload exceeds maximum size")]
    PayloadTooLarge,
    #[error("empty payload not allowed for this record kind")]
    EmptyPayload,
    #[error("stream not found")]
    StreamNotFound,
    #[error("supersede target sequence not found")]
    SupersedeTargetMissing,
    #[error("supersede target already superseded")]
    SupersedeTargetAlreadyReplaced,
    #[error("serialization failed")]
    Serialization,
    #[error("deserialization failed")]
    Deserialization,
    #[error("hash chain verification failed at sequence {sequence}")]
    ChainBroken { sequence: u64 },
    #[error("cursor past end of stream")]
    CursorPastEnd,
}
