use crate::chain::{hash_record, verify_chain};
use crate::error::DurableRecordError;
use crate::model::{
    AppendAck, RecordCursor, RecordEnvelope, RecordKind, StoredRecord, SupersedeLink,
};
use parking_lot::RwLock;
use reak_types::{RecordHash, RecordSequence, StreamId, TenantId, MAX_RECORD_PAYLOAD_BYTES};
use std::collections::HashMap;

type StreamKey = (String, u64);

pub trait DurableRecordStore: Send + Sync {
    fn append(&self, envelope: RecordEnvelope) -> Result<AppendAck, DurableRecordError>;

    fn supersede(
        &self,
        envelope: RecordEnvelope,
        link: SupersedeLink,
    ) -> Result<AppendAck, DurableRecordError>;

    fn read_from(
        &self,
        tenant_id: &TenantId,
        stream_id: StreamId,
        cursor: RecordCursor,
        limit: usize,
    ) -> Result<Vec<StoredRecord>, DurableRecordError>;

    fn verify_stream(
        &self,
        tenant_id: &TenantId,
        stream_id: StreamId,
    ) -> Result<(), DurableRecordError>;
}

#[derive(Default)]
pub struct MemoryDurableRecordStore {
    streams: RwLock<HashMap<StreamKey, Vec<StoredRecord>>>,
    superseded: RwLock<HashMap<StreamKey, HashMap<u64, ()>>>,
}

impl MemoryDurableRecordStore {
    pub fn new() -> Self {
        Self::default()
    }

    fn key(tenant: &TenantId, stream: StreamId) -> StreamKey {
        (tenant.as_str().to_string(), stream.raw())
    }

    fn validate_envelope(envelope: &RecordEnvelope) -> Result<(), DurableRecordError> {
        if envelope.payload.len() > MAX_RECORD_PAYLOAD_BYTES {
            return Err(DurableRecordError::PayloadTooLarge);
        }
        if envelope.payload.is_empty() && envelope.record_kind == RecordKind::Data {
            return Err(DurableRecordError::EmptyPayload);
        }
        Ok(())
    }

    fn append_internal(
        &self,
        envelope: RecordEnvelope,
        supersede: Option<SupersedeLink>,
    ) -> Result<AppendAck, DurableRecordError> {
        Self::validate_envelope(&envelope)?;
        let key = Self::key(&envelope.tenant_id, envelope.stream_id);

        if let Some(link) = &supersede {
            let superseded_map = self.superseded.read();
            if let Some(map) = superseded_map.get(&key) {
                if map.contains_key(&link.target_sequence.raw()) {
                    return Err(DurableRecordError::SupersedeTargetAlreadyReplaced);
                }
            }
            let streams = self.streams.read();
            let vec = streams.get(&key).ok_or(DurableRecordError::StreamNotFound)?;
            if link.target_sequence.raw() as usize > vec.len() || link.target_sequence.raw() == 0 {
                return Err(DurableRecordError::SupersedeTargetMissing);
            }
        }

        let mut streams = self.streams.write();
        let vec = streams.entry(key.clone()).or_default();

        let sequence = RecordSequence::new(vec.len() as u64 + 1);
        let prev_hash = vec
            .last()
            .map(|r| r.record_hash)
            .unwrap_or(RecordHash::ZERO);

        let record_hash = hash_record(&prev_hash, sequence.raw(), &envelope, supersede.as_ref());

        let stored = StoredRecord {
            sequence,
            prev_hash,
            record_hash,
            envelope,
            supersede,
        };

        if let Some(link) = stored.supersede.clone() {
            self.superseded
                .write()
                .entry(key)
                .or_default()
                .insert(link.target_sequence.raw(), ());
        }

        vec.push(stored);
        Ok(AppendAck {
            sequence,
            record_hash,
        })
    }
}

impl DurableRecordStore for MemoryDurableRecordStore {
    fn append(&self, envelope: RecordEnvelope) -> Result<AppendAck, DurableRecordError> {
        if envelope.record_kind != RecordKind::Data {
            return Err(DurableRecordError::EmptyPayload);
        }
        self.append_internal(envelope, None)
    }

    fn supersede(
        &self,
        envelope: RecordEnvelope,
        link: SupersedeLink,
    ) -> Result<AppendAck, DurableRecordError> {
        let mut env = envelope;
        env.record_kind = RecordKind::Supersede;
        self.append_internal(env, Some(link))
    }

    fn read_from(
        &self,
        tenant_id: &TenantId,
        stream_id: StreamId,
        cursor: RecordCursor,
        limit: usize,
    ) -> Result<Vec<StoredRecord>, DurableRecordError> {
        let key = Self::key(tenant_id, stream_id);
        let streams = self.streams.read();
        let vec = streams.get(&key).ok_or(DurableRecordError::StreamNotFound)?;
        let seq = cursor.sequence.raw();
        let start = if seq == 0 {
            0
        } else {
            (seq - 1) as usize
        };
        if start > vec.len() {
            return Err(DurableRecordError::CursorPastEnd);
        }
        let end = (start + limit).min(vec.len());
        Ok(vec[start..end].to_vec())
    }

    fn verify_stream(
        &self,
        tenant_id: &TenantId,
        stream_id: StreamId,
    ) -> Result<(), DurableRecordError> {
        let key = Self::key(tenant_id, stream_id);
        let streams = self.streams.read();
        let vec = streams.get(&key).ok_or(DurableRecordError::StreamNotFound)?;
        verify_chain(vec).map_err(|sequence| DurableRecordError::ChainBroken { sequence })
    }
}
