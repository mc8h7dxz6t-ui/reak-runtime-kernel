use crate::error::ReplayError;
use crate::model::ReplayReport;
use reak_durable_record::{DurableRecordStore, RecordCursor};
use reak_registry::RegistryCatalog;
use reak_types::{RecordSequence, StreamId, TenantId};

pub struct ReplayEngine;

impl ReplayEngine {
    /// Verify hash chain and return read-only snapshot. Non-normative for future action.
    pub fn verify_history(
        store: &impl DurableRecordStore,
        tenant_id: &TenantId,
        stream_id: StreamId,
    ) -> Result<ReplayReport, ReplayError> {
        store.verify_stream(tenant_id, stream_id)?;
        let snapshot = store.read_from(
            tenant_id,
            stream_id,
            RecordCursor {
                sequence: RecordSequence::new(1),
            },
            usize::MAX,
        )?;
        let last = snapshot
            .last()
            .map(|r| r.sequence)
            .unwrap_or(RecordSequence::ZERO);
        Ok(ReplayReport {
            records_verified: snapshot.len(),
            last_sequence: last,
            chain_valid: true,
            snapshot,
        })
    }

    /// Cross-check registry pointers exist for audit reconstruction metadata.
    pub fn reconstruct_view(
        store: &impl DurableRecordStore,
        registry: &RegistryCatalog,
        tenant_id: &TenantId,
        stream_id: StreamId,
        artifact_id: &reak_types::ArtifactId,
    ) -> Result<ReplayReport, ReplayError> {
        let _pointer = registry.resolve_pointer(artifact_id)?;
        Self::verify_history(store, tenant_id, stream_id)
    }
}
