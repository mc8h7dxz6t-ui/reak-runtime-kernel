mod rcv_fixtures;

use rcv_fixtures::{engine, input, truth};
use reak_recovery::RecoveryStrategy;
use reak_truth::TruthConclusion;

#[test]
fn established_success_no_action() {
    let rcv = engine();
    let record = rcv
        .propose_recovery(input(truth(
            "tru-1",
            "dsp-1",
            TruthConclusion::EstablishedSuccess,
            None,
        )))
        .unwrap();
    assert_eq!(record.strategy, RecoveryStrategy::NoAction);
    assert!(!record.authorizes_execution);
    rcv.verify().unwrap();
}

#[test]
fn authorize_intent_never_authorizes_execution() {
    let rcv = engine();
    let rec = rcv
        .propose_recovery(input(truth(
            "tru-2",
            "dsp-2",
            TruthConclusion::EstablishedFailure,
            None,
        )))
        .unwrap();
    let auth = rcv.authorize_recovery_intent(&rec.recovery_id).unwrap();
    assert!(!auth.authorizes_execution);
}
