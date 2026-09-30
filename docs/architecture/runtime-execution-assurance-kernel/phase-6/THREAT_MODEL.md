# Phase 6 threat model

| ID | Threat | Mitigation |
|----|--------|------------|
| T-REC-01 | Forged alignment | Compare uses observation records only; mismatch emitted on divergence |
| T-REC-02 | Collapsing conflicts | Multiple classes → `ConflictingObservations` without picking a winner |
| T-REC-03 | Unknown → success inference | Explicit mismatch when unknown vs expected success/failure |
| T-REC-04 | Replay of mismatch emit | UES budget + append-only chain |
| T-REC-05 | Log tampering | `verify` / `recover_from_log` via replay engine |
| T-REC-06 | Resource exhaustion | UES fail-closed on `emit_mismatch` |
