# REAK Phase 6 — Reconciliation engine

**Status:** Implementation complete — gate **PROPOSED**  
**Contract:** IF-REC-01 (`reak-reconciliation`)

## Scope

Compare caller-supplied **expected consequence** to **observation records**; emit durable **mismatch** records only. No truth, dispatch, recovery planning, or conflict resolution.

## Evidence

| Document | Purpose |
|----------|---------|
| [IMPLEMENTATION_REPORT.md](./IMPLEMENTATION_REPORT.md) | Overview |
| [RECONCILIATION_STATE_MACHINE.md](./RECONCILIATION_STATE_MACHINE.md) | Compare / emit lifecycle |
| [THREAT_MODEL.md](./THREAT_MODEL.md) | Threats |
| [SECURITY_REVIEW.md](./SECURITY_REVIEW.md) | Review |
| [HOSTILE_TEST_REPORT.md](./HOSTILE_TEST_REPORT.md) | Hostile tests |
| [PROPERTY_TEST_REPORT.md](./PROPERTY_TEST_REPORT.md) | Proptest |
| [PERFORMANCE_NOTES.md](./PERFORMANCE_NOTES.md) | Performance |
| [IMPLEMENTATION_MANIFEST.json](./IMPLEMENTATION_MANIFEST.json) | Manifest |
| [REAK_PHASE6_GATE_PROPOSAL.json](./REAK_PHASE6_GATE_PROPOSAL.json) | Gate proposal |

**Verify:** `cd runtime-execution-assurance-kernel && cargo test -p reak-reconciliation && cargo test --workspace`
