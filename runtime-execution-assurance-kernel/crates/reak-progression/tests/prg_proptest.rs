use proptest::prelude::*;
use reak_progression::classify_next_state;

mod prg_fixtures;

proptest! {
    #[test]
    fn classification_is_deterministic(seed in 0u8..40) {
        let inp = sample_input(seed);
        let a = classify_next_state(&inp).unwrap();
        let b = classify_next_state(&inp).unwrap();
        prop_assert_eq!(a, b);
    }
}

fn sample_input(seed: u8) -> reak_progression::ProgressionInput {
    use prg_fixtures::{authorization, input, recovery, truth};
    use reak_recovery::RecoveryStrategy;
    use reak_truth::TruthConclusion;
    let tid = format!("tru-p-{}", seed);
    let rid = format!("rcv-p-{}", seed);
    input(
        truth(&tid, "dsp-p", TruthConclusion::NonEstablished),
        recovery(&rid, &tid, "dsp-p", RecoveryStrategy::Await, true),
        authorization(&rid, &tid),
    )
}
