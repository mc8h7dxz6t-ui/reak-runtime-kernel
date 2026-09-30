# Phase 7 — Truth engine

## API (IF-TRU-01)

| Method | Role |
|--------|------|
| `derive_conclusion(input)` | Map `ReconciliationRecord` + `AdmissibilitySet` → append `TruthRecord` |
| `preserve_non_established(input, reason)` | Explicit non-established record; never upgrades |
| `verify()` | TRU stream chain verification |
| `recover_from_log()` | Index rebuild only |

## Conclusions

`EstablishedSuccess`, `EstablishedFailure`, `NonEstablished` (+ `NonEstablishedReason`).

Only `ReconciledSuccess` / `ReconciledFailure` with full admissibility may establish. Unknown/conflict/insufficient reconciliation outcomes remain non-established.

## Dependencies

`reak-reconciliation`, `reak-durable-record`, `reak-replay`, `reak-types`. Admissibility is a caller-supplied snapshot (evidence hooks crate deferred).

## Tests

11 integration tests; workspace green.
