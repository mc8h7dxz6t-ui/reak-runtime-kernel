use reak_types::{ArtifactId, RecordSequence};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDescriptor {
    pub artifact_id: ArtifactId,
    pub media_type: String,
    pub content_locator: String,
    pub lineage_hint: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegistryPointer {
    pub artifact_id: ArtifactId,
    pub registered_at_sequence: RecordSequence,
    pub descriptor: ArtifactDescriptor,
}
