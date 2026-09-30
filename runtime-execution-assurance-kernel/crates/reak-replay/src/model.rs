use reak_durable_record::StoredRecord;
use reak_types::RecordSequence;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayReport {
    pub records_verified: usize,
    pub last_sequence: RecordSequence,
    pub chain_valid: bool,
    pub snapshot: Vec<StoredRecord>,
}
