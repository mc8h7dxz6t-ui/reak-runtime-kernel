use reak_types::{RecordHash, RecordSequence, StreamId, TenantId};
use serde::{Deserialize, Serialize};

/// Immutable envelope submitted to append.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordEnvelope {
    pub tenant_id: TenantId,
    pub stream_id: StreamId,
    pub schema_version: u32,
    pub record_kind: RecordKind,
    pub payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RecordKind {
    Data,
    Supersede,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupersedeLink {
    pub target_sequence: RecordSequence,
    pub reason_code: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppendAck {
    pub sequence: RecordSequence,
    pub record_hash: RecordHash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordCursor {
    pub sequence: RecordSequence,
}

/// Record as stored (immutable after append).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredRecord {
    pub sequence: RecordSequence,
    pub prev_hash: RecordHash,
    pub record_hash: RecordHash,
    pub envelope: RecordEnvelope,
    pub supersede: Option<SupersedeLink>,
}
