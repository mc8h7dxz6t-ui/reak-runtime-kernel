# Phase 4 property test report

**Command:** `cargo test -p reak-dispatch --test dsp_proptest --test dsp_state_machine`

| Property | Test | Result |
|----------|------|--------|
| All listed outcomes terminal | `terminal_outcomes_are_closed`, `all_terminal_outcomes_ack_once` | Pass |
| Illegal skip Released→AwaitingAck without Released state | `skip_released_to_awaiting_is_illegal` | Pass |
| Documented legal edges | `legal_transitions_documented` | Pass |
| Backward transitions illegal | `illegal_transitions_rejected` | Pass |

Property tests complement unit tests; they do not replace hostile concurrency cases (see HOSTILE_TEST_REPORT.md).
