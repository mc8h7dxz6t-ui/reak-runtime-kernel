use crate::error::DispatchError;
use reak_authority::ScopeClass;
use reak_commitment::{CommitmentDigest, CommitmentEngine, CommitmentStatus};
use reak_exposure::ExposureService;
use reak_policy_context::{PolicyContext, PolicyEpochRef};
use reak_types::EpochId;
use reak_ues::UncertaintyLedger;

pub fn parse_grant(grant_id: &str) -> Option<(String, String)> {
    let mut parts = grant_id.splitn(2, ':');
    let principal = parts.next()?;
    let scope = parts.next()?;
    if principal.is_empty() || scope.is_empty() {
        return None;
    }
    Some((principal.to_string(), scope.to_string()))
}

pub struct VerificationContext<'a> {
    pub commitment_engine: &'a CommitmentEngine,
    pub exposure: &'a ExposureService,
    pub policy: &'a PolicyContext,
    pub ues: &'a UncertaintyLedger,
}

pub fn verify_pre_dispatch(
    ctx: &VerificationContext<'_>,
    commitment_id: &str,
    digest: &CommitmentDigest,
) -> Result<reak_commitment::CommitmentRecord, DispatchError> {
    let record = ctx
        .commitment_engine
        .verify_binding(commitment_id, digest)
        .map_err(DispatchError::Commitment)?;

    if record.status != CommitmentStatus::Bound {
        return Err(DispatchError::CommitmentNotDispatchable);
    }

    if record.reservation_link.reservation_id.is_empty() {
        return Err(DispatchError::ReservationInvalid);
    }

    let (principal, scope_id) = parse_grant(&record.authority_link.grant_id)
        .ok_or(DispatchError::AuthorityRevoked)?;
    let scope = ScopeClass {
        class_id: scope_id,
    };
    ctx.exposure
        .authority()
        .verify_scope(&principal, &scope)
        .map_err(|e| match e {
            reak_authority::AuthorityError::NotActive | reak_authority::AuthorityError::Denied => {
                DispatchError::AuthorityRevoked
            }
            other => DispatchError::Authority(other),
        })?;

    let epoch = EpochId::parse(&record.policy_epoch).map_err(|_| DispatchError::PolicyMismatch)?;
    ctx.policy
        .resolve_epoch(&PolicyEpochRef {
            epoch_id: epoch,
            content_hash: record.policy_content_hash,
        })
        .map_err(|e| match e {
            reak_policy_context::PolicyContextError::HashMismatch
            | reak_policy_context::PolicyContextError::UnknownEpoch => {
                DispatchError::PolicyMismatch
            }
            other => DispatchError::Policy(other),
        })?;

    // UES: bind already consumed budget; dispatch only checks bind left a finite remainder snapshot.
    if record.ues_remaining_after_bind > 1_000_000_000 {
        return Err(DispatchError::UesPrecondition);
    }

    let _ = ctx.ues;
    Ok(record)
}
