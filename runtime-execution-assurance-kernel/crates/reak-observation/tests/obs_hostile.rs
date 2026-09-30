mod obs_fixtures;

use obs_fixtures::{sample_input, tenant};
use reak_observation::{ObservationEngine, ObservationError, ObservedClassification};
use reak_types::MAX_RECORD_PAYLOAD_BYTES;

#[test]
fn replay_same_source_event_rejected() {
    let engine = ObservationEngine::new(tenant());
    let input = sample_input("dsp_t3", "evt-dup", ObservedClassification::ObservedFailure);
    let p1 = engine.record(input.clone()).unwrap();
    engine.append(p1, 200).unwrap();
    let p2 = engine.record(input).unwrap();
    let err = engine.append(p2, 201).unwrap_err();
    assert_eq!(err, ObservationError::ReplayRejected);
}

#[test]
fn conflicting_classifications_both_durable() {
    let engine = ObservationEngine::new(tenant());
    let p1 = engine
        .record(sample_input("dsp_t4", "evt-a", ObservedClassification::ObservedSuccess))
        .unwrap();
    let p2 = engine
        .record(sample_input("dsp_t4", "evt-b", ObservedClassification::ObservedFailure))
        .unwrap();
    engine.append(p1, 300).unwrap();
    engine.append(p2, 301).unwrap();
    let obs = engine.observations_for_ticket("dsp_t4");
    assert_eq!(obs.len(), 2);
    assert_ne!(obs[0].classification, obs[1].classification);
}

#[test]
fn malformed_oversized_payload_rejected() {
    let engine = ObservationEngine::new(tenant());
    let mut input = sample_input("dsp_t5", "evt-big", ObservedClassification::ObservedUnknown);
    input.raw_payload = vec![0u8; MAX_RECORD_PAYLOAD_BYTES + 1];
    assert_eq!(
        engine.record(input).unwrap_err(),
        ObservationError::MalformedPayload
    );
}

#[test]
fn malformed_empty_ticket_rejected() {
    let engine = ObservationEngine::new(tenant());
    let input = sample_input("", "evt-x", ObservedClassification::ObservationUnavailable);
    assert_eq!(
        engine.record(input).unwrap_err(),
        ObservationError::InvalidReference
    );
}

#[test]
fn delayed_observation_appended_after_earlier() {
    let engine = ObservationEngine::new(tenant());
    let mut late = sample_input("dsp_t6", "evt-late", ObservedClassification::ObservedTimeout);
    late.reference.observed_at_unix_ms = 9_999_999_999_999;
    let early = sample_input("dsp_t6", "evt-early", ObservedClassification::ObservedSuccess);
    engine.append(engine.record(early).unwrap(), 400).unwrap();
    engine.append(engine.record(late).unwrap(), 401).unwrap();
    let obs = engine.observations_for_ticket("dsp_t6");
    assert_eq!(obs.len(), 2);
    assert_eq!(obs[0].reference.source_event_id, "evt-early");
}
