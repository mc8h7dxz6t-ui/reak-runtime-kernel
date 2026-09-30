use reak_observation::state_machine;
use reak_observation::ObservedClassification;

#[test]
fn no_truth_inference_edges() {
    for from in [
        ObservedClassification::ObservedUnknown,
        ObservedClassification::ObservedFailure,
        ObservedClassification::ObservationUnavailable,
    ] {
        for to in [
            ObservedClassification::ObservedSuccess,
            ObservedClassification::ObservedFailure,
        ] {
            assert!(!state_machine::would_infer_truth(from, to));
        }
    }
}
