use reak_observation::{
    ObservationInput, ObservationMetadata, ObservationReference, ObservedClassification,
};
use reak_types::TenantId;

pub fn sample_input(ticket: &str, event_id: &str, class: ObservedClassification) -> ObservationInput {
    ObservationInput {
        reference: ObservationReference {
            dispatch_ticket_id: ticket.into(),
            provider_id: "provider.sim".into(),
            provider_operation_id: "op-1".into(),
            correlation_ids: vec!["corr-a".into()],
            observed_at_unix_ms: 1_700_000_001_000,
            observation_source: "sim-adapter".into(),
            source_event_id: event_id.into(),
        },
        classification: class,
        raw_payload: vec![1, 2, 3],
        metadata: ObservationMetadata {
            labels: vec![("lane".into(), "test".into())],
        },
    }
}

pub fn tenant() -> TenantId {
    TenantId::parse("tenant-obs").unwrap()
}
