use crate::model::{RecordEnvelope, StoredRecord, SupersedeLink};
use reak_types::RecordHash;
use sha2::{Digest, Sha256};

/// Canonical hash over record content (deterministic for same inputs).
pub fn hash_record(
    prev_hash: &RecordHash,
    sequence: u64,
    envelope: &RecordEnvelope,
    supersede: Option<&SupersedeLink>,
) -> RecordHash {
    let mut hasher = Sha256::new();
    hasher.update(prev_hash.as_bytes());
    hasher.update(sequence.to_le_bytes());
    hasher.update(envelope.schema_version.to_le_bytes());
    hasher.update([envelope.record_kind as u8]);
    hasher.update((envelope.payload.len() as u64).to_le_bytes());
    hasher.update(&envelope.payload);
    if let Some(link) = supersede {
        hasher.update(link.target_sequence.raw().to_le_bytes());
        hasher.update(link.reason_code.to_le_bytes());
    } else {
        hasher.update([0u8]);
    }
    let digest = hasher.finalize();
    RecordHash::from_bytes(digest.into())
}

pub fn verify_chain(records: &[StoredRecord]) -> Result<(), u64> {
    let mut expected_prev = RecordHash::ZERO;
    for rec in records {
        if rec.prev_hash != expected_prev {
            return Err(rec.sequence.raw());
        }
        let computed = hash_record(
            &rec.prev_hash,
            rec.sequence.raw(),
            &rec.envelope,
            rec.supersede.as_ref(),
        );
        if computed != rec.record_hash {
            return Err(rec.sequence.raw());
        }
        expected_prev = rec.record_hash;
    }
    Ok(())
}
