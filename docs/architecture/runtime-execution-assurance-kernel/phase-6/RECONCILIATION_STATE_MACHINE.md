# Reconciliation lifecycle

Not a truth state machine — only compare outcomes and optional mismatch emission.

## Compare verdicts

| Verdict | Meaning |
|---------|---------|
| `Aligned` | Single consistent observation class matches expected |
| `Mismatch::MissingObservations` | No observations for ticket |
| `Mismatch::ConflictingObservations` | Multiple distinct classes for same ticket |
| `Mismatch::ClassificationMismatch` | Single class does not match expected (includes unknown vs success/failure) |

## Emit rules

- `emit_mismatch` only when verdict is `Mismatch`  
- `Aligned` → `NotAMismatch` (no durable record)  
- Each emit appends one immutable `ReconciliationHistoryEntry::MismatchEmitted`

## Forbidden

No truth conclusion, no retry/dispatch, no observation mutation, no conflict “winner” selection.
