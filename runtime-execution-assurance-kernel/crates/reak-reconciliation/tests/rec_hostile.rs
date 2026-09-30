mod rec_fixtures;

use rec_fixtures::{
    base_input, commitment_ref, dsp_outcome, engine, obs_record, released, ticket_issued,
};
use reak_dispatch::DispatchOutcome;
use reak_observation::ObservedClassification;
use reak_reconciliation::{ReconciliationError, ReconciliationOutcome};

#[test]
fn conflicting_observations() {
    let rec = engine();
    let input = base_input(
        "dsp-c",
        "cmt-1",
        vec![
            ticket_issued("dsp-c", "cmt-1"),
            released("dsp-c"),
            dsp_outcome("dsp-c", DispatchOutcome::Success),
        ],
        vec![
            obs_record("dsp-c", "a", ObservedClassification::ObservedSuccess, "o-a"),
            obs_record("dsp-c", "b", ObservedClassification::ObservedFailure, "o-b"),
        ],
    );
    let record = rec.reconcile(input).unwrap();
    assert_eq!(record.outcome, ReconciliationOutcome::ObservationConflict);
    assert_eq!(record.observation_ids.len(), 2);
}

#[test]
fn duplicate_observation_source_events() {
    let rec = engine();
    let o1 = obs_record("dsp-d", "dup", ObservedClassification::ObservedSuccess, "o1");
    let o2 = obs_record("dsp-d", "dup", ObservedClassification::ObservedSuccess, "o2");
    let input = base_input(
        "dsp-d",
        "cmt-1",
        vec![
            ticket_issued("dsp-d", "cmt-1"),
            released("dsp-d"),
            dsp_outcome("dsp-d", DispatchOutcome::Success),
        ],
        vec![o1, o2],
    );
    let record = rec.reconcile(input).unwrap();
    assert_eq!(record.outcome, ReconciliationOutcome::DuplicateObservation);
}

#[test]
fn missing_dispatch() {
    let rec = engine();
    let input = base_input("dsp-miss", "cmt-1", vec![], vec![]);
    assert_eq!(rec.reconcile(input).unwrap_err(), ReconciliationError::MissingDispatch);
}

#[test]
fn forged_commitment_reference() {
    let rec = engine();
    let input = base_input(
        "dsp-f",
        "cmt-wrong",
        vec![ticket_issued("dsp-f", "cmt-1"), released("dsp-f")],
        vec![],
    );
    assert_eq!(
        rec.reconcile(input).unwrap_err(),
        ReconciliationError::ForgedCommitmentReference
    );
}

#[test]
fn orphan_observation_in_history() {
    let rec = engine();
    let input = base_input(
        "dsp-o",
        "cmt-1",
        vec![ticket_issued("dsp-o", "cmt-1"), released("dsp-o")],
        vec![obs_record("other-ticket", "e", ObservedClassification::ObservedSuccess, "ox")],
    );
    let record = rec.reconcile(input).unwrap();
    assert_eq!(record.outcome, ReconciliationOutcome::UnexpectedObservation);
}

#[test]
fn dispatch_not_observed() {
    let rec = engine();
    let input = base_input(
        "dsp-no",
        "cmt-1",
        vec![ticket_issued("dsp-no", "cmt-1"), released("dsp-no")],
        vec![],
    );
    let record = rec.reconcile(input).unwrap();
    assert_eq!(record.outcome, ReconciliationOutcome::DispatchNotObserved);
}

#[test]
fn replay_attack_duplicate_reconcile() {
    let rec = engine();
    let input = base_input(
        "dsp-rep",
        "cmt-1",
        vec![
            ticket_issued("dsp-rep", "cmt-1"),
            released("dsp-rep"),
            dsp_outcome("dsp-rep", DispatchOutcome::Failed),
        ],
        vec![obs_record("dsp-rep", "e", ObservedClassification::ObservedFailure, "o")],
    );
    rec.reconcile(input.clone()).unwrap();
    assert_eq!(
        rec.reconcile(input).unwrap_err(),
        ReconciliationError::DuplicateReconciliation
    );
}

#[test]
fn duplicate_dispatch_id_in_history() {
    let rec = engine();
    let input = base_input(
        "dsp-dup",
        "cmt-1",
        vec![
            ticket_issued("dsp-dup", "cmt-1"),
            ticket_issued("dsp-dup", "cmt-1"),
        ],
        vec![],
    );
    assert_eq!(
        rec.reconcile(input).unwrap_err(),
        ReconciliationError::DuplicateDispatchId
    );
}

#[test]
fn multiple_valid_observations() {
    let rec = engine();
    let input = base_input(
        "dsp-mv",
        "cmt-1",
        vec![
            ticket_issued("dsp-mv", "cmt-1"),
            released("dsp-mv"),
            dsp_outcome("dsp-mv", DispatchOutcome::Success),
        ],
        vec![
            obs_record("dsp-mv", "e1", ObservedClassification::ObservedSuccess, "o1"),
            obs_record("dsp-mv", "e2", ObservedClassification::ObservedSuccess, "o2"),
        ],
    );
    let record = rec.reconcile(input).unwrap();
    assert_eq!(record.outcome, ReconciliationOutcome::MultipleValidObservations);
}
