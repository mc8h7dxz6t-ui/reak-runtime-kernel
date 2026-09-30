mod prg_fixtures;

use prg_fixtures::{authorization, engine, input, recovery, truth};
use reak_progression::ProgressionState;
use reak_recovery::RecoveryStrategy;
use reak_truth::TruthConclusion;

#[test]
fn success_terminal() {
    let prg = engine();
    let tr = truth("tru-1", "dsp-1", TruthConclusion::EstablishedSuccess);
    let rec = recovery("rcv-1", "tru-1", "dsp-1", RecoveryStrategy::NoAction, true);
    let record = prg
        .determine_next_step(input(tr, rec, authorization("rcv-1", "tru-1")))
        .unwrap();
    assert_eq!(record.state, ProgressionState::Terminal);
    prg.verify().unwrap();
}

#[test]
fn retry_unauthorized_awaits() {
    let prg = engine();
    let tr = truth("tru-2", "dsp-2", TruthConclusion::NonEstablished);
    let rec = recovery("rcv-2", "tru-2", "dsp-2", RecoveryStrategy::Retry, false);
    let record = prg
        .determine_next_step(input(tr, rec, authorization("rcv-2", "tru-2")))
        .unwrap();
    assert_eq!(record.state, ProgressionState::Await);
}

#[test]
fn retry_authorized_permitted() {
    let prg = engine();
    let tr = truth("tru-3", "dsp-3", TruthConclusion::NonEstablished);
    let rec = recovery("rcv-3", "tru-3", "dsp-3", RecoveryStrategy::Retry, true);
    let record = prg
        .determine_next_step(input(tr, rec, authorization("rcv-3", "tru-3")))
        .unwrap();
    assert_eq!(record.state, ProgressionState::RetryPermitted);
}
