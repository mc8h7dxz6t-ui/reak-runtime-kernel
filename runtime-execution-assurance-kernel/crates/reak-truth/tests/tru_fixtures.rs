use reak_commitment::CommitmentDigest;
use reak_reconciliation::{
    CommitmentReference, PolicyReference, ReconciliationOutcome, ReconciliationRecord,
    ReconciliationReplayMetadata, UesReference,
};
use reak_truth::{AdmissibilitySet, TruthDerivationInput, TruthEngine};
use reak_types::{RecordHash, RecordSequence, TenantId};

pub fn engine() -> TruthEngine {
    TruthEngine::new(TenantId::parse("tenant-tru").unwrap())
}

pub fn admissible() -> AdmissibilitySet {
    AdmissibilitySet {
        catalogue_handles: vec!["evd/handle-1".into()],
        all_required_admissible: true,
    }
}

pub fn reconciliation_record(
    rec_id: &str,
    ticket: &str,
    outcome: ReconciliationOutcome,
) -> ReconciliationRecord {
    ReconciliationRecord {
        reconciliation_id: rec_id.into(),
        commitment_ref: CommitmentReference {
            commitment_id: "cmt-1".into(),
            commitment_digest: CommitmentDigest([1u8; 32]),
        },
        dispatch_ticket_id: ticket.into(),
        policy_ref: PolicyReference {
            epoch_id: "epoch-1".into(),
            content_hash: RecordHash::from_bytes([2u8; 32]),
        },
        ues_ref: UesReference {
            reconciliation_stage_remaining: 10,
        },
        outcome,
        dispatch_outcome: None,
        observation_ids: vec![],
        observation_classes: vec![],
        input_digest_hex: format!("dig-{}", rec_id),
        replay: ReconciliationReplayMetadata {
            durable_sequence: RecordSequence::new(1),
            prev_chain_hash: RecordHash::ZERO,
            record_chain_hash: RecordHash::ZERO,
        },
    }
}

pub fn input(rec: ReconciliationRecord) -> TruthDerivationInput {
    TruthDerivationInput {
        reconciliation: rec,
        admissibility: admissible(),
    }
}
