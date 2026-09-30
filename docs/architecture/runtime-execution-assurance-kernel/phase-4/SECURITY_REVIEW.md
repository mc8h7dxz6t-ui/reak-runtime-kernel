# Phase 4 security review — Dispatch engine

## Privilege review

Only `DispatchEngine` exposes `execute_once`. No other `reak-*` crate in this workspace performs dispatch. Commitment and foundation crates do not call dispatch APIs.

## Replay attack review

Binding verification requires live commitment digest and id. Stale digests fail at CMT layer. DSP stream append is sequential; recovery refuses inconsistent chains.

## Duplicate-send analysis

Uniqueness enforced at ticket issuance (commitment + operation lineage:generation). Second `execute_once` on same ticket rejected after first release. Concurrent `issue_ticket` stress test: exactly one success.

## Concurrency review

`parking_lot::Mutex` serialises `issue_ticket`, `execute_once`, and `ack_or_nack`. Read-heavy paths use `RwLock` for maps; mutations hold serial guard first on write paths.

## State corruption review

No silent coercion of illegal states. Terminal map consulted before mutating operations. Recovery deserialises typed history entries only.

## Resource exhaustion review

No retry storm from UNKNOWN. Ticket maps grow O(issued tickets). Durable store is in-memory for Phase 4 (same pattern as Phase 2–3).

## Verdict

**No architectural contradiction** with Phase 1 spine (commitment → dispatch). Fail-closed behaviour consistent with frozen architecture. Gate security criteria satisfied for Phase 4 scope.
