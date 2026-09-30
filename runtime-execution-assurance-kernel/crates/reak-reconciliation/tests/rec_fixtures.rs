use reak_commitment::CommitmentDigest;
use reak_dispatch::{
    DispatchAttemptRecord, DispatchHistoryEntry, DispatchOutcome, DispatchOutcomeRecord,
    DispatchReplayMetadata, DispatchState, DispatchTicket,
};
use reak_observation::{
    ObservationInput, ObservationMetadata, ObservationReference, ObservationReplayMetadata,
    ObservedClassification,
};
use reak_reconciliation::{
    CommitmentReference, PolicyReference, ReconciliationEngine, ReconciliationInput, UesReference,
};
use reak_types::{RecordHash, RecordSequence, TenantId};

pub fn tenant() -> TenantId {
    TenantId::parse("tenant-rec").unwrap()
}

pub fn engine() -> ReconciliationEngine {
    ReconciliationEngine::new(tenant())
}

pub fn policy_ref() -> PolicyReference {
    PolicyReference {
        epoch_id: "epoch-1".into(),
        content_hash: RecordHash::from_bytes([9u8; 32]),
    }
}

pub fn commitment_ref(id: &str) -> CommitmentReference {
    CommitmentReference {
        commitment_id: id.into(),
        commitment_digest: CommitmentDigest([7u8; 32]),
    }
}

pub fn ticket_issued(ticket_id: &str, commitment_id: &str) -> DispatchHistoryEntry {
    DispatchHistoryEntry::TicketIssued(DispatchTicket {
        ticket_id: ticket_id.into(),
        commitment_id: commitment_id.into(),
        attempt_number: 1,
        operation_lineage: "line".into(),
        operation_generation: 1,
        state: DispatchState::TicketIssued,
        issued_at_unix_ms: 1,
    })
}

pub fn released(ticket_id: &str) -> DispatchHistoryEntry {
    DispatchHistoryEntry::Released(DispatchAttemptRecord {
        ticket_id: ticket_id.into(),
        attempt_number: 1,
        released_at_unix_ms: 2,
        replay: DispatchReplayMetadata {
            durable_sequence: RecordSequence::new(1),
            prev_chain_hash: RecordHash::ZERO,
            record_chain_hash: RecordHash::ZERO,
        },
    })
}

pub fn dsp_outcome(ticket_id: &str, outcome: DispatchOutcome) -> DispatchHistoryEntry {
    DispatchHistoryEntry::Outcome(DispatchOutcomeRecord {
        ticket_id: ticket_id.into(),
        outcome,
        classified_at_unix_ms: 3,
        replay: DispatchReplayMetadata {
            durable_sequence: RecordSequence::new(2),
            prev_chain_hash: RecordHash::ZERO,
            record_chain_hash: RecordHash::ZERO,
        },
        recovery_metadata: None,
        cancellation_reason: None,
    })
}

pub fn obs_record(
    ticket_id: &str,
    event_id: &str,
    class: ObservedClassification,
    obs_id: &str,
) -> reak_observation::ObservationRecord {
    reak_observation::ObservationRecord {
        observation_id: obs_id.into(),
        reference: ObservationReference {
            dispatch_ticket_id: ticket_id.into(),
            provider_id: "provider.sim".into(),
            provider_operation_id: "op".into(),
            correlation_ids: vec![],
            observed_at_unix_ms: 10,
            observation_source: "sim".into(),
            source_event_id: event_id.into(),
        },
        classification: class,
        raw_payload: vec![1],
        metadata: ObservationMetadata { labels: vec![] },
        content_digest_hex: "abc".into(),
        appended_at_unix_ms: 11,
        replay: ObservationReplayMetadata {
            durable_sequence: RecordSequence::new(1),
            prev_chain_hash: RecordHash::ZERO,
            record_chain_hash: RecordHash::ZERO,
        },
    }
}

pub fn base_input(
    ticket: &str,
    commitment_id: &str,
    dispatch_history: Vec<DispatchHistoryEntry>,
    observation_history: Vec<reak_observation::ObservationRecord>,
) -> ReconciliationInput {
    ReconciliationInput {
        commitment_ref: commitment_ref(commitment_id),
        dispatch_ticket_id: ticket.into(),
        dispatch_history,
        observation_history,
        policy_ref: policy_ref(),
        ues_ref: UesReference {
            reconciliation_stage_remaining: 50,
        },
    }
}
