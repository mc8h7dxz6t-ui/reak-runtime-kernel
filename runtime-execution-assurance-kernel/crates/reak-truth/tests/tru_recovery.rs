mod tru_fixtures;

use tru_fixtures::{engine, input, reconciliation_record};
use reak_reconciliation::ReconciliationOutcome;

#[test]
fn recover_after_derive() {
    let tru = engine();
    tru.derive_conclusion(input(reconciliation_record(
        "rec-r",
        "dsp-r",
        ReconciliationOutcome::ReconciledSuccess,
    )))
    .unwrap();
    assert_eq!(tru.recover_from_log().unwrap(), 1);
}

#[test]
fn recover_empty() {
    let tru = engine();
    assert_eq!(tru.recover_from_log().unwrap(), 0);
}
