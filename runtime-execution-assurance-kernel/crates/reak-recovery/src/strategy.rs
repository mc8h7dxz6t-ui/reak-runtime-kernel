use crate::model::RecoveryStrategy;
use reak_truth::{NonEstablishedReason, TruthConclusion, TruthRecord};

/// Deterministic strategy from immutable truth only (no dispatch, no new truth).
pub fn select_strategy(truth: &TruthRecord) -> RecoveryStrategy {
    match truth.conclusion {
        TruthConclusion::EstablishedSuccess => RecoveryStrategy::NoAction,
        TruthConclusion::EstablishedFailure => RecoveryStrategy::Compensate,
        TruthConclusion::NonEstablished => match truth.non_established_reason {
            Some(NonEstablishedReason::EvidenceInsufficient) => RecoveryStrategy::Retry,
            Some(NonEstablishedReason::DispatchNotObserved) => RecoveryStrategy::Retry,
            Some(NonEstablishedReason::ReconciliationUnknown) => RecoveryStrategy::Await,
            Some(NonEstablishedReason::ObservationConflict) => RecoveryStrategy::Escalate,
            Some(NonEstablishedReason::UnexpectedObservation) => RecoveryStrategy::Escalate,
            Some(NonEstablishedReason::DuplicateObservation) => RecoveryStrategy::Abort,
            Some(NonEstablishedReason::MultipleValidObservations) => RecoveryStrategy::Escalate,
            Some(NonEstablishedReason::AdmissibilityInsufficient) => RecoveryStrategy::Await,
            Some(NonEstablishedReason::ReconciliationNotEstablishing) => RecoveryStrategy::Await,
            None => RecoveryStrategy::Await,
        },
    }
}
