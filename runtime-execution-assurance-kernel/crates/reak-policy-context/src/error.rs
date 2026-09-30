#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PolicyContextError {
    #[error("unknown policy epoch")]
    UnknownEpoch,
    #[error("epoch content hash mismatch")]
    HashMismatch,
    #[error("invalid epoch identifier")]
    InvalidEpoch(#[from] reak_types::IdError),
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("snapshot serialization failed")]
    Serialization,
}
