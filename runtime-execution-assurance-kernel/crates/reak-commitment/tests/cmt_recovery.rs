mod cmt_fixtures;

use cmt_fixtures::{sample_intent, wired_stack};

#[test]
fn recover_from_log_is_idempotent() {
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
    let id = binding.record.commitment_id.clone();
    assert_eq!(stack.commitment.recover_from_log().unwrap(), 1);
    assert_eq!(stack.commitment.recover_from_log().unwrap(), 1);
    stack
        .commitment
        .verify_binding(&id, &binding.digest)
        .unwrap();
}
