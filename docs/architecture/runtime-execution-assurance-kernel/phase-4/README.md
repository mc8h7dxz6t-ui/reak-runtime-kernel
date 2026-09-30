# REAK Phase 4 — Dispatch engine

**Status:** Implementation complete — gate **PROPOSED**  
**Contract:** IF-DSP-01 (`reak-dispatch`)

## Scope

Sole runtime owner of outbound dispatch **process** (ticket, verify, single release, terminal outcome). No adapters, provider I/O, observation, reconciliation, truth, recovery execution, or qualification.

## Evidence

| Document | Purpose |
|----------|---------|
| [IMPLEMENTATION_REPORT.md](./IMPLEMENTATION_REPORT.md) | Overview |
| [DISPATCH_STATE_MACHINE.md](./DISPATCH_STATE_MACHINE.md) | Legal transitions |
| [THREAT_MODEL.md](./THREAT_MODEL.md) | Dispatch-specific threats |
| [SECURITY_REVIEW.md](./SECURITY_REVIEW.md) | Reviews |
| [HOSTILE_TEST_REPORT.md](./HOSTILE_TEST_REPORT.md) | Hostile tests |
| [PROPERTY_TEST_REPORT.md](./PROPERTY_TEST_REPORT.md) | Proptest |
| [PERFORMANCE_REPORT.md](./PERFORMANCE_REPORT.md) | Performance |
| [IMPLEMENTATION_MANIFEST.json](./IMPLEMENTATION_MANIFEST.json) | Manifest |
| [REAK_PHASE4_GATE_PROPOSAL.json](./REAK_PHASE4_GATE_PROPOSAL.json) | Gate proposal |

**Verify:** `cd runtime-execution-assurance-kernel && cargo test -p reak-dispatch && cargo test --workspace`
