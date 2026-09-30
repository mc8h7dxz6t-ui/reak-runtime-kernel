use reak_policy_context::PolicySnapshotHandle;
use reak_types::RecordSequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeClass {
    pub class_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityRequest {
    pub principal_id: String,
    pub scope: ScopeClass,
    pub policy: PolicySnapshotHandle,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityRecord {
    pub grant_id: String,
    pub principal_id: String,
    pub scope: ScopeClass,
    pub policy_epoch: String,
    pub issued_at_sequence: RecordSequence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DenialRecord {
    pub principal_id: String,
    pub reason_code: u32,
}
