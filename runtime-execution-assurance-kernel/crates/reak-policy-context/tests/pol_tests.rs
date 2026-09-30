use reak_policy_context::{PolicyContext, PolicyEpochRef};
use reak_types::{EpochId, TenantId};

#[test]
fn pin_requires_registered_epoch() {
    let ctx = PolicyContext::new(TenantId::parse("t").unwrap());
    let epoch = EpochId::parse("e1").unwrap();
    ctx.register_epoch(epoch.clone(), vec![1, 2, 3]).unwrap();
    let hash = {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update([1, 2, 3]);
        h.finalize().into()
    };
    let handle = ctx
        .pin_epoch(PolicyEpochRef {
            epoch_id: epoch,
            content_hash: hash,
        })
        .unwrap();
    assert!(handle.pinned_at_sequence.raw() >= 1);
}

#[test]
fn resolve_rejects_hash_mismatch() {
    let ctx = PolicyContext::new(TenantId::parse("t").unwrap());
    let epoch = EpochId::parse("e1").unwrap();
    ctx.register_epoch(epoch.clone(), vec![9]).unwrap();
    let err = ctx
        .resolve_epoch(&PolicyEpochRef {
            epoch_id: epoch,
            content_hash: [0u8; 32],
        })
        .unwrap_err();
    assert_eq!(err, reak_policy_context::PolicyContextError::HashMismatch);
}
