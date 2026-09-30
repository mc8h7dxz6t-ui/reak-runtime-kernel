use crate::error::RegistryError;
use crate::model::{ArtifactDescriptor, RegistryPointer};
use parking_lot::RwLock;
use reak_durable_record::{
    DurableRecordStore, MemoryDurableRecordStore, RecordEnvelope, RecordKind,
};
use reak_types::{ArtifactId, StreamId, TenantId};
use std::collections::HashMap;

const REG_STREAM: u64 = 0x0052_4547;

pub struct RegistryCatalog {
    tenant_id: TenantId,
    store: MemoryDurableRecordStore,
    index: RwLock<HashMap<String, RegistryPointer>>,
}

impl RegistryCatalog {
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            store: MemoryDurableRecordStore::new(),
            index: RwLock::new(HashMap::new()),
        }
    }

    pub fn durable_store(&self) -> &MemoryDurableRecordStore {
        &self.store
    }

    pub fn register_artifact(
        &self,
        descriptor: ArtifactDescriptor,
    ) -> Result<RegistryPointer, RegistryError> {
        if descriptor.media_type.is_empty() || descriptor.content_locator.is_empty() {
            return Err(RegistryError::Serialization);
        }
        let payload =
            serde_json::to_vec(&descriptor).map_err(|_| RegistryError::Serialization)?;
        let ack = self.store.append(RecordEnvelope {
            tenant_id: self.tenant_id.clone(),
            stream_id: StreamId::new(REG_STREAM),
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        })?;
        let pointer = RegistryPointer {
            artifact_id: descriptor.artifact_id.clone(),
            registered_at_sequence: ack.sequence,
            descriptor,
        };
        self.index
            .write()
            .insert(pointer.artifact_id.as_str().to_string(), pointer.clone());
        Ok(pointer)
    }

    pub fn resolve_pointer(&self, artifact_id: &ArtifactId) -> Result<RegistryPointer, RegistryError> {
        self.index
            .read()
            .get(artifact_id.as_str())
            .cloned()
            .ok_or(RegistryError::NotFound)
    }
}
