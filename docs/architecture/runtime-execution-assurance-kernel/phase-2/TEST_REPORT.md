# Test report — Phase 2

**Command:** `cargo test --workspace`  
**Date:** 2026-09-30  
**Result:** All tests passed.

## Inventory

| Crate | Tests |
|-------|-------|
| reak-durable-record | 1 unit, 3 hostile, 1 proptest |
| reak-policy-context | 2 |
| reak-ues | 2 hostile |
| reak-registry | 2 |
| reak-replay | 2 |
| reak-authority | 2 |
| reak-exposure | 2 |

**Total:** 16 automated tests (excluding doc-tests).

## Coverage themes

- Normal paths: grant, reserve, register, append, pin epoch
- Invalid inputs: oversize payload, empty rules, zero units
- Hostile: concurrent append, budget overrun, ceiling breach, hash mismatch
- Boundary: cursor read, empty replay stream
- Property: durable append + verify roundtrip (proptest)

## Regression

Workspace CI entrypoint: `cargo test --workspace`. Future: forbid-edge script against Phase 1 graph.
