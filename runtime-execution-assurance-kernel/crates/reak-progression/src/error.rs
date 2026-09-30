#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ProgressionError {
    #[error("invalid progression input")]
    InvalidInput,
    #[error("truth and recovery mismatch")]
    BindingMismatch,
    #[error("recovery execution flag must remain false")]
    IllegalExecutionFlag,
    #[error("duplicate progression for input")]
    DuplicateProgression,
    #[error("replay inconsistent")]
    ReplayInconsistent,
    #[error("serialization failed")]
    Serialization,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("replay error")]
    Replay(#[from] reak_replay::ReplayError),
}
