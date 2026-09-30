# Hostile test report — Phase 2

Hostile tests live in crate `tests/` trees (not Qualification Kernel — per programme boundary).

| Test | Module | Scenario | Expected |
|------|--------|----------|----------|
| rejects_oversized_payload | durable-record | >1 MiB payload | `PayloadTooLarge` |
| concurrent_appends_produce_unique_sequences | durable-record | 32 threads append | Unique sequences 1..32, chain valid |
| consume_without_declare_fails_closed | ues | Consume undeclared stage | `BudgetNotDeclared` |
| overrun_records_violation_and_fails | ues | Double consume over budget | `BudgetExhausted` |
| resolve_rejects_hash_mismatch | policy-context | Wrong content hash | `HashMismatch` |
| breach_fails_closed | exposure | Reserve over ceiling | `CeilingBreached` + audit append |
| revoke_prevents_verify | authority | Use after revoke | `NotActive` |

## Not yet automated (deferred)

- Serialization corruption injection (requires fuzz target on decode paths)
- Crash simulation mid-append (requires persistence layer)
- Memory exhaustion (explicit soak in Qualification Kernel)

These do not block Phase 2 implementation complete; they feed Qualification Kernel backlog.
