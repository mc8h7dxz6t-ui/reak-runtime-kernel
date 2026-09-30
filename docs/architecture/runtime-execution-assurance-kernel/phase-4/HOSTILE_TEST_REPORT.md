# Phase 4 hostile test report

**Command:** `cargo test -p reak-dispatch --test dsp_hostile --test dsp_recovery`

| Test | Scenario | Expected | Result |
|------|----------|----------|--------|
| `duplicate_ticket_rejected` | Second `issue_ticket` same commitment | `DuplicateDispatch` | Pass |
| `concurrent_issue_single_winner` | 6 threads race `issue_ticket` | 1 ok, 5 duplicate | Pass |
| `authority_revoked_before_issue` | Revoke grant before issue | `AuthorityRevoked` | Pass |
| `double_execute_rejected` | Two `execute_once` same ticket | Second err | Pass |
| `recover_after_crash_before_send_allows_single_release` | Log replay after issue only | `execute_once` ok once | Pass |
| `recover_after_crash_after_send_blocks_second_release` | Log replay after release | Second execute err | Pass |
| `recover_idempotent_after_terminal_outcome` | Double recovery | Stable entry count | Pass |

**Flow hostile themes (dsp_flow):**

| Test | Theme | Result |
|------|-------|--------|
| `unknown_does_not_allow_second_execute` | Lost ack / unknown must not re-dispatch | Pass |

**Coverage gaps (accepted for Phase 4):** Randomised scheduling across mixed issue/execute/ack interleaving deferred to Qualification Kernel; kernel provides deterministic hooks and serialisation points.
