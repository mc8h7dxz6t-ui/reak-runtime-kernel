use reak_durable_record::{DurableRecordStore, MemoryDurableRecordStore, RecordEnvelope, RecordKind};
use reak_replay::ReplayEngine;
use reak_types::{StreamId, TenantId};

#[test]
fn verify_empty_stream_fails() {
    let store = MemoryDurableRecordStore::new();
    let tenant = TenantId::parse("t").unwrap();
    let stream = StreamId::new(99);
    assert!(ReplayEngine::verify_history(&store, &tenant, stream).is_err());
}

#[test]
fn verify_nonempty_stream() {
    let store = MemoryDurableRecordStore::new();
    let tenant = TenantId::parse("t").unwrap();
    let stream = StreamId::new(1);
    store
        .append(RecordEnvelope {
            tenant_id: tenant.clone(),
            stream_id: stream,
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload: vec![42],
        })
        .unwrap();
    let report = ReplayEngine::verify_history(&store, &tenant, stream).unwrap();
    assert_eq!(report.records_verified, 1);
    assert!(report.chain_valid);
}
