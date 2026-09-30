#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum UesError {
    #[error("stage budget not declared")]
    BudgetNotDeclared,
    #[error("uncertainty budget exhausted")]
    BudgetExhausted,
    #[error("invalid delta")]
    InvalidDelta,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("serialization failed")]
    Serialization,
}
