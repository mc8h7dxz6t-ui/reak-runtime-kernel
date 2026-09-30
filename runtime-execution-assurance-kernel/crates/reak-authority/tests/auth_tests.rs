use reak_authority::{AuthorityRequest, AuthorityService, ScopeClass};
use reak_policy_context::{PolicyContext, PolicyEpochRef};
use reak_types::{EpochId, TenantId};

fn epoch_hash(rules: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(rules);
    h.finalize().into()
}

fn setup() -> (AuthorityService, reak_policy_context::PolicySnapshotHandle) {
    let tenant = TenantId::parse("tenant").unwrap();
    let policy = PolicyContext::new(tenant.clone());
    let epoch = EpochId::parse("pol-v1").unwrap();
    let rules = vec![10u8, 20];
    policy.register_epoch(epoch.clone(), rules.clone()).unwrap();
    let handle = policy
        .pin_epoch(PolicyEpochRef {
            epoch_id: epoch,
            content_hash: epoch_hash(&rules),
        })
        .unwrap();
    let auth = AuthorityService::new(tenant, policy);
    (auth, handle)
}

#[test]
fn grant_and_verify_scope() {
    let (auth, policy_handle) = setup();
    auth.grant_attempt(AuthorityRequest {
        principal_id: "alice".into(),
        scope: ScopeClass {
            class_id: "pay".into(),
        },
        policy: policy_handle,
    })
    .unwrap();
    let rec = auth
        .verify_scope(
            "alice",
            &ScopeClass {
                class_id: "pay".into(),
            },
        )
        .unwrap();
    assert_eq!(rec.principal_id, "alice");
}

#[test]
fn revoke_prevents_verify() {
    let (auth, policy_handle) = setup();
    auth.grant_attempt(AuthorityRequest {
        principal_id: "bob".into(),
        scope: ScopeClass {
            class_id: "x".into(),
        },
        policy: policy_handle,
    })
    .unwrap();
    auth.revoke("bob:x").unwrap();
    assert!(auth
        .verify_scope("bob", &ScopeClass { class_id: "x".into() })
        .is_err());
}
