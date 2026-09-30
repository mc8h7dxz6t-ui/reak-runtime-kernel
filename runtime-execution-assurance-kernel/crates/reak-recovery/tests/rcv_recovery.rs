mod rcv_fixtures;

use rcv_fixtures::{engine, input, truth};
use reak_truth::TruthConclusion;

#[test]
fn recover_rebuilds_indexes() {
    let rcv = engine();
    let rec = rcv
        .propose_recovery(input(truth("tru-rec", "dsp-rec", TruthConclusion::EstablishedFailure, None)))
        .unwrap();
    rcv.authorize_recovery_intent(&rec.recovery_id).unwrap();
    assert_eq!(rcv.recover_from_log().unwrap(), 2);
}
