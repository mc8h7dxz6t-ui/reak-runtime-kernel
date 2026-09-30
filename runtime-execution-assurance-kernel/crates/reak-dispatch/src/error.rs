#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DispatchError {
    #[error("commitment verification failed")]
    Commitment(#[from] reak_commitment::CommitmentError),
    #[error("authority no longer valid")]
    AuthorityRevoked,
    #[error("authority error")]
    Authority(#[from] reak_authority::AuthorityError),
    #[error("policy epoch mismatch")]
    PolicyMismatch,
    #[error("policy error")]
    Policy(#[from] reak_policy_context::PolicyContextError),
    #[error("commitment digest mismatch")]
    DigestMismatch,
    #[error("replay inconsistent")]
    ReplayInconsistent,
    #[error("commitment voided or invalid status")]
    CommitmentNotDispatchable,
    #[error("duplicate dispatch for commitment")]
    DuplicateDispatch,
    #[error("dispatch ticket not found")]
    TicketNotFound,
    #[error("illegal state transition")]
    IllegalTransition,
    #[error("dispatch already terminal")]
    AlreadyTerminal,
    #[error("concurrent dispatch conflict")]
    ConcurrentDispatch,
    #[error("reservation no longer valid")]
    ReservationInvalid,
    #[error("ues precondition failed")]
    UesPrecondition,
    #[error("unknown acknowledgement requires manual resolution")]
    UnknownAck,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("serialization failed")]
    Serialization,
}
