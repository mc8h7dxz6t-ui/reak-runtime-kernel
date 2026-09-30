use crate::error::ReconciliationError;
use crate::model::{
    ReconciliationInput, ReconciliationOutcome, ReconciliationRecord, ReconciliationReplayMetadata,
};
use reak_dispatch::{DispatchHistoryEntry, DispatchOutcome};
use reak_observation::{ObservationRecord, ObservedClassification};
use reak_types::{RecordHash, RecordSequence};
use sha2::{Digest, Sha256};

pub fn input_digest_hex(input: &ReconciliationInput) -> String {
    let payload = serde_json::to_vec(input).unwrap_or_default();
    hex::encode(Sha256::digest(&payload))
}

fn field_ok(s: &str) -> bool {
    !s.is_empty() && s.len() <= reak_types::MAX_ID_UTF8_BYTES
}

pub fn validate_input(input: &ReconciliationInput) -> Result<(), ReconciliationError> {
    if !field_ok(&input.commitment_ref.commitment_id)
        || !field_ok(&input.dispatch_ticket_id)
        || !field_ok(&input.policy_ref.epoch_id)
    {
        return Err(ReconciliationError::InvalidInput);
    }
    if input.policy_ref.content_hash == RecordHash::ZERO {
        return Err(ReconciliationError::InvalidInput);
    }
    Ok(())
}

struct DispatchView {
    commitment_id: String,
    released: bool,
    terminal: Option<DispatchOutcome>,
}

fn project_dispatch(
    history: &[DispatchHistoryEntry],
    ticket_id: &str,
) -> Result<Option<DispatchView>, ReconciliationError> {
    let mut issued = 0u32;
    let mut view: Option<DispatchView> = None;

    for entry in history {
        match entry {
            DispatchHistoryEntry::TicketIssued(t) if t.ticket_id == ticket_id => {
                issued += 1;
                if issued > 1 {
                    return Err(ReconciliationError::DuplicateDispatchId);
                }
                view = Some(DispatchView {
                    commitment_id: t.commitment_id.clone(),
                    released: false,
                    terminal: None,
                });
            }
            DispatchHistoryEntry::Released(r) if r.ticket_id == ticket_id => {
                if let Some(v) = view.as_mut() {
                    v.released = true;
                }
            }
            DispatchHistoryEntry::Outcome(o) if o.ticket_id == ticket_id => {
                if let Some(v) = view.as_mut() {
                    v.terminal = Some(o.outcome);
                }
            }
            _ => {}
        }
    }
    Ok(view)
}

fn observations_for_ticket(
    history: &[ObservationRecord],
    ticket_id: &str,
) -> Vec<ObservationRecord> {
    history
        .iter()
        .filter(|o| o.reference.dispatch_ticket_id == ticket_id)
        .cloned()
        .collect()
}

fn orphan_observations(
    history: &[ObservationRecord],
    ticket_id: &str,
) -> Vec<ObservationRecord> {
    history
        .iter()
        .filter(|o| o.reference.dispatch_ticket_id != ticket_id)
        .cloned()
        .collect()
}

fn duplicate_source_events(obs: &[ObservationRecord]) -> bool {
    let mut seen = std::collections::HashSet::new();
    for o in obs {
        let key = (
            o.reference.observation_source.clone(),
            o.reference.source_event_id.clone(),
        );
        if !seen.insert(key) {
            return true;
        }
    }
    false
}

fn unique_classes(obs: &[ObservationRecord]) -> Vec<ObservedClassification> {
    let mut v = obs.iter().map(|o| o.classification).collect::<Vec<_>>();
    v.sort_by_key(|c| *c as u8);
    v.dedup();
    v
}

fn map_dispatch_observed(
    dispatch: DispatchOutcome,
    observed: ObservedClassification,
) -> ReconciliationOutcome {
    if observed == ObservedClassification::ObservedUnknown
        || observed == ObservedClassification::ObservationUnavailable
        || dispatch == DispatchOutcome::Unknown
    {
        return ReconciliationOutcome::ReconciledUnknown;
    }
    match (dispatch, observed) {
        (DispatchOutcome::Success, ObservedClassification::ObservedSuccess) => {
            ReconciliationOutcome::ReconciledSuccess
        }
        (DispatchOutcome::Failed, ObservedClassification::ObservedFailure) => {
            ReconciliationOutcome::ReconciledFailure
        }
        (DispatchOutcome::Cancelled, ObservedClassification::ObservedCancelled) => {
            ReconciliationOutcome::ReconciledFailure
        }
        (DispatchOutcome::Timeout, ObservedClassification::ObservedTimeout) => {
            ReconciliationOutcome::ReconciledUnknown
        }
        _ => ReconciliationOutcome::ObservationConflict,
    }
}

/// Deterministic reconciliation outcome (no clock, no randomness).
pub fn compute_outcome(input: &ReconciliationInput) -> Result<ReconciliationOutcome, ReconciliationError> {
    validate_input(input)?;

    if input.commitment_ref.commitment_id.is_empty() {
        return Err(ReconciliationError::MissingCommitment);
    }

    let dispatch = project_dispatch(&input.dispatch_history, &input.dispatch_ticket_id)?;
    let dispatch = dispatch.ok_or(ReconciliationError::MissingDispatch)?;

    if dispatch.commitment_id != input.commitment_ref.commitment_id {
        return Err(ReconciliationError::ForgedCommitmentReference);
    }

    if !orphan_observations(&input.observation_history, &input.dispatch_ticket_id).is_empty() {
        return Ok(ReconciliationOutcome::UnexpectedObservation);
    }

    let obs = observations_for_ticket(&input.observation_history, &input.dispatch_ticket_id);

    if duplicate_source_events(&obs) {
        return Ok(ReconciliationOutcome::DuplicateObservation);
    }

    if obs.is_empty() {
        if dispatch.released && dispatch.terminal.is_none() {
            return Ok(ReconciliationOutcome::DispatchNotObserved);
        }
        if dispatch.terminal.is_none() {
            return Ok(ReconciliationOutcome::EvidenceInsufficient);
        }
        return Ok(ReconciliationOutcome::DispatchNotObserved);
    }

    if !dispatch.released {
        return Ok(ReconciliationOutcome::UnexpectedObservation);
    }

    let unique = unique_classes(&obs);
    if unique.len() > 1 {
        return Ok(ReconciliationOutcome::ObservationConflict);
    }

    if obs.len() > 1 {
        return Ok(ReconciliationOutcome::MultipleValidObservations);
    }

    let terminal = match dispatch.terminal {
        Some(t) => t,
        None => return Ok(ReconciliationOutcome::EvidenceInsufficient),
    };
    let observed = unique[0];

    Ok(map_dispatch_observed(terminal, observed))
}

pub fn build_record(
    input: &ReconciliationInput,
    outcome: ReconciliationOutcome,
    replay: ReconciliationReplayMetadata,
    reconciliation_id: String,
) -> ReconciliationRecord {
    let obs = observations_for_ticket(&input.observation_history, &input.dispatch_ticket_id);
    let observation_ids = obs.iter().map(|o| o.observation_id.clone()).collect();
    let observation_classes = obs
        .iter()
        .map(|o| (o.observation_id.clone(), o.classification))
        .collect();
    let dispatch_outcome = project_dispatch(&input.dispatch_history, &input.dispatch_ticket_id)
        .ok()
        .flatten()
        .and_then(|v| v.terminal);

    ReconciliationRecord {
        reconciliation_id,
        commitment_ref: input.commitment_ref.clone(),
        dispatch_ticket_id: input.dispatch_ticket_id.clone(),
        policy_ref: input.policy_ref.clone(),
        ues_ref: input.ues_ref.clone(),
        outcome,
        dispatch_outcome,
        observation_ids,
        observation_classes,
        input_digest_hex: input_digest_hex(input),
        replay,
    }
}
