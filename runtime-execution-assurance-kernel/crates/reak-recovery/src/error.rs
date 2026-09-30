#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RecoveryError {
    #[error("invalid recovery input")]
    InvalidInput,
    #[error("duplicate recovery for truth record")]
    DuplicateRecovery,
    #[error("recovery record not found")]
    NotFound,
    #[error("intent already authorized")]
    AlreadyAuthorized,
    #[error("replay inconsistent")]
    ReplayInconsistent,
    #[error("serialization failed")]
    Serialization,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("replay error")]
    Replay(#[from] reak_replay::ReplayError),
}
