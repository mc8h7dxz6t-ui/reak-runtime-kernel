use proptest::prelude::*;
use rcv_fixtures::{input, truth};
use reak_recovery::select_strategy;

mod rcv_fixtures;

proptest! {
    #[test]
    fn strategy_deterministic(i in 0u8..30) {
        let tr = truth(
            &format!("tru-p-{}", i),
            "dsp-p",
            reak_truth::TruthConclusion::NonEstablished,
            Some(reak_truth::NonEstablishedReason::ReconciliationUnknown),
        );
        let a = select_strategy(&tr);
        let b = select_strategy(&tr);
        prop_assert_eq!(a, b);
    }
}
