use crate::error::ProgressionError;
use crate::model::{ProgressionInput, ProgressionState};
use reak_recovery::RecoveryStrategy;
use reak_truth::TruthConclusion;

pub fn input_digest_hex(input: &ProgressionInput) -> String {
    let payload = serde_json::to_vec(input).unwrap_or_default();
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(&payload))
}

pub fn validate_input(input: &ProgressionInput) -> Result<(), ProgressionError> {
    if input.truth.truth_id.is_empty()
        || input.recovery.recovery_id.is_empty()
        || input.policy.epoch_id.is_empty()
        || input.policy.content_hash == reak_types::RecordHash::ZERO
    {
        return Err(ProgressionError::InvalidInput);
    }
    if input.recovery.truth_id != input.truth.truth_id
        || input.authorization.truth_id != input.truth.truth_id
        || input.authorization.recovery_id != input.recovery.recovery_id
    {
        return Err(ProgressionError::BindingMismatch);
    }
    if input.recovery.authorizes_execution || input.authorization.authorizes_execution {
        return Err(ProgressionError::IllegalExecutionFlag);
    }
    Ok(())
}

/// Deterministic next-step classification (no dispatch, no mutation of inputs).
pub fn classify_next_state(input: &ProgressionInput) -> Result<ProgressionState, ProgressionError> {
    validate_input(input)?;
    let authorized = input.recovery.intent_authorized;

    match input.recovery.strategy {
        RecoveryStrategy::Abort => Ok(ProgressionState::Abort),
        RecoveryStrategy::Await => Ok(ProgressionState::Await),
        RecoveryStrategy::NoAction => {
            if input.truth.conclusion == TruthConclusion::EstablishedSuccess {
                Ok(ProgressionState::Terminal)
            } else {
                Ok(ProgressionState::NoFurtherAction)
            }
        }
        RecoveryStrategy::Retry => {
            if authorized {
                Ok(ProgressionState::RetryPermitted)
            } else {
                Ok(ProgressionState::Await)
            }
        }
        RecoveryStrategy::Compensate => {
            if authorized {
                Ok(ProgressionState::CompensationPermitted)
            } else {
                Ok(ProgressionState::Await)
            }
        }
        RecoveryStrategy::Escalate => {
            if authorized {
                Ok(ProgressionState::HumanApprovalRequired)
            } else {
                Ok(ProgressionState::Await)
            }
        }
    }
}
