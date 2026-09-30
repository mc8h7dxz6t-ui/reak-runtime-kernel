mod prg_fixtures;

use prg_fixtures::{authorization, engine, input, recovery, truth};
use reak_recovery::RecoveryStrategy;
use reak_truth::TruthConclusion;

#[test]
fn classify_one_thousand_under_budget() {
    let prg = engine();
    let tr = truth("tru-perf", "dsp-perf", TruthConclusion::EstablishedSuccess);
    let rec = recovery("rcv-perf", "tru-perf", "dsp-perf", RecoveryStrategy::NoAction, true);
    let inp = input(tr, rec, authorization("rcv-perf", "tru-perf"));
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        prg.classify_only(&inp).unwrap();
    }
    assert!(start.elapsed().as_millis() < 500);
}
