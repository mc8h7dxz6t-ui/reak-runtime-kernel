# Phase 9 — Progression engine implementation

## Boundaries

| Allowed | Forbidden |
|---------|-----------|
| Read immutable `TruthRecord`, `RecoveryRecord`, `RecoveryIntentAuthorization`, `PolicySnapshotRef` | Dispatch, observe, reconcile, derive truth |
| Append-only `ProgressionRecord` to durable PRG stream | Mutate truth, recovery, or prior progression history |
| Deterministic `ProgressionState` classification | Execute retry, compensation, or human workflows |
| `verify()` / `recover_from_log()` replay | Adapters, providers, network I/O |

## API (IF-PRG-01)

- `ProgressionEngine::determine_next_step(ProgressionInput)` — validate bindings, classify state, append durable record  
- `ProgressionEngine::verify()` — replay chain integrity (empty stream OK)  
- `ProgressionEngine::recover_from_log()` — rebuild in-memory indexes from durable log  

`ProgressionInput` bundles truth, recovery, authorization, and policy snapshot ref. Classification is pure (`classify_next_state`) after validation.

## Module layout

| Module | Role |
|--------|------|
| `model` | `ProgressionInput`, `ProgressionState`, `ProgressionRecord`, history envelope |
| `classify` | `validate_input`, `input_digest_hex`, `classify_next_state` |
| `engine` | Durable append, dedupe by recovery id and input digest, concurrency serial mutex |
| `error` | `ProgressionError` taxonomy |

**Stream:** `StreamId(0x0050_5247)` — append-only JSON `ProgressionHistoryEntry::Determined`.

## Dedupe and identity

- At most one progression per `recovery_id` and per canonical input digest (SHA-256 of JSON-serialized `ProgressionInput`).  
- `progression_id` = `prg_{dispatch_ticket_id}_{durable_sequence}`.

## Tests (integration)

| Suite | Count | Focus |
|-------|-------|-------|
| `prg_flow` | 3 | Terminal, retry permitted / await |
| `prg_hostile` | 4 | Binding, execution flag, duplicate, escalate |
| `prg_recovery` | 1 | `recover_from_log` after determine |
| `prg_concurrency` | 2 | Same recovery race, distinct recoveries |
| `prg_proptest` | 1 | Classification determinism |
| `prg_performance` | 1 | 1000 classifications &lt; 500 ms |

**Total:** 12 integration tests; full workspace `cargo test --workspace` green.

## Frozen phases

Phases 1–8 crates are unchanged except workspace membership for `reak-progression`. No adapter or provider code added.
