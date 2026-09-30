mod rec_fixtures;

use rec_fixtures::{
    base_input, dsp_outcome, engine, obs_record, released, ticket_issued,
};
use reak_dispatch::DispatchOutcome;
use reak_observation::ObservedClassification;
use reak_reconciliation::ReconciliationOutcome;

#[test]
fn reconciled_success_path() {
    let rec = engine();
    let input = base_input(
        "dsp-1",
        "cmt-1",
        vec![
            ticket_issued("dsp-1", "cmt-1"),
            released("dsp-1"),
            dsp_outcome("dsp-1", DispatchOutcome::Success),
        ],
        vec![obs_record("dsp-1", "e1", ObservedClassification::ObservedSuccess, "o1")],
    );
    let record = rec.reconcile(input).unwrap();
    assert_eq!(record.outcome, ReconciliationOutcome::ReconciledSuccess);
    rec.verify().unwrap();
}

#[test]
fn reconciled_unknown_never_becomes_success() {
    let rec = engine();
    let input = base_input(
        "dsp-2",
        "cmt-1",
        vec![
            ticket_issued("dsp-2", "cmt-1"),
            released("dsp-2"),
            dsp_outcome("dsp-2", DispatchOutcome::Success),
        ],
        vec![obs_record("dsp-2", "e1", ObservedClassification::ObservedUnknown, "o2")],
    );
    let record = rec.reconcile(input).unwrap();
    assert_eq!(record.outcome, ReconciliationOutcome::ReconciledUnknown);
}
