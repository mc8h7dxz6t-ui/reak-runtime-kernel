mod prg_fixtures;

use prg_fixtures::{authorization, engine, input, recovery, truth};
use reak_progression::ProgressionError;
use reak_recovery::RecoveryStrategy;
use reak_truth::TruthConclusion;
use std::sync::Arc;
use std::thread;

#[test]
fn concurrent_distinct_recoveries() {
    let prg = Arc::new(engine());
    let mut handles = vec![];
    for i in 0..5 {
        let p = Arc::clone(&prg);
        handles.push(thread::spawn(move || {
            let tid = format!("tru-{}", i);
            let rid = format!("rcv-{}", i);
            let tr = truth(&tid, "dsp-c", TruthConclusion::EstablishedSuccess);
            let rec = recovery(&rid, &tid, "dsp-c", RecoveryStrategy::NoAction, true);
            p.determine_next_step(input(tr, rec, authorization(&rid, &tid)))
                .unwrap();
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn concurrent_same_recovery_one_winner() {
    let prg = Arc::new(engine());
    let tr = truth("tru-race", "dsp-race", TruthConclusion::EstablishedFailure);
    let rec = recovery("rcv-race", "tru-race", "dsp-race", RecoveryStrategy::Compensate, true);
    let inp = input(tr, rec, authorization("rcv-race", "tru-race"));
    let mut handles = vec![];
    for _ in 0..4 {
        let p = Arc::clone(&prg);
        let inp = inp.clone();
        handles.push(thread::spawn(move || p.determine_next_step(inp)));
    }
    let mut ok = 0;
    let mut dup = 0;
    for h in handles {
        match h.join().unwrap() {
            Ok(_) => ok += 1,
            Err(ProgressionError::DuplicateProgression) => dup += 1,
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_eq!(ok, 1);
    assert_eq!(dup, 3);
}
