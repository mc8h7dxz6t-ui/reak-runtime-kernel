#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ObservationError {
    #[error("invalid observation reference")]
    InvalidReference,
    #[error("malformed or oversized payload")]
    MalformedPayload,
    #[error("observation timestamp invalid")]
    InvalidTimestamp,
    #[error("duplicate source event (replay)")]
    ReplayRejected,
    #[error("replay chain inconsistent")]
    ReplayInconsistent,
    #[error("serialization failed")]
    Serialization,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("replay error")]
    Replay(#[from] reak_replay::ReplayError),
}
