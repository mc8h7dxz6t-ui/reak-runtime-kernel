mod rec_fixtures;

use rec_fixtures::{base_input, dsp_outcome, engine, obs_record, released, ticket_issued};
use reak_dispatch::DispatchOutcome;
use reak_observation::ObservedClassification;
use reak_reconciliation::ReconciliationError;
use std::sync::Arc;
use std::thread;

#[test]
fn concurrent_reconcile_distinct_inputs() {
    let rec = Arc::new(engine());
    let mut handles = vec![];
    for i in 0..6 {
        let r = Arc::clone(&rec);
        let ticket = format!("dsp-conc-{}", i);
        handles.push(thread::spawn(move || {
            let input = base_input(
                &ticket,
                "cmt-1",
                vec![
                    ticket_issued(&ticket, "cmt-1"),
                    released(&ticket),
                    dsp_outcome(&ticket, DispatchOutcome::Success),
                ],
                vec![obs_record(
                    &ticket,
                    "e",
                    ObservedClassification::ObservedSuccess,
                    "o",
                )],
            );
            r.reconcile(input).unwrap();
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(rec.recover_from_log().unwrap(), 6);
}

#[test]
fn concurrent_same_input_one_winner() {
    let rec = Arc::new(engine());
    let input = base_input(
        "dsp-race",
        "cmt-1",
        vec![
            ticket_issued("dsp-race", "cmt-1"),
            released("dsp-race"),
            dsp_outcome("dsp-race", DispatchOutcome::Success),
        ],
        vec![obs_record("dsp-race", "e", ObservedClassification::ObservedSuccess, "o")],
    );
    let mut handles = vec![];
    for _ in 0..5 {
        let r = Arc::clone(&rec);
        let input = input.clone();
        handles.push(thread::spawn(move || r.reconcile(input)));
    }
    let mut ok = 0;
    let mut dup = 0;
    for h in handles {
        match h.join().unwrap() {
            Ok(_) => ok += 1,
            Err(ReconciliationError::DuplicateReconciliation) => dup += 1,
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_eq!(ok, 1);
    assert_eq!(dup, 4);
}
