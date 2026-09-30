# Phase 5 implementation report — Observation engine

## Mission

`reak-observation` implements IF-OBS-01: durable, append-only storage of what external sources **appear** to report, without inferring truth or touching dispatch/commitment state.

## Crate layout

| Module | Responsibility |
|--------|----------------|
| `validate` | Input bounds; content digest; no value invention |
| `model` | References, classifications, pending vs durable records |
| `state_machine` | Documents that classifications are never inferred |
| `engine` | `record`, `append`, `verify`, `recover_from_log` |

## Public API

- `record(input)` — validate and build `PendingObservation` (no durable write)  
- `append(pending, appended_at_unix_ms)` — single immutable `ObservationRecord` + hash-chained log entry  
- `verify()` — replay chain verification (empty stream OK)  
- `recover_from_log()` — rebuild indexes only; no reconciliation  

## Independence

Dependencies: `reak-types`, `reak-durable-record`, `reak-replay` only. **No** `reak-dispatch`, `reak-commitment`, or network crates.

Dispatch ticket id is an opaque string reference supplied by the caller (simulated adapter in tests).

## Classifications

Caller-supplied: `ObservedSuccess`, `ObservedFailure`, `ObservedUnknown`, `ObservedTimeout`, `ObservedCancelled`, `ObservationUnavailable`. Engine stores verbatim.

## Multi-observation

Per-ticket vector preserves append order. Conflicting classifications for the same ticket remain separate records. Duplicate `(observation_source, source_event_id)` rejected (`ReplayRejected`).

## Tests

17 integration tests across flow, hostile, recovery, concurrency, property, and state-machine suites. Workspace tests green.
