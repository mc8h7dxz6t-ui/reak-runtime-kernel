mod rcv_fixtures;

use rcv_fixtures::{engine, input, truth};
use reak_recovery::RecoveryError;
use reak_truth::TruthConclusion;
use std::sync::Arc;
use std::thread;

#[test]
fn concurrent_distinct_truths() {
    let rcv = Arc::new(engine());
    let mut handles = vec![];
    for i in 0..5 {
        let r = Arc::clone(&rcv);
        handles.push(thread::spawn(move || {
            let tid = format!("tru-conc-{}", i);
            r.propose_recovery(input(truth(
                &tid,
                "dsp-conc",
                TruthConclusion::EstablishedSuccess,
                None,
            )))
            .unwrap();
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn concurrent_same_truth_one_winner() {
    let rcv = Arc::new(engine());
    let inp = input(truth("tru-race", "dsp-race", TruthConclusion::EstablishedFailure, None));
    let mut handles = vec![];
    for _ in 0..4 {
        let r = Arc::clone(&rcv);
        let inp = inp.clone();
        handles.push(thread::spawn(move || r.propose_recovery(inp)));
    }
    let mut ok = 0;
    let mut dup = 0;
    for h in handles {
        match h.join().unwrap() {
            Ok(_) => ok += 1,
            Err(RecoveryError::DuplicateRecovery) => dup += 1,
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_eq!(ok, 1);
    assert_eq!(dup, 3);
}
