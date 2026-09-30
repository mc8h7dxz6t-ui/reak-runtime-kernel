mod catalog;
mod error;
mod model;

pub use catalog::RegistryCatalog;
pub use error::RegistryError;
pub use model::{ArtifactDescriptor, RegistryPointer};

pub const INTERFACE: reak_types::InterfaceId = reak_types::INTERFACE_REG_01;
