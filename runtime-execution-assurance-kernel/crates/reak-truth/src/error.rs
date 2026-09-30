#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TruthError {
    #[error("invalid derivation input")]
    InvalidInput,
    #[error("duplicate truth for reconciliation")]
    DuplicateTruth,
    #[error("replay inconsistent")]
    ReplayInconsistent,
    #[error("serialization failed")]
    Serialization,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("replay error")]
    Replay(#[from] reak_replay::ReplayError),
}
