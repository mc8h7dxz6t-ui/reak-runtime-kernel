use reak_recovery::{PolicySnapshotRef, RecoveryEngine, RecoveryInput};
use reak_truth::{TruthConclusion, TruthRecord, TruthReplayMetadata};
use reak_types::{RecordHash, RecordSequence, TenantId};

pub fn engine() -> RecoveryEngine {
    RecoveryEngine::new(TenantId::parse("tenant-rcv").unwrap())
}

pub fn policy() -> PolicySnapshotRef {
    PolicySnapshotRef {
        epoch_id: "epoch-1".into(),
        content_hash: RecordHash::from_bytes([3u8; 32]),
    }
}

pub fn truth(
    truth_id: &str,
    ticket: &str,
    conclusion: TruthConclusion,
    reason: Option<reak_truth::NonEstablishedReason>,
) -> TruthRecord {
    TruthRecord {
        truth_id: truth_id.into(),
        reconciliation_id: format!("rec-{}", truth_id),
        dispatch_ticket_id: ticket.into(),
        conclusion,
        non_established_reason: reason,
        input_digest_hex: format!("dig-{}", truth_id),
        replay: TruthReplayMetadata {
            durable_sequence: RecordSequence::new(1),
            prev_chain_hash: RecordHash::ZERO,
            record_chain_hash: RecordHash::ZERO,
        },
    }
}

pub fn input(tr: TruthRecord) -> RecoveryInput {
    RecoveryInput {
        truth: tr,
        policy: policy(),
    }
}
