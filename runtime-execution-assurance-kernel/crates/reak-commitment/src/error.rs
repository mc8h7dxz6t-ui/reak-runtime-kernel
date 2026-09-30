#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CommitmentError {
    #[error("authority not valid for bind")]
    AuthorityRevoked,
    #[error("authority verification failed")]
    Authority(#[from] reak_authority::AuthorityError),
    #[error("exposure reservation failed")]
    Exposure(#[from] reak_exposure::ExposureError),
    #[error("policy epoch mismatch")]
    EpochMismatch,
    #[error("policy error")]
    Policy(#[from] reak_policy_context::PolicyContextError),
    #[error("uncertainty budget exhausted")]
    UesExhausted(#[from] reak_ues::UesError),
    #[error("duplicate operation commitment")]
    DuplicateOperation,
    #[error("commitment digest mismatch")]
    DigestMismatch,
    #[error("commitment not found")]
    NotFound,
    #[error("commitment voided")]
    Voided,
    #[error("commitment already voided")]
    AlreadyVoided,
    #[error("replay chain invalid")]
    ReplayInconsistent,
    #[error("unknown dependency reference")]
    UnknownDependency,
    #[error("reservation no longer valid")]
    ReservationExpired,
    #[error("invalid intent")]
    InvalidIntent,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("serialization failed")]
    Serialization,
}
