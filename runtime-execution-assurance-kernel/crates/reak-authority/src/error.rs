#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AuthorityError {
    #[error("grant denied")]
    Denied,
    #[error("grant not found or revoked")]
    NotActive,
    #[error("scope mismatch")]
    ScopeMismatch,
    #[error("policy error")]
    Policy(#[from] reak_policy_context::PolicyContextError),
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("serialization failed")]
    Serialization,
}
