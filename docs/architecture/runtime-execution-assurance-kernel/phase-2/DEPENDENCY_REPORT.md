# Dependency report — Phase 2

## Workspace crate graph (foundation only)

```
reak-types
reak-durable-record → reak-types
reak-policy-context → reak-durable-record, reak-types
reak-ues → reak-durable-record, reak-types
reak-registry → reak-durable-record, reak-types
reak-replay → reak-durable-record, reak-registry, reak-types
reak-authority → reak-policy-context, reak-durable-record, reak-types
reak-exposure → reak-authority, reak-durable-record, reak-types
```

Aligned with Phase 1 `DEPENDENCY_GRAPH.json` foundation edges. No cycles. No commitment/dispatch crates.

## External crates

| Crate | Justification |
|-------|----------------|
| serde / serde_json | Validated serialization for envelopes and audit payloads |
| sha2 | Tamper-evident hash chain (IF-DR-01) |
| thiserror | Explicit error taxonomy |
| parking_lot | Lower-overhead RwLock for concurrent append paths |
| proptest | Property tests (dev) |
| hex | Test helpers (dev, durable-record) |

No HTTP, async executor, or ORM dependencies.

## Lint

Run `cargo tree -p reak-exposure` before adding dependencies; new crates require governance admission.
