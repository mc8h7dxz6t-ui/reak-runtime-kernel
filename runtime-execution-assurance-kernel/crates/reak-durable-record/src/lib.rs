//! IF-DR-01 — append-only durable record with hash chain.
//!
//! # Threat considerations
//! - Hostile payloads: bounded size, reject on deserialize failure.
//! - Tampering: each record links `prev_hash`; verification walks the chain.
//! - Concurrency: one writer per `(tenant, stream)` via shard locks; reads are consistent per cursor.
//! - Privilege: this module stores bytes only; semantic ownership is the caller's plane.

mod chain;
mod error;
mod model;
mod store;

pub use error::DurableRecordError;
pub use model::{
    AppendAck, RecordCursor, RecordEnvelope, RecordKind, StoredRecord, SupersedeLink,
};
pub use store::{DurableRecordStore, MemoryDurableRecordStore};

pub const INTERFACE: reak_types::InterfaceId = reak_types::INTERFACE_DR_01;

#[cfg(test)]
mod unit_tests {
    use super::*;
    use reak_types::{StreamId, TenantId};

    #[test]
    fn supersede_marks_target() {
        let store = MemoryDurableRecordStore::new();
        let tenant = TenantId::parse("t").unwrap();
        let stream = StreamId::new(1);
        store
            .append(RecordEnvelope {
                tenant_id: tenant.clone(),
                stream_id: stream,
                schema_version: 1,
                record_kind: RecordKind::Data,
                payload: vec![1],
            })
            .unwrap();
        store
            .supersede(
                RecordEnvelope {
                    tenant_id: tenant.clone(),
                    stream_id: stream,
                    schema_version: 1,
                    record_kind: RecordKind::Data,
                    payload: vec![2],
                },
                SupersedeLink {
                    target_sequence: RecordSequence::new(1),
                    reason_code: 1,
                },
            )
            .unwrap();
        store.verify_stream(&tenant, stream).unwrap();
    }

    use reak_types::RecordSequence;
}
