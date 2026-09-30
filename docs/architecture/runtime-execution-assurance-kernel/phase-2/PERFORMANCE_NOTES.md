# Performance notes — Phase 2

**Status:** Design intent + asymptotic behaviour; no benchmark gate in Phase 2.

## Targets (engineering)

- Millions of appends per stream over process lifetime (memory-backed in Phase 2; disk in later phases).
- O(1) append amortized per stream (Vec push + hash).
- O(n) verify chain for n records (full scan; batch verify API future).
- Minimal allocations: reuse `Vec` in envelope where callers own buffers; hash over payload reference.

## Locking

- One `RwLock` per store map; contention across tenants/streams is shardable in persistence adapter by `(tenant, stream_id)`.

## Not optimized (intentionally)

- JSON serialization for audit payloads — clarity over wire format; binary schema Phase 3+.
- No object pooling — premature for foundation.

## Correctness precedence

No optimization applied that skips validation or chain computation.
