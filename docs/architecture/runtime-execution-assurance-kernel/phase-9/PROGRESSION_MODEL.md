# Progression model (IF-PRG-01)

## Purpose

Progression is the **terminal classifier** of the REAK runtime spine. It answers: given frozen truth and recovery artifacts, what is the next **admissible** operational class—without performing that operation.

## Inputs (immutable)

| Field | Type | Constraint |
|-------|------|------------|
| `truth` | `TruthRecord` | Non-empty `truth_id`; drives `NoAction` → `Terminal` vs `NoFurtherAction` |
| `recovery` | `RecoveryRecord` | Must reference same `truth_id`; `authorizes_execution` must be **false** |
| `authorization` | `RecoveryIntentAuthorization` | Must bind `recovery_id` + `truth_id`; `authorizes_execution` **false** |
| `policy` | `PolicySnapshotRef` | Non-empty `epoch_id`; non-zero `content_hash` |

Binding rule: `recovery.truth_id == truth.truth_id`, `authorization` matches both ids.

## Outputs (append-only)

`ProgressionRecord`:

- `progression_id`, `truth_id`, `recovery_id`, `dispatch_ticket_id`  
- `state: ProgressionState`  
- `input_digest_hex` — deterministic replay key  
- `replay` — durable sequence + hash chain metadata  

History is a single variant: `ProgressionHistoryEntry::Determined(ProgressionRecord)`.

## Progression states

| State | Meaning (intent class only) |
|-------|-----------------------------|
| `Terminal` | Success concluded; no further runtime progression for this lineage |
| `Await` | Strategy or authorization not yet sufficient to permit action |
| `RetryPermitted` | Policy allows retry class; execution happens outside progression |
| `CompensationPermitted` | Compensation class permitted |
| `HumanApprovalRequired` | Escalation with authorization maps here |
| `Abort` | Recovery strategy abort |
| `NoFurtherAction` | NoAction with non-success truth |

## Determinism

For fixed `ProgressionInput`, `classify_next_state` and `input_digest_hex` are pure functions. Replaying the durable log via `recover_from_log` reproduces the same indexes and record set.

## Non-goals

- No upgrade of truth conclusions  
- No change to recovery strategy or authorization flags  
- No dispatch ticket issuance or execution  
