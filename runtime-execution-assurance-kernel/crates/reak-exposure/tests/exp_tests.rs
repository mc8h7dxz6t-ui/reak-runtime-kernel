use reak_authority::{AuthorityRequest, AuthorityService, ScopeClass};
use reak_exposure::{ExposureError, ExposureReservation, ExposureService};
use reak_policy_context::{PolicyContext, PolicyEpochRef};
use reak_types::{EpochId, TenantId};

fn wired() -> ExposureService {
    let tenant = TenantId::parse("t").unwrap();
    let policy = PolicyContext::new(tenant.clone());
    let epoch = EpochId::parse("e").unwrap();
    let rules = vec![1u8];
    policy.register_epoch(epoch.clone(), rules.clone()).unwrap();
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(&rules).into();
    let handle = policy
        .pin_epoch(PolicyEpochRef {
            epoch_id: epoch,
            content_hash: hash,
        })
        .unwrap();
    let auth = AuthorityService::new(tenant.clone(), policy);
    auth.grant_attempt(AuthorityRequest {
        principal_id: "p1".into(),
        scope: ScopeClass {
            class_id: "c".into(),
        },
        policy: handle,
    })
    .unwrap();
    let exp = ExposureService::new(tenant, auth, 100);
    exp.set_ceiling("p1", 10);
    exp
}

#[test]
fn reserve_within_ceiling() {
    let exp = wired();
    let token = exp
        .reserve(ExposureReservation {
            principal_id: "p1".into(),
            scope: ScopeClass {
                class_id: "c".into(),
            },
            units: 5,
        })
        .unwrap();
    assert_eq!(token.units_reserved, 5);
    exp.release("p1", 5).unwrap();
}

#[test]
fn breach_fails_closed() {
    let exp = wired();
    let err = exp
        .reserve(ExposureReservation {
            principal_id: "p1".into(),
            scope: ScopeClass {
                class_id: "c".into(),
            },
            units: 11,
        })
        .unwrap_err();
    assert_eq!(err, ExposureError::CeilingBreached);
}
