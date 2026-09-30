use proptest::prelude::*;
use reak_observation::state_machine;
use reak_observation::ObservedClassification;

proptest! {
    #[test]
    fn unknown_never_inferred_to_success(
        unknown in Just(ObservedClassification::ObservedUnknown),
        success in Just(ObservedClassification::ObservedSuccess),
    ) {
        prop_assert!(!state_machine::would_infer_truth(unknown, success));
    }

    #[test]
    fn all_classes_first_class(c in prop_oneof![
        Just(ObservedClassification::ObservedSuccess),
        Just(ObservedClassification::ObservedFailure),
        Just(ObservedClassification::ObservedUnknown),
        Just(ObservedClassification::ObservedTimeout),
        Just(ObservedClassification::ObservedCancelled),
        Just(ObservedClassification::ObservationUnavailable),
    ]) {
        prop_assert!(state_machine::classification_is_first_class(c));
    }
}
