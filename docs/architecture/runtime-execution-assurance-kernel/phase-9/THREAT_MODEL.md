# Phase 9 threat model

## Assets

- Append-only PRG durable stream (integrity via replay engine)  
- Uniqueness of progression per recovery lineage  
- Deterministic classification (audit / compliance replay)

## Threat actors

| Actor | Goal |
|-------|------|
| Malicious caller | Force dispatch or execution via progression API |
| Replay attacker | Re-submit same input to obtain conflicting states |
| Log tamperer | Alter prior progression records |
| Race attacker | Double-append progression for one recovery |

## Threats and mitigations

| ID | Threat | Mitigation |
|----|--------|------------|
| T1 | Set `authorizes_execution` true to bypass safety | Rejected with `IllegalExecutionFlag` before classify |
| T2 | Cross-wire truth/recovery/authorization | `BindingMismatch` |
| T3 | Second progression for same recovery | `DuplicateProgression` (recovery id + digest indexes) |
| T4 | Mutate truth/recovery in-place after fact | Out of scope: inputs are snapshots; progression does not write back |
| T5 | Tamper durable log | `verify()` / `recover_from_log` fail via replay hash chain |
| T6 | Concurrent duplicate determine | Serial mutex + duplicate checks under lock |
| T7 | Progression performs retry/compensate | No execution paths; states are labels only |

## Residual risk

- Host process memory not encrypted (standard REAK in-memory store).  
- Policy snapshot ref is not re-validated against a policy service (no providers in phase 9).  

## Out of scope

Network adversaries, adapter compromise, and dispatch-layer attacks (covered by frozen phases 4–8).
