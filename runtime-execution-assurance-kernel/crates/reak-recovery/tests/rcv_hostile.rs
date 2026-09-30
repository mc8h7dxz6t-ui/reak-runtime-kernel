mod rcv_fixtures;

use rcv_fixtures::{engine, input, truth};
use reak_recovery::{RecoveryError, RecoveryStrategy};
use reak_truth::{NonEstablishedReason, TruthConclusion};

#[test]
fn unknown_truth_awaits_not_retry_as_success() {
    let rcv = engine();
    let record = rcv
        .propose_recovery(input(truth(
            "tru-u",
            "dsp-u",
            TruthConclusion::NonEstablished,
            Some(NonEstablishedReason::ReconciliationUnknown),
        )))
        .unwrap();
    assert_eq!(record.strategy, RecoveryStrategy::Await);
}

#[test]
fn conflict_escalates() {
    let rcv = engine();
    let record = rcv
        .propose_recovery(input(truth(
            "tru-c",
            "dsp-c",
            TruthConclusion::NonEstablished,
            Some(NonEstablishedReason::ObservationConflict),
        )))
        .unwrap();
    assert_eq!(record.strategy, RecoveryStrategy::Escalate);
}

#[test]
fn duplicate_truth_recovery_rejected() {
    let rcv = engine();
    let inp = input(truth(
        "tru-d",
        "dsp-d",
        TruthConclusion::NonEstablished,
        Some(NonEstablishedReason::EvidenceInsufficient),
    ));
    rcv.propose_recovery(inp.clone()).unwrap();
    assert_eq!(
        rcv.propose_recovery(inp).unwrap_err(),
        RecoveryError::DuplicateRecovery
    );
}

#[test]
fn evidence_insufficient_retries() {
    let rcv = engine();
    let record = rcv
        .propose_recovery(input(truth(
            "tru-r",
            "dsp-r",
            TruthConclusion::NonEstablished,
            Some(NonEstablishedReason::EvidenceInsufficient),
        )))
        .unwrap();
    assert_eq!(record.strategy, RecoveryStrategy::Retry);
}
