# Phase 4 threat model — Dispatch engine

**Asset:** Dispatch ticket + durable DSP stream (authoritative record of release attempts and outcomes)  
**Trust zone:** Kernel runtime; callers hold references to commitment/exposure/policy/UES engines  
**Adversary:** Malicious or buggy caller, concurrent threads, replay of stale commitment material

## Threat scenarios

| ID | Threat | Mitigation |
|----|--------|------------|
| T-DSP-01 | **Replay attack** — re-issue dispatch with old commitment digest | `verify_binding` + digest in `DispatchSpec`; commitment replay inside CMT |
| T-DSP-02 | **Duplicate send** — two tickets or two releases per commitment | `by_commitment` / `by_operation` indexes; single `execute_once` per ticket; serial mutex on mutating paths |
| T-DSP-03 | **Authority bypass** — dispatch after revoke | `verify_scope` at `issue_ticket` via `exposure.authority()` |
| T-DSP-04 | **Policy drift** — epoch/hash mismatch | `resolve_epoch` with content hash from commitment record |
| T-DSP-05 | **Concurrent issue race** | `dispatch_serial` mutex; at most one winning `issue_ticket` |
| T-DSP-06 | **UNKNOWN retry** — treat unknown as retryable | Terminal `Unknown` blocks further `execute_once` (tested) |
| T-DSP-07 | **Log tampering** | Hash-chained durable records; `ReplayEngine::verify_history` on outcome |
| T-DSP-08 | **State corruption** | Illegal transitions return errors; recovery rebuilds from verified log only |
| T-DSP-09 | **Resource exhaustion** | In-memory maps bounded by issued tickets; no unbounded retry loops in engine |

## Out of scope (future layers)

Provider impersonation, wire-level MITM, and adapter credential theft — addressed when integration host exists, not in Phase 4.

## Residual risk

Crash between durable append and in-memory update could leave log ahead of indexes until `recover_from_log` runs; operators must replay recovery on startup (documented in state machine).
