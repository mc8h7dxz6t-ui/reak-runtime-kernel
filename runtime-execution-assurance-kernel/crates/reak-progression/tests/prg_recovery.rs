mod prg_fixtures;

use prg_fixtures::{authorization, engine, input, recovery, truth};
use reak_recovery::RecoveryStrategy;
use reak_truth::TruthConclusion;

#[test]
fn recover_after_determine() {
    let prg = engine();
    let tr = truth("tru-r", "dsp-r", TruthConclusion::EstablishedSuccess);
    let rec = recovery("rcv-r", "tru-r", "dsp-r", RecoveryStrategy::NoAction, true);
    prg.determine_next_step(input(tr, rec, authorization("rcv-r", "tru-r")))
        .unwrap();
    assert_eq!(prg.recover_from_log().unwrap(), 1);
}
