use crate::error::ObservationError;
use crate::model::{ObservationInput, PendingObservation};
use reak_types::{MAX_ID_UTF8_BYTES, MAX_RECORD_PAYLOAD_BYTES};
use sha2::{Digest, Sha256};

fn field_ok(s: &str) -> bool {
    !s.is_empty() && s.len() <= MAX_ID_UTF8_BYTES
}

pub fn validate_input(input: &ObservationInput) -> Result<(), ObservationError> {
    let r = &input.reference;
    if !field_ok(&r.dispatch_ticket_id)
        || !field_ok(&r.provider_id)
        || !field_ok(&r.provider_operation_id)
        || !field_ok(&r.observation_source)
        || !field_ok(&r.source_event_id)
    {
        return Err(ObservationError::InvalidReference);
    }
    for c in &r.correlation_ids {
        if !field_ok(c) {
            return Err(ObservationError::InvalidReference);
        }
    }
    if r.observed_at_unix_ms == 0 {
        return Err(ObservationError::InvalidTimestamp);
    }
    if input.raw_payload.len() > MAX_RECORD_PAYLOAD_BYTES {
        return Err(ObservationError::MalformedPayload);
    }
    for (k, v) in &input.metadata.labels {
        if !field_ok(k) || !field_ok(v) {
            return Err(ObservationError::MalformedPayload);
        }
    }
    Ok(())
}

pub fn digest_hex(input: &ObservationInput) -> String {
    let mut h = Sha256::new();
    h.update(input.reference.dispatch_ticket_id.as_bytes());
    h.update(input.reference.provider_operation_id.as_bytes());
    h.update(input.reference.source_event_id.as_bytes());
    h.update(&input.raw_payload);
    h.update([input.classification as u8]);
    hex::encode(h.finalize())
}

pub fn record_pending(input: ObservationInput) -> Result<PendingObservation, ObservationError> {
    validate_input(&input)?;
    let content_digest_hex = digest_hex(&input);
    Ok(PendingObservation {
        reference: input.reference,
        classification: input.classification,
        raw_payload: input.raw_payload,
        metadata: input.metadata,
        content_digest_hex,
    })
}
