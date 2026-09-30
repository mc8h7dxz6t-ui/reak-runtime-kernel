#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ReplayError {
    #[error("durable record error")]
    DurableRecord(#[from] reak_durable_record::DurableRecordError),
    #[error("registry error")]
    Registry(#[from] reak_registry::RegistryError),
    #[error("chain verification failed")]
    ChainInvalid,
}
