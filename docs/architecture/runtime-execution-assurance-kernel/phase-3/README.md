# REAK Phase 3 — Commitment engine

**Status:** **ACCEPTED / FROZEN** (bug fixes only)  
**Contract:** IF-CMT-01 (`reak-commitment`)

## Scope

Runtime commitment engine only. No dispatch, observation, reconciliation, adapters, or persistence backends.

## Evidence

| Document | Purpose |
|----------|---------|
| [IMPLEMENTATION_REPORT.md](./IMPLEMENTATION_REPORT.md) | Overview |
| [INTERNAL_ARCHITECTURE.md](./INTERNAL_ARCHITECTURE.md) | State machine + data flow |
| [THREAT_MODEL.md](./THREAT_MODEL.md) | Commitment-specific threats |
| [SECURITY_REVIEW.md](./SECURITY_REVIEW.md) | Reviews |
| [HOSTILE_TEST_REPORT.md](./HOSTILE_TEST_REPORT.md) | Hostile tests |
| [PROPERTY_TEST_REPORT.md](./PROPERTY_TEST_REPORT.md) | Proptest |
| [PERFORMANCE_NOTES.md](./PERFORMANCE_NOTES.md) | Performance |
| [FOUNDATION_TOUCH_RECORD.md](./FOUNDATION_TOUCH_RECORD.md) | Allowed foundation delta |
| [IMPLEMENTATION_MANIFEST.json](./IMPLEMENTATION_MANIFEST.json) | Manifest |
| [REAK_PHASE3_GATE_PROPOSAL.json](./REAK_PHASE3_GATE_PROPOSAL.json) | Gate proposal |

**Verify:** `cd runtime-execution-assurance-kernel && cargo test -p reak-commitment`
