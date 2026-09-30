#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RegistryError {
    #[error("artifact not found")]
    NotFound,
    #[error("invalid artifact id")]
    InvalidId(#[from] reak_types::IdError),
    #[error("descriptor serialization failed")]
    Serialization,
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
}
