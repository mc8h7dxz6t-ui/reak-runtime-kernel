# Phase 8 — Recovery engine

## Boundaries

| Allowed | Forbidden |
|---------|-----------|
| Read `TruthRecord` | Dispatch, observe, reconcile |
| Append `RecoveryRecord` | Modify or re-derive truth |
| Select `RecoveryStrategy` | `authorizes_execution: true` |

## API (IF-RCV-01)

- `propose_recovery(RecoveryInput)` — deterministic strategy + durable propose record  
- `authorize_recovery_intent(recovery_id)` — append authorization (`authorizes_execution` remains false)  
- `verify()` / `recover_from_log()`  

## Strategies

`NoAction`, `Retry`, `Compensate`, `Escalate`, `Await`, `Abort` — mapped deterministically from truth conclusion and non-established reason.

## Tests

12 integration tests; workspace green.
