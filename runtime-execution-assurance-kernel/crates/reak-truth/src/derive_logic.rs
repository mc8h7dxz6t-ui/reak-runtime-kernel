use crate::error::TruthError;
use crate::model::{
    AdmissibilitySet, NonEstablishedReason, TruthConclusion, TruthDerivationInput,
    TruthRecord, TruthReplayMetadata,
};
use reak_reconciliation::ReconciliationOutcome;
use sha2::{Digest, Sha256};

pub fn input_digest_hex(input: &TruthDerivationInput) -> String {
    let payload = serde_json::to_vec(input).unwrap_or_default();
    hex::encode(Sha256::digest(&payload))
}

pub fn validate_input(input: &TruthDerivationInput) -> Result<(), TruthError> {
    if input.reconciliation.reconciliation_id.is_empty()
        || input.reconciliation.dispatch_ticket_id.is_empty()
    {
        return Err(TruthError::InvalidInput);
    }
    Ok(())
}

fn reason_from_reconciliation(outcome: ReconciliationOutcome) -> NonEstablishedReason {
    match outcome {
        ReconciliationOutcome::ReconciledUnknown => NonEstablishedReason::ReconciliationUnknown,
        ReconciliationOutcome::ObservationConflict => NonEstablishedReason::ObservationConflict,
        ReconciliationOutcome::EvidenceInsufficient => NonEstablishedReason::EvidenceInsufficient,
        ReconciliationOutcome::DispatchNotObserved => NonEstablishedReason::DispatchNotObserved,
        ReconciliationOutcome::UnexpectedObservation => NonEstablishedReason::UnexpectedObservation,
        ReconciliationOutcome::DuplicateObservation => NonEstablishedReason::DuplicateObservation,
        ReconciliationOutcome::MultipleValidObservations => {
            NonEstablishedReason::MultipleValidObservations
        }
        ReconciliationOutcome::ReconciledSuccess | ReconciliationOutcome::ReconciledFailure => {
            NonEstablishedReason::ReconciliationNotEstablishing
        }
    }
}

/// IF-TRU-01: map reconciliation + admissibility to a truth conclusion (deterministic).
pub fn derive_conclusion(input: &TruthDerivationInput) -> Result<(TruthConclusion, Option<NonEstablishedReason>), TruthError> {
    validate_input(input)?;
    if !input.admissibility.all_required_admissible {
        return Ok((
            TruthConclusion::NonEstablished,
            Some(NonEstablishedReason::AdmissibilityInsufficient),
        ));
    }

    match input.reconciliation.outcome {
        ReconciliationOutcome::ReconciledSuccess => Ok((TruthConclusion::EstablishedSuccess, None)),
        ReconciliationOutcome::ReconciledFailure => Ok((TruthConclusion::EstablishedFailure, None)),
        other => Ok((
            TruthConclusion::NonEstablished,
            Some(reason_from_reconciliation(other)),
        )),
    }
}

/// IF-TRU-01: explicit non-established preservation (never upgrades to established).
pub fn preserve_non_established(
    input: &TruthDerivationInput,
    reason: NonEstablishedReason,
) -> Result<(TruthConclusion, Option<NonEstablishedReason>), TruthError> {
    validate_input(input)?;
    let (_, auto) = derive_conclusion(input)?;
    if matches!(
        auto,
        Some(NonEstablishedReason::AdmissibilityInsufficient)
            | Some(NonEstablishedReason::ReconciliationUnknown)
            | Some(NonEstablishedReason::ObservationConflict)
    ) {
        return Ok((TruthConclusion::NonEstablished, auto));
    }
    Ok((TruthConclusion::NonEstablished, Some(reason)))
}

pub fn build_record(
    input: &TruthDerivationInput,
    conclusion: TruthConclusion,
    reason: Option<NonEstablishedReason>,
    replay: TruthReplayMetadata,
    truth_id: String,
) -> TruthRecord {
    TruthRecord {
        truth_id,
        reconciliation_id: input.reconciliation.reconciliation_id.clone(),
        dispatch_ticket_id: input.reconciliation.dispatch_ticket_id.clone(),
        conclusion,
        non_established_reason: reason,
        input_digest_hex: input_digest_hex(input),
        replay,
    }
}
