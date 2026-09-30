# Phase 9 hostile test report

**Command:** `cargo test -p reak-progression --test prg_hostile`

## Cases

| Test | Attack / edge | Expected | Result |
|------|---------------|----------|--------|
| `binding_mismatch_rejected` | Recovery references wrong `truth_id` | `BindingMismatch` | Pass |
| `illegal_execution_flag_rejected` | `recovery.authorizes_execution = true` | `IllegalExecutionFlag` | Pass |
| `duplicate_progression_rejected` | Same input twice | `DuplicateProgression` | Pass |
| `escalate_maps_to_human_approval` | Authorized escalate | `HumanApprovalRequired` (not execution) | Pass |

## Concurrency (related)

`prg_concurrency`: concurrent `determine_next_step` on same recovery — exactly one winner; others get `DuplicateProgression`. Distinct recoveries append independently.

## Conclusion

Hostile suite passes. Progression rejects execution flags and binding violations and enforces single progression per recovery input.
