use proptest::prelude::*;
use reak_reconciliation::compute_outcome;
use reak_reconciliation::{ReconciliationInput, ReconciliationOutcome};

proptest! {
    #[test]
    fn deterministic_same_input_same_outcome(a in 0u8..5) {
        let input = sample_input(a);
        let o1 = compute_outcome(&input).unwrap();
        let o2 = compute_outcome(&input).unwrap();
        prop_assert_eq!(o1, o2);
    }
}

fn sample_input(seed: u8) -> ReconciliationInput {
    use rec_fixtures::{
        base_input, dsp_outcome, obs_record, released, ticket_issued,
    };
    let ticket = format!("dsp-p-{}", seed);
    base_input(
        &ticket,
        "cmt-1",
        vec![
            ticket_issued(&ticket, "cmt-1"),
            released(&ticket),
            dsp_outcome(&ticket, reak_dispatch::DispatchOutcome::Success),
        ],
        vec![obs_record(
            &ticket,
            "e",
            reak_observation::ObservedClassification::ObservedUnknown,
            "o",
        )],
    )
}

mod rec_fixtures;
