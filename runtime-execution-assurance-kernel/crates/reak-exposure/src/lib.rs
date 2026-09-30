mod engine;
mod error;
mod model;

pub use engine::ExposureService;
pub use error::ExposureError;
pub use model::{CeilingBreachRecord, ExposureReservation, ReservationToken};

pub const INTERFACE: reak_types::InterfaceId = reak_types::INTERFACE_EXP_01;
