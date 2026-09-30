# Phase 4 implementation report — Dispatch engine

## Mission

`reak-dispatch` implements IF-DSP-01: the **only** runtime module permitted to initiate irreversible external **effect attempts** (logical release only in this phase; no network or SDK).

## Crate layout

| Module | Responsibility |
|--------|----------------|
| `verify` | Pre-dispatch checks: commitment binding, status, authority, policy epoch, reservation id, UES snapshot sanity |
| `state_machine` | Explicit transition predicates and terminal outcome set |
| `engine` | `DispatchEngine`: tickets, indexes, DSP durable stream, `issue_ticket`, `execute_once`, `ack_or_nack`, `recover_from_log` |
| `model` | Immutable records: ticket, attempt, outcome, history entries |
| `error` | Fail-closed `DispatchError` taxonomy |

## Public API (IF-DSP-01)

- `issue_ticket` — full verification; no external effect  
- `execute_once` — at most one release per ticket; appends `Released`  
- `ack_or_nack` — immutable terminal outcome; verifies replay chain after outcome  

## Ownership boundaries (respected)

**Owns:** dispatch identity, state, history, timestamps, outcome class, attempt numbering, replay metadata on records, cancellation reason field, recovery metadata slot.

**Does not:** observe providers, reconcile, determine truth, run system recovery, execute replay beyond log verify, generate evidence reports, or qualify.

## Dependencies

Foundation + `reak-commitment` only (no new foundation interface changes in Phase 4).

## Test summary

15 integration tests across flow, hostile, recovery, state machine, and proptest suites. Workspace `cargo test --workspace` green.

## Architectural note

`Released` is persisted in the durable log; the in-memory ticket jumps from `TicketIssued` to `AwaitingAck` within a single `execute_once` call. This matches “exactly one release” without exposing a re-entrant `Released` API surface.
