# Phase 9 — Progression engine (IF-PRG-01)

Final runtime decision engine: classifies the next **legally admissible** step from immutable Truth and Recovery inputs. Progression never dispatches, observes, recovers, or mutates prior history.

| Document | Purpose |
|----------|---------|
| [IMPLEMENTATION_REPORT.md](./IMPLEMENTATION_REPORT.md) | Crate layout, API, invariants |
| [PROGRESSION_MODEL.md](./PROGRESSION_MODEL.md) | Inputs, outputs, digest, stream |
| [STATE_MACHINE.md](./STATE_MACHINE.md) | Strategy × authorization → state |
| [THREAT_MODEL.md](./THREAT_MODEL.md) | Abuse cases and mitigations |
| [SECURITY_REVIEW.md](./SECURITY_REVIEW.md) | Security posture |
| [HOSTILE_REPORT.md](./HOSTILE_REPORT.md) | Hostile integration tests |
| [PROPERTY_REPORT.md](./PROPERTY_REPORT.md) | Determinism property tests |
| [PERFORMANCE_REPORT.md](./PERFORMANCE_REPORT.md) | Classification throughput |
| [IMPLEMENTATION_MANIFEST.json](./IMPLEMENTATION_MANIFEST.json) | Machine-readable manifest |
| [REAK_PHASE9_GATE_PROPOSAL.json](./REAK_PHASE9_GATE_PROPOSAL.json) | Gate proposal |

**Crate:** `runtime-execution-assurance-kernel/crates/reak-progression`

**Verify:** `cd runtime-execution-assurance-kernel && cargo test -p reak-progression && cargo test --workspace`
