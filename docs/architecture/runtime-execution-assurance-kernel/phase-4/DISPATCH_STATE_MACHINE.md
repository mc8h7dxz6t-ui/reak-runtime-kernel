# Dispatch state machine (IF-DSP-01)

Immutable lifecycle for one dispatch ticket. No implicit transitions. Terminal outcomes are recorded once and never permit a second external release for the same commitment.

## States

| State | Meaning |
|-------|---------|
| `TicketIssued` | Pre-dispatch verification succeeded; durable `TicketIssued` entry appended; no outbound release yet |
| `Released` | Transient logical state: `Released` history entry appended (outbound release authorised); not held in the in-memory ticket between `execute_once` steps |
| `AwaitingAck` | Exactly one release recorded; awaiting immutable terminal outcome classification |

## Terminal outcomes (immutable)

`Success`, `Unknown`, `Failed`, `Cancelled`, `Timeout` — each recorded as a durable `Outcome` entry. After any terminal outcome, `execute_once` and second `ack_or_nack` are rejected (`AlreadyTerminal` / `IllegalTransition`).

**UNKNOWN rule:** `Unknown` does **not** authorise another `execute_once` or automatic retry.

## Legal transitions

| From | To | API / event |
|------|-----|-------------|
| — | `TicketIssued` | `issue_ticket` after `verify_pre_dispatch` |
| `TicketIssued` | `Released` | `execute_once` (durable `Released` append) |
| `Released` | `AwaitingAck` | `execute_once` completes (in-memory ticket update) |
| `AwaitingAck` | `AwaitingAck` | Idempotent no-op in `can_transition` (no second release) |
| `AwaitingAck` | terminal | `ack_or_nack` with terminal `DispatchOutcome` |

## Illegal transitions (fail closed)

| Attempt | Error |
|---------|-------|
| Second `issue_ticket` same commitment / operation | `DuplicateDispatch` |
| `execute_once` when not `TicketIssued` | `IllegalTransition` |
| `execute_once` after release / terminal | `IllegalTransition` / `AlreadyTerminal` |
| `ack_or_nack` before release | `IllegalTransition` |
| Second `ack_or_nack` | `AlreadyTerminal` |
| Skip `Released` in durable log | Prevented by ordered append in `execute_once` |

## Recovery projection

`recover_from_log` replays the DSP durable stream in order:

1. `TicketIssued` → rebuild ticket map and uniqueness indexes  
2. `Released` → set ticket state to `AwaitingAck`  
3. `Outcome` → populate terminal map  

Crash **before** send: only `TicketIssued` in log → recovery leaves ticket dispatchable once.  
Crash **after** send: `Released` present → recovery sets `AwaitingAck` → second `execute_once` rejected.

## Code reference

Predicate helpers: `reak-dispatch::state_machine::{can_transition, is_terminal_outcome, TERMINAL_OUTCOMES}`.
