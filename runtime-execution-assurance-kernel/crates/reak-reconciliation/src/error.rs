#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ReconciliationError {
    #[error("invalid reconciliation input")]
    InvalidInput,
    #[error("missing commitment reference")]
    MissingCommitment,
    #[error("missing dispatch for ticket")]
    MissingDispatch,
    #[error("commitment reference does not match dispatch")]
    ForgedCommitmentReference,
    #[error("duplicate dispatch ticket in history")]
    DuplicateDispatchId,
    #[error("duplicate reconciliation for input digest")]
    DuplicateReconciliation,
    #[error("replay inconsistent")]
    ReplayInconsistent,
    #[error("serialization failed")]
    Serialization,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("replay error")]
    Replay(#[from] reak_replay::ReplayError),
}
