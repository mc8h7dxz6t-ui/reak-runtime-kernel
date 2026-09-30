#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ExposureError {
    #[error("authority not valid for reservation")]
    Authority(#[from] reak_authority::AuthorityError),
    #[error("ceiling breached")]
    CeilingBreached,
    #[error("reservation not found")]
    ReservationNotFound,
    #[error("invalid reservation amount")]
    InvalidAmount,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("serialization failed")]
    Serialization,
}
