mod tru_fixtures;

use tru_fixtures::{engine, input, reconciliation_record};
use reak_reconciliation::ReconciliationOutcome;
use reak_truth::TruthError;
use std::sync::Arc;
use std::thread;

#[test]
fn concurrent_distinct_reconciliations() {
    let tru = Arc::new(engine());
    let mut handles = vec![];
    for i in 0..5 {
        let t = Arc::clone(&tru);
        handles.push(thread::spawn(move || {
            let rec_id = format!("rec-conc-{}", i);
            let ticket = format!("dsp-conc-{}", i);
            t.derive_conclusion(input(reconciliation_record(
                &rec_id,
                &ticket,
                ReconciliationOutcome::ReconciledSuccess,
            )))
            .unwrap();
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(tru.recover_from_log().unwrap(), 5);
}

#[test]
fn concurrent_same_reconciliation_one_winner() {
    let tru = Arc::new(engine());
    let inp = input(reconciliation_record(
        "rec-race",
        "dsp-race",
        ReconciliationOutcome::ReconciledSuccess,
    ));
    let mut handles = vec![];
    for _ in 0..4 {
        let t = Arc::clone(&tru);
        let inp = inp.clone();
        handles.push(thread::spawn(move || t.derive_conclusion(inp)));
    }
    let mut ok = 0;
    let mut dup = 0;
    for h in handles {
        match h.join().unwrap() {
            Ok(_) => ok += 1,
            Err(TruthError::DuplicateTruth) => dup += 1,
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_eq!(ok, 1);
    assert_eq!(dup, 3);
}
