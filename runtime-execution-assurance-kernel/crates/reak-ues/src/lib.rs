//! IF-UES-01 — per-stage uncertainty budgets; fail closed on violation.

mod error;
mod ledger;
mod model;

pub use error::UesError;
pub use ledger::UncertaintyLedger;
pub use model::{BudgetReceipt, StageBudget, UncertaintyDelta, ViolationRecord};

pub const INTERFACE: reak_types::InterfaceId = reak_types::INTERFACE_UES_01;
