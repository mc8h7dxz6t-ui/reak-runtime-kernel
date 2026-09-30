mod engine;
mod error;
mod model;

pub use engine::AuthorityService;
pub use error::AuthorityError;
pub use model::{AuthorityRecord, AuthorityRequest, DenialRecord, ScopeClass};

pub const INTERFACE: reak_types::InterfaceId = reak_types::INTERFACE_AUTH_01;
