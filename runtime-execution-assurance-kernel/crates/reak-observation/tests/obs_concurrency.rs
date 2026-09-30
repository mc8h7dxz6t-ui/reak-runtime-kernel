mod obs_fixtures;

use obs_fixtures::{sample_input, tenant};
use reak_observation::{ObservationEngine, ObservationError, ObservedClassification};
use std::sync::Arc;
use std::thread;

#[test]
fn concurrent_appends_unique_observations() {
    let engine = Arc::new(ObservationEngine::new(tenant()));
    let mut handles = vec![];
    for i in 0..8 {
        let e = Arc::clone(&engine);
        handles.push(thread::spawn(move || {
            let event_id = format!("evt-{}", i);
            let input = sample_input(
                "dsp_conc",
                &event_id,
                ObservedClassification::ObservedUnknown,
            );
            let p = e.record(input).unwrap();
            e.append(p, 700 + i as u64).unwrap()
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(engine.observations_for_ticket("dsp_conc").len(), 8);
}

#[test]
fn concurrent_replay_event_one_winner() {
    let engine = Arc::new(ObservationEngine::new(tenant()));
    let input = sample_input("dsp_race", "evt-race", ObservedClassification::ObservedFailure);
    let mut handles = vec![];
    for i in 0..5 {
        let e = Arc::clone(&engine);
        let input = input.clone();
        handles.push(thread::spawn(move || {
            let p = e.record(input).unwrap();
            e.append(p, 800 + i as u64)
        }));
    }
    let mut ok = 0;
    let mut replay = 0;
    for h in handles {
        match h.join().unwrap() {
            Ok(_) => ok += 1,
            Err(ObservationError::ReplayRejected) => replay += 1,
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_eq!(ok, 1);
    assert_eq!(replay, 4);
}
