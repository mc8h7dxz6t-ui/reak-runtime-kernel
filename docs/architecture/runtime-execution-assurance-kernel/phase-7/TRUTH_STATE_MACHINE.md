# Truth conclusion lifecycle

1. **Input frozen:** reconciliation record + admissibility snapshot.  
2. **Derive:** deterministic mapping to `TruthConclusion`.  
3. **Append:** single immutable `TruthHistoryEntry::Conclusion`.  

## Establishment rules

| Reconciliation outcome | Admissible | Truth conclusion |
|------------------------|------------|------------------|
| `ReconciledSuccess` | yes | `EstablishedSuccess` |
| `ReconciledFailure` | yes | `EstablishedFailure` |
| Any non-establishing outcome | * | `NonEstablished` |
| Establishing outcome | no | `NonEstablished` (admissibility) |

Unknown reconciliation never becomes established success/failure.

## Forbidden

Re-dispatch, re-observe, re-reconcile, recovery execution, progression.
