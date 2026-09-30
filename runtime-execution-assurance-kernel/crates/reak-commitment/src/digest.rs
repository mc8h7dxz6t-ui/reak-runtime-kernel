use crate::model::{AuthorityLink, CommitmentIntent, OperationIdentity, PlaneReference, ReservationLink};
use sha2::{Digest, Sha256};

pub fn operation_digest(operation: &OperationIdentity, intent_bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(operation.lineage_id.as_bytes());
    h.update(operation.generation.to_le_bytes());
    h.update((intent_bytes.len() as u64).to_le_bytes());
    h.update(intent_bytes);
    h.finalize().into()
}

pub fn commitment_digest(
    operation_digest: &[u8; 32],
    authority: &AuthorityLink,
    reservation: &ReservationLink,
    policy_epoch: &str,
    policy_hash: &[u8; 32],
    provider_id: &str,
    truth: &PlaneReference,
    recovery: &PlaneReference,
    progression: &PlaneReference,
    wall_time_unix_ms: u64,
) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(operation_digest);
    h.update(authority.grant_id.as_bytes());
    h.update(authority.issued_at_sequence.raw().to_le_bytes());
    h.update(reservation.reservation_id.as_bytes());
    h.update(reservation.units_reserved.to_le_bytes());
    h.update(policy_epoch.as_bytes());
    h.update(policy_hash);
    h.update(provider_id.as_bytes());
    hash_plane_ref(&mut h, truth);
    hash_plane_ref(&mut h, recovery);
    hash_plane_ref(&mut h, progression);
    h.update(wall_time_unix_ms.to_le_bytes());
    h.finalize().into()
}

fn hash_plane_ref(h: &mut Sha256, r: &PlaneReference) {
    h.update(r.registry_pointer.as_bytes());
    h.update(r.record_sequence.raw().to_le_bytes());
}

pub fn commitment_id_from_digest(digest: &[u8; 32]) -> String {
    format!("cmt_{}", hex_encode(digest))
}

fn hex_encode(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn validate_intent_refs(intent: &CommitmentIntent) -> bool {
    !intent.operation.lineage_id.is_empty()
        && intent.provider_id.len() <= 256
        && !intent.intent_bytes.is_empty()
        && !intent.truth_ref.registry_pointer.is_empty()
        && !intent.recovery_ref.registry_pointer.is_empty()
        && !intent.progression_ref.registry_pointer.is_empty()
        && intent.exposure_units > 0
        && intent.ues_eliminable_units > 0
}
