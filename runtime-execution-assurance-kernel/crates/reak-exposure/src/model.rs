use reak_authority::ScopeClass;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExposureReservation {
    pub principal_id: String,
    pub scope: ScopeClass,
    pub units: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReservationToken {
    pub reservation_id: String,
    pub units_reserved: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CeilingBreachRecord {
    pub principal_id: String,
    pub attempted_units: u64,
    pub ceiling: u64,
}
