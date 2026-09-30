mod tru_fixtures;

use tru_fixtures::{admissible, engine, input, reconciliation_record};
use reak_reconciliation::ReconciliationOutcome;
use reak_truth::{NonEstablishedReason, TruthConclusion, TruthDerivationInput, TruthError};

#[test]
fn admissibility_insufficient_blocks_establishment() {
    let tru = engine();
    let mut inp = input(reconciliation_record(
        "rec-a",
        "dsp-a",
        ReconciliationOutcome::ReconciledSuccess,
    ));
    inp.admissibility.all_required_admissible = false;
    let record = tru.derive_conclusion(inp).unwrap();
    assert_eq!(record.conclusion, TruthConclusion::NonEstablished);
}

#[test]
fn conflict_remains_non_established() {
    let tru = engine();
    let record = tru
        .derive_conclusion(input(reconciliation_record(
            "rec-c",
            "dsp-c",
            ReconciliationOutcome::ObservationConflict,
        )))
        .unwrap();
    assert_eq!(record.conclusion, TruthConclusion::NonEstablished);
}

#[test]
fn duplicate_truth_rejected() {
    let tru = engine();
    let inp = input(reconciliation_record(
        "rec-d",
        "dsp-d",
        ReconciliationOutcome::ReconciledFailure,
    ));
    tru.derive_conclusion(inp.clone()).unwrap();
    assert_eq!(tru.derive_conclusion(inp).unwrap_err(), TruthError::DuplicateTruth);
}

#[test]
fn preserve_non_established_never_upgrades() {
    let tru = engine();
    let inp = input(reconciliation_record(
        "rec-p",
        "dsp-p",
        ReconciliationOutcome::ReconciledSuccess,
    ));
    let record = tru
        .preserve_non_established(inp, NonEstablishedReason::ReconciliationUnknown)
        .unwrap();
    assert_eq!(record.conclusion, TruthConclusion::NonEstablished);
}
