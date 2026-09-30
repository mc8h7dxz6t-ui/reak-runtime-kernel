use reak_recovery::{
    PolicySnapshotRef, RecoveryIntentAuthorization, RecoveryRecord, RecoveryReplayMetadata,
    RecoveryStrategy,
};
use reak_progression::{ProgressionEngine, ProgressionInput};
use reak_truth::{TruthConclusion, TruthRecord, TruthReplayMetadata};
use reak_types::{RecordHash, RecordSequence, TenantId};

pub fn engine() -> ProgressionEngine {
    ProgressionEngine::new(TenantId::parse("tenant-prg").unwrap())
}

pub fn policy() -> PolicySnapshotRef {
    PolicySnapshotRef {
        epoch_id: "epoch-1".into(),
        content_hash: RecordHash::from_bytes([4u8; 32]),
    }
}

pub fn truth(id: &str, ticket: &str, conclusion: TruthConclusion) -> TruthRecord {
    TruthRecord {
        truth_id: id.into(),
        reconciliation_id: format!("rec-{}", id),
        dispatch_ticket_id: ticket.into(),
        conclusion,
        non_established_reason: None,
        input_digest_hex: format!("td-{}", id),
        replay: TruthReplayMetadata {
            durable_sequence: RecordSequence::new(1),
            prev_chain_hash: RecordHash::ZERO,
            record_chain_hash: RecordHash::ZERO,
        },
    }
}

pub fn recovery(
    id: &str,
    truth_id: &str,
    ticket: &str,
    strategy: RecoveryStrategy,
    authorized: bool,
) -> RecoveryRecord {
    RecoveryRecord {
        recovery_id: id.into(),
        truth_id: truth_id.into(),
        dispatch_ticket_id: ticket.into(),
        strategy,
        authorizes_execution: false,
        intent_authorized: authorized,
        input_digest_hex: format!("rd-{}", id),
        replay: RecoveryReplayMetadata {
            durable_sequence: RecordSequence::new(1),
            prev_chain_hash: RecordHash::ZERO,
            record_chain_hash: RecordHash::ZERO,
        },
    }
}

pub fn authorization(recovery_id: &str, truth_id: &str) -> RecoveryIntentAuthorization {
    RecoveryIntentAuthorization {
        recovery_id: recovery_id.into(),
        truth_id: truth_id.into(),
        authorizes_execution: false,
        replay: RecoveryReplayMetadata {
            durable_sequence: RecordSequence::new(2),
            prev_chain_hash: RecordHash::ZERO,
            record_chain_hash: RecordHash::ZERO,
        },
    }
}

pub fn input(
    tr: TruthRecord,
    rec: RecoveryRecord,
    auth: RecoveryIntentAuthorization,
) -> ProgressionInput {
    ProgressionInput {
        truth: tr,
        recovery: rec,
        authorization: auth,
        policy: policy(),
    }
}
