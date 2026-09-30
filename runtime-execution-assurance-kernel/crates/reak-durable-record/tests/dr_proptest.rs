use proptest::prelude::*;
use reak_durable_record::{DurableRecordStore, MemoryDurableRecordStore, RecordEnvelope, RecordKind};
use reak_types::{StreamId, TenantId};

proptest! {
    #[test]
    fn append_verify_roundtrip(payload in prop::collection::vec(any::<u8>(), 0..4096)) {
        if payload.is_empty() {
            return Ok(());
        }
        let store = MemoryDurableRecordStore::new();
        let tenant = TenantId::parse("prop").unwrap();
        let stream = StreamId::new(1);
        let envelope = RecordEnvelope {
            tenant_id: tenant.clone(),
            stream_id: stream,
            schema_version: 1,
            record_kind: RecordKind::Data,
            payload,
        };
        store.append(envelope).unwrap();
        store.verify_stream(&tenant, stream).unwrap();
    }
}
