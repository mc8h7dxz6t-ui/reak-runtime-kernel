use crate::model::ObservedClassification;

/// Observation records have no runtime inference transitions — only append of caller-supplied class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationLifecycle {
    Validated,
    Durable,
}

pub fn classification_is_first_class(c: ObservedClassification) -> bool {
    matches!(
        c,
        ObservedClassification::ObservedUnknown
            | ObservedClassification::ObservationUnavailable
            | ObservedClassification::ObservedSuccess
            | ObservedClassification::ObservedFailure
            | ObservedClassification::ObservedTimeout
            | ObservedClassification::ObservedCancelled
    )
}

/// Engine never maps Unknown → Success/Failure.
pub fn would_infer_truth(_from: ObservedClassification, _to: ObservedClassification) -> bool {
    false
}
