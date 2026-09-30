# Phase 4 performance report

**Scope:** In-process dispatch governance (no network).

## Design choices

| Goal | Approach |
|------|----------|
| Low allocation | Reuse `String` keys; clone tickets only on API boundaries; JSON payload append per history event |
| Deterministic latency | Single `dispatch_serial` mutex — predictable ordering, no lock convoys across tenants in this in-memory tenant-scoped engine |
| Bounded memory | Maps keyed by ticket / commitment / operation; no unbounded retry queues |
| Lock minimisation | `RwLock` for read-mostly indexes; mutex only on mutating API entry |

## Measurements

No micro-benchmark suite in Phase 4 (Qualification Kernel may add HRT later). Structural bounds: O(1) index lookups per operation; O(n) recovery over stream length n.

## Follow-up

Persistent DSP backend and multi-tenant sharding are adapter/storage concerns; engine API already separates durable append from verification.
