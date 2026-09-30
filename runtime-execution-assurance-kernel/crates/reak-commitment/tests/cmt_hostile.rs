mod cmt_fixtures;

use cmt_fixtures::{sample_intent, wired_stack};
use reak_commitment::CommitmentError;
use std::sync::Arc;
use std::thread;

#[test]
fn authority_revoked_before_bind_fails() {
    let stack = wired_stack();
    stack
        .exposure
        .authority()
        .revoke("principal:effect.pay")
        .unwrap();
    let err = stack
        .commitment
        .bind(
            &stack.exposure,
            &stack.policy,
            &stack.ues,
            sample_intent(&stack.policy_handle),
        )
        .unwrap_err();
    assert_eq!(err, CommitmentError::AuthorityRevoked);
}

#[test]
fn concurrent_bind_same_operation_one_wins() {
    let stack = Arc::new(wired_stack());
    let intent = sample_intent(&stack.policy_handle);
    let mut handles = vec![];
    for _ in 0..8 {
        let s = Arc::clone(&stack);
        let intent = intent.clone();
        handles.push(thread::spawn(move || {
            s.commitment
                .bind(&s.exposure, &s.policy, &s.ues, intent)
        }));
    }
    let mut ok = 0;
    let mut dup = 0;
    for h in handles {
        match h.join().unwrap() {
            Ok(_) => ok += 1,
            Err(CommitmentError::DuplicateOperation) => dup += 1,
            Err(e) => panic!("unexpected {:?}", e),
        }
    }
    assert_eq!(ok, 1);
    assert_eq!(dup, 7);
}

#[test]
fn ues_exhaustion_fails_closed() {
    let stack = wired_stack();
    let mut intent = sample_intent(&stack.policy_handle);
    intent.ues_eliminable_units = 10_000;
    let err = stack
        .commitment
        .bind(&stack.exposure, &stack.policy, &stack.ues, intent)
        .unwrap_err();
    assert!(matches!(err, CommitmentError::UesExhausted(_)));
}

#[test]
fn digest_mismatch_on_verify() {
    let stack = wired_stack();
    let binding = stack
        .commitment
        .bind(
            &stack.exposure,
            &stack.policy,
            &stack.ues,
            sample_intent(&stack.policy_handle),
        )
        .unwrap();
    let bad = reak_commitment::CommitmentDigest([0u8; 32]);
    let err = stack
        .commitment
        .verify_binding(&binding.record.commitment_id, &bad)
        .unwrap_err();
    assert_eq!(err, CommitmentError::DigestMismatch);
}
