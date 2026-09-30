mod obs_fixtures;

use obs_fixtures::{sample_input, tenant};
use reak_observation::{ObservationEngine, ObservedClassification};

#[test]
fn crash_before_append_leaves_empty_log() {
    let engine = ObservationEngine::new(tenant());
    let _pending = engine
        .record(sample_input("dsp_crash1", "evt-c1", ObservedClassification::ObservedUnknown))
        .unwrap();
    assert_eq!(engine.recover_from_log().unwrap(), 0);
}

#[test]
fn crash_after_append_recoverable() {
    let engine = ObservationEngine::new(tenant());
    let pending = engine
        .record(sample_input("dsp_crash2", "evt-c2", ObservedClassification::ObservedSuccess))
        .unwrap();
    engine.append(pending, 500).unwrap();
    let n = engine.recover_from_log().unwrap();
    assert_eq!(n, 1);
    assert_eq!(engine.observations_for_ticket("dsp_crash2").len(), 1);
}

#[test]
fn recover_idempotent() {
    let engine = ObservationEngine::new(tenant());
    engine
        .append(
            engine
                .record(sample_input("dsp_crash3", "evt-c3", ObservedClassification::ObservedCancelled))
                .unwrap(),
            600,
        )
        .unwrap();
    let a = engine.recover_from_log().unwrap();
    let b = engine.recover_from_log().unwrap();
    assert_eq!(a, b);
}
