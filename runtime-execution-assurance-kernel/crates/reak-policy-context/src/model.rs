use reak_types::{EpochId, RecordSequence};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEpochRef {
    pub epoch_id: EpochId,
    pub content_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySnapshotHandle {
    pub epoch_id: EpochId,
    pub content_hash: [u8; 32],
    pub pinned_at_sequence: RecordSequence,
}
