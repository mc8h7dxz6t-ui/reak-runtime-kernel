use reak_authority::{AuthorityRequest, AuthorityService, ScopeClass};
use reak_commitment::{CommitmentEngine, CommitmentIntent, OperationIdentity, PlaneReference};
use reak_dispatch::DispatchEngine;
use reak_exposure::ExposureService;
use reak_policy_context::{PolicyContext, PolicyEpochRef};
use reak_types::{EpochId, RecordSequence, TenantId};
use reak_ues::{StageBudget, UncertaintyLedger};
use sha2::{Digest, Sha256};

pub struct WiredDispatch {
    pub tenant: TenantId,
    pub policy: PolicyContext,
    pub exposure: ExposureService,
    pub ues: UncertaintyLedger,
    pub commitment: CommitmentEngine,
    pub dispatch: DispatchEngine,
    pub policy_handle: reak_policy_context::PolicySnapshotHandle,
    pub binding: reak_commitment::CommitmentBinding,
}

pub fn wired_dispatch() -> WiredDispatch {
    let tenant = TenantId::parse("tenant-dsp").unwrap();
    let epoch = EpochId::parse("epoch-1").unwrap();
    let rules = vec![7u8, 8, 9];
    let hash = Sha256::digest(&rules);

    let policy_auth = PolicyContext::new(tenant.clone());
    policy_auth.register_epoch(epoch.clone(), rules.clone()).unwrap();
    let policy_handle = policy_auth
        .pin_epoch(PolicyEpochRef {
            epoch_id: epoch.clone(),
            content_hash: hash.into(),
        })
        .unwrap();

    let authority = AuthorityService::new(tenant.clone(), policy_auth);
    authority
        .grant_attempt(AuthorityRequest {
            principal_id: "principal".into(),
            scope: ScopeClass {
                class_id: "effect.pay".into(),
            },
            policy: policy_handle.clone(),
        })
        .unwrap();

    let policy = PolicyContext::new(tenant.clone());
    policy
        .register_epoch(epoch.clone(), rules.clone())
        .unwrap();

    let exposure = ExposureService::new(tenant.clone(), authority, 1_000);
    exposure.set_ceiling("principal", 100);

    let ues = UncertaintyLedger::new(tenant.clone());
    ues.declare_budget(StageBudget {
        stage: reak_commitment::COMMITMENT_UES_STAGE,
        max_eliminable_units: 50,
    })
    .unwrap();

    let commitment = CommitmentEngine::new(tenant.clone());
    let intent = CommitmentIntent {
        operation: OperationIdentity {
            lineage_id: "line-dsp-1".into(),
            generation: 1,
        },
        principal_id: "principal".into(),
        scope_class_id: "effect.pay".into(),
        provider_id: "provider.bank".into(),
        intent_bytes: vec![9, 9, 9],
        truth_ref: PlaneReference {
            registry_pointer: "truth/ptr".into(),
            record_sequence: RecordSequence::new(1),
        },
        recovery_ref: PlaneReference {
            registry_pointer: "rcv/ptr".into(),
            record_sequence: RecordSequence::new(1),
        },
        progression_ref: PlaneReference {
            registry_pointer: "prg/ptr".into(),
            record_sequence: RecordSequence::new(1),
        },
        exposure_units: 5,
        ues_eliminable_units: 1,
        policy: policy_handle.clone(),
        wall_time_unix_ms: 1_700_000_000_000,
    };
    let binding = commitment
        .bind(&exposure, &policy, &ues, intent)
        .unwrap();

    let dispatch = DispatchEngine::new(tenant.clone());

    WiredDispatch {
        tenant,
        policy,
        exposure,
        ues,
        commitment,
        dispatch,
        policy_handle,
        binding,
    }
}
