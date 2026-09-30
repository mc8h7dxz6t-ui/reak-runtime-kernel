mod prg_fixtures;

use prg_fixtures::{authorization, engine, input, recovery, truth};
use reak_progression::{ProgressionError, ProgressionState};
use reak_recovery::RecoveryStrategy;
use reak_truth::TruthConclusion;

#[test]
fn binding_mismatch_rejected() {
    let prg = engine();
    let tr = truth("tru-a", "dsp-a", TruthConclusion::EstablishedSuccess);
    let rec = recovery("rcv-a", "tru-wrong", "dsp-a", RecoveryStrategy::NoAction, true);
    let err = prg
        .determine_next_step(input(tr, rec, authorization("rcv-a", "tru-wrong")))
        .unwrap_err();
    assert_eq!(err, ProgressionError::BindingMismatch);
}

#[test]
fn illegal_execution_flag_rejected() {
    let prg = engine();
    let tr = truth("tru-b", "dsp-b", TruthConclusion::EstablishedFailure);
    let mut rec = recovery("rcv-b", "tru-b", "dsp-b", RecoveryStrategy::Compensate, true);
    rec.authorizes_execution = true;
    let err = prg
        .determine_next_step(input(tr, rec, authorization("rcv-b", "tru-b")))
        .unwrap_err();
    assert_eq!(err, ProgressionError::IllegalExecutionFlag);
}

#[test]
fn duplicate_progression_rejected() {
    let prg = engine();
    let tr = truth("tru-c", "dsp-c", TruthConclusion::EstablishedFailure);
    let rec = recovery("rcv-c", "tru-c", "dsp-c", RecoveryStrategy::Compensate, true);
    let inp = input(tr, rec, authorization("rcv-c", "tru-c"));
    prg.determine_next_step(inp.clone()).unwrap();
    assert_eq!(
        prg.determine_next_step(inp).unwrap_err(),
        ProgressionError::DuplicateProgression
    );
}

#[test]
fn escalate_maps_to_human_approval() {
    let prg = engine();
    let tr = truth("tru-d", "dsp-d", TruthConclusion::NonEstablished);
    let rec = recovery("rcv-d", "tru-d", "dsp-d", RecoveryStrategy::Escalate, true);
    let record = prg
        .determine_next_step(input(tr, rec, authorization("rcv-d", "tru-d")))
        .unwrap();
    assert_eq!(record.state, ProgressionState::HumanApprovalRequired);
}
