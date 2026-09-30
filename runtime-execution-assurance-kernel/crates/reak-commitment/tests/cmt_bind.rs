mod cmt_fixtures;

use cmt_fixtures::{sample_intent, wired_stack};
use reak_commitment::CommitmentError;

#[test]
fn bind_produces_immutable_record_and_digest() {
    let stack = wired_stack();
    let intent = sample_intent(&stack.policy_handle);
    let binding = stack
        .commitment
        .bind(
            &stack.exposure,
            &stack.policy,
            &stack.ues,
            intent,
        )
        .unwrap();
    assert_eq!(binding.record.status, reak_commitment::CommitmentStatus::Bound);
    stack
        .commitment
        .verify_binding(&binding.record.commitment_id, &binding.digest)
        .unwrap();
}

#[test]
fn duplicate_operation_rejected() {
    let stack = wired_stack();
    let intent = sample_intent(&stack.policy_handle);
    stack
        .commitment
        .bind(
            &stack.exposure,
            &stack.policy,
            &stack.ues,
            intent.clone(),
        )
        .unwrap();
    let err = stack
        .commitment
        .bind(
            &stack.exposure,
            &stack.policy,
            &stack.ues,
            intent,
        )
        .unwrap_err();
    assert_eq!(err, CommitmentError::DuplicateOperation);
}

#[test]
fn void_then_verify_fails_closed() {
    let stack = wired_stack();
    let intent = sample_intent(&stack.policy_handle);
    let binding = stack
        .commitment
        .bind(
            &stack.exposure,
            &stack.policy,
            &stack.ues,
            intent,
        )
        .unwrap();
    stack
        .commitment
        .void_binding(&binding.record.commitment_id)
        .unwrap();
    let err = stack
        .commitment
        .verify_binding(&binding.record.commitment_id, &binding.digest)
        .unwrap_err();
    assert_eq!(err, CommitmentError::Voided);
}
