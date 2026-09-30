# Phase 6 implementation report

## Crate

`reak-reconciliation` — IF-REC-01

## API

| Operation | Behaviour |
|-----------|-----------|
| `compare_expected_observed` | Pure structural compare: missing, conflicting, or classification mismatch vs aligned |
| `emit_mismatch` | Append immutable `ReconciliationRecord` when verdict is mismatch; consumes UES stage budget |
| `verify` | Hash-chain verification of REC stream |
| `recover_from_log` | Rebuild indexes; no truth derivation |

## Dependencies

`reak-observation`, `reak-ues`, `reak-durable-record`, `reak-replay`, `reak-types`. No `reak-dispatch`, `reak-truth`, or network.

## Rules

- Multiple observations per ticket: conflict if classifications differ; records list all pairs on emit.
- `ObservedUnknown` / `ObservationUnavailable` never aligned with expected Success/Failure.
- Aligned comparisons are not durably emitted (`NotAMismatch`).

## Tests

8 integration tests (flow, hostile, recovery, proptest). Workspace green.
