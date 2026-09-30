mod tru_fixtures;

use tru_fixtures::{engine, input, reconciliation_record};
use reak_reconciliation::ReconciliationOutcome;
use reak_truth::TruthConclusion;

#[test]
fn established_success_from_reconciled_success() {
    let tru = engine();
    let record = tru
        .derive_conclusion(input(reconciliation_record(
            "rec-1",
            "dsp-1",
            ReconciliationOutcome::ReconciledSuccess,
        )))
        .unwrap();
    assert_eq!(record.conclusion, TruthConclusion::EstablishedSuccess);
    assert!(record.non_established_reason.is_none());
    tru.verify().unwrap();
}

#[test]
fn unknown_stays_non_established() {
    let tru = engine();
    let record = tru
        .derive_conclusion(input(reconciliation_record(
            "rec-2",
            "dsp-2",
            ReconciliationOutcome::ReconciledUnknown,
        )))
        .unwrap();
    assert_eq!(record.conclusion, TruthConclusion::NonEstablished);
}
