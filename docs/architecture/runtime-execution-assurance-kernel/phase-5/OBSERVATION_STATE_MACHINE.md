# Observation lifecycle (IF-OBS-01)

Observations are **not** a truth state machine. There are no transitions between Success/Failure/Unknown inside this module.

## Lifecycle states (engine-internal)

| State | Meaning |
|-------|---------|
| `Validated` | `record()` succeeded; pending struct in caller memory |
| `Durable` | `append()` wrote hash-chained `ObservationHistoryEntry::Appended` |

## Rules

1. Classifications are **write-once** at append time from caller input.  
2. `ObservedUnknown` and `ObservationUnavailable` are first-class; never upgraded or downgraded here.  
3. Zero, one, or many observations per dispatch ticket id — never merged.  
4. Ordering authority is **append order** on the OBS stream; `observed_at_unix_ms` is recorded metadata only (delayed/reordered provider timestamps do not reorder the log).  
5. Recovery replays entries into indexes; does not reconcile or dispatch.

## Forbidden (enforced by absence of APIs)

No APIs for truth, retry, dispatch release, commitment mutation, conflict resolution, or recovery planning.
