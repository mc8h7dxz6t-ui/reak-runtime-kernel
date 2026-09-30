mod cmt_fixtures;

use cmt_fixtures::{sample_intent, wired_stack};
use proptest::prelude::*;

proptest! {
    #[test]
    fn digest_verify_deterministic(seed in 0u64..1000) {
        let stack = wired_stack();
        let mut intent = sample_intent(&stack.policy_handle);
        intent.operation.generation = seed + 1;
        intent.operation.lineage_id = format!("line-{}", seed);
        let binding = stack
            .commitment
            .bind(&stack.exposure, &stack.policy, &stack.ues, intent)
            .unwrap();
        let again = stack.commitment.verify_binding(
            &binding.record.commitment_id,
            &binding.digest,
        ).unwrap();
        prop_assert_eq!(again.commitment_id, binding.record.commitment_id);
    }
}
