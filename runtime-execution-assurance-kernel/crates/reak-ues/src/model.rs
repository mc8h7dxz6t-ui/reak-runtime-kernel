use reak_types::StageId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageBudget {
    pub stage: StageId,
    pub max_eliminable_units: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UncertaintyDelta {
    pub eliminable_units: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BudgetReceipt {
    pub stage: StageId,
    pub remaining: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViolationRecord {
    pub stage: StageId,
    pub attempted: u64,
    pub remaining: u64,
}
