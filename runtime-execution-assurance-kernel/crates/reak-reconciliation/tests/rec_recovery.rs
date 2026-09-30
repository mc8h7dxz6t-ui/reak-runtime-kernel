mod rec_fixtures;

use rec_fixtures::{base_input, dsp_outcome, engine, obs_record, released, ticket_issued};
use reak_dispatch::DispatchOutcome;
use reak_observation::ObservedClassification;

#[test]
fn recover_after_reconcile() {
    let rec = engine();
    let input = base_input(
        "dsp-rec",
        "cmt-1",
        vec![
            ticket_issued("dsp-rec", "cmt-1"),
            released("dsp-rec"),
            dsp_outcome("dsp-rec", DispatchOutcome::Success),
        ],
        vec![obs_record("dsp-rec", "e", ObservedClassification::ObservedSuccess, "o")],
    );
    rec.reconcile(input).unwrap();
    assert_eq!(rec.recover_from_log().unwrap(), 1);
    assert_eq!(rec.records_for_ticket("dsp-rec").len(), 1);
}

#[test]
fn recover_empty() {
    let rec = engine();
    assert_eq!(rec.recover_from_log().unwrap(), 0);
}
