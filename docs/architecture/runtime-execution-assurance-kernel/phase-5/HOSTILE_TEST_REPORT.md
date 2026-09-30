# Phase 5 hostile test report

**Command:** `cargo test -p reak-observation`

| Test | Scenario | Result |
|------|----------|--------|
| `replay_same_source_event_rejected` | Replay attack / duplicate source event | Pass |
| `conflicting_classifications_both_durable` | Conflicting observation | Pass |
| `malformed_oversized_payload_rejected` | Malformed payload | Pass |
| `malformed_empty_ticket_rejected` | Invalid reference | Pass |
| `delayed_observation_appended_after_earlier` | Delayed / reordered timestamps | Pass |
| `crash_before_append_leaves_empty_log` | Crash before append | Pass |
| `crash_after_append_recoverable` | Crash after append | Pass |
| `concurrent_replay_event_one_winner` | Concurrent duplicate event | Pass |
| `zero_observations_for_ticket` | Missing observation | Pass |

**Corrupted history:** Chain break detection delegated to `reak-durable-record` / `reak-replay` (shared DR hostile suite). OBS recovery returns `Serialization` on non-deserializable payloads.
