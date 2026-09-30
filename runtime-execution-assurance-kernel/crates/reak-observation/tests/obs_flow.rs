mod obs_fixtures;

use obs_fixtures::{sample_input, tenant};
use reak_observation::{ObservationEngine, ObservedClassification};

#[test]
fn record_append_verify_success() {
    let engine = ObservationEngine::new(tenant());
    let pending = engine
        .record(sample_input("dsp_t1", "evt-1", ObservedClassification::ObservedSuccess))
        .unwrap();
    let rec = engine.append(pending, 100).unwrap();
    assert_eq!(rec.classification, ObservedClassification::ObservedSuccess);
    engine.verify().unwrap();
}

#[test]
fn unknown_stored_without_conversion() {
    let engine = ObservationEngine::new(tenant());
    let pending = engine
        .record(sample_input("dsp_t2", "evt-u", ObservedClassification::ObservedUnknown))
        .unwrap();
    let rec = engine.append(pending, 101).unwrap();
    assert_eq!(rec.classification, ObservedClassification::ObservedUnknown);
}

#[test]
fn zero_observations_for_ticket() {
    let engine = ObservationEngine::new(tenant());
    engine.verify().unwrap();
    assert!(engine.observations_for_ticket("missing").is_empty());
}
