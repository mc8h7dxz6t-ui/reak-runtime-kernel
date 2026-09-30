# Phase 5 threat model — Observation engine

| ID | Threat | Mitigation |
|----|--------|------------|
| T-OBS-01 | Forged observation | Validate references; bounded payload; content digest stored |
| T-OBS-02 | Duplicated observation (replay) | Reject duplicate `(observation_source, source_event_id)` |
| T-OBS-03 | Reordered observations | Append order is canonical; timestamps not used to reorder |
| T-OBS-04 | Delayed observation | Allowed as separate event with own `source_event_id` |
| T-OBS-05 | Replay attack | Source event idempotency index fail-closed |
| T-OBS-06 | Stale provider response | Stored as-is; staleness handled in reconciliation (out of scope) |
| T-OBS-07 | Malformed payload | Size limits; invalid references rejected |
| T-OBS-08 | Timestamp manipulation | Timestamp stored as claimed; not trusted for truth |
| T-OBS-09 | Identifier collision | Observation id includes durable sequence after append |
| T-OBS-10 | Log corruption | `verify()` / `recover_from_log` use replay chain verification |
