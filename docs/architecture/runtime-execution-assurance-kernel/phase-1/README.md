# Runtime Execution Assurance Kernel (REAK) — Phase 1

**Status:** Architecture extraction and admission (no implementation).  
**Authority:** Programme Constitution (FINAL_ARCHITECTURAL_CONSTITUTION_REVIEW.md), R0 traceability matrix, Implementation & Qualification Roadmap v1.0, CCS v1.0, EvidenceLab Core Constitution (boundary only), IQ-007/008/009, R3 assembly manifest, R4 commitment definition, EVP v1.0, PRB v1.0.

**Reference implementation (not source of truth):** Consequence II / `axiom-consequence-ii`.

## Deliverables

| # | Document | Purpose |
|---|----------|---------|
| 1 | [KERNEL_ARCHITECTURE.md](./KERNEL_ARCHITECTURE.md) | Boundaries, responsibilities, integration posture |
| 2 | [KERNEL_MODULE_GRAPH.md](./KERNEL_MODULE_GRAPH.md) + [KERNEL_MODULE_GRAPH.json](./KERNEL_MODULE_GRAPH.json) | Admitted modules and seams |
| 3 | [INTERFACE_CONTRACTS.md](./INTERFACE_CONTRACTS.md) + [INTERFACE_CONTRACTS.json](./INTERFACE_CONTRACTS.json) | Versioned contracts |
| 4 | [DEPENDENCY_GRAPH.json](./DEPENDENCY_GRAPH.json) | Acyclic module dependencies |
| 5 | [TRUST_BOUNDARY_MAP.md](./TRUST_BOUNDARY_MAP.md) | Trust and privilege zones |
| 6 | [THREAT_MODEL.md](./THREAT_MODEL.md) | Hostile-environment assumptions |
| 7 | [QUALIFICATION_MATRIX.json](./QUALIFICATION_MATRIX.json) | Per-module qualification path and claim ceilings |
| 8 | [MODULE_ADMISSION_REPORT.md](./MODULE_ADMISSION_REPORT.md) | Admission / rejection with six-criteria analysis |
| 9 | [HISTORICAL_IMPORT_REPORT.md](./HISTORICAL_IMPORT_REPORT.md) | What may be copied from Consequence II and how |
| 10 | [BUILD_ORDER.md](./BUILD_ORDER.md) | Phased implementation aligned to R4–R17 |

**Programme separation:** [PROGRAMME_TRIAD_BOUNDARIES.md](./PROGRAMME_TRIAD_BOUNDARIES.md)

**Phase gate:** [REAK_PHASE1_GATE.json](./REAK_PHASE1_GATE.json)

## Three programmes

| Programme | Repository (target) | Owns |
|-----------|---------------------|------|
| **REAK** | `runtime-execution-assurance-kernel` | Production runtime planes and host integration only |
| **Qualification Kernel** | `qualification-kernel` | CCS execution, hostile/mutation/fuzz/chaos, IQ harness |
| **EvidenceLab** | `evidencelab` | Reports, certificates, claim ceilings publication, verification |

**Preservation:** Legacy Archive (frozen Consequence II), Research Sandbox (unrestricted), Production Kernel (this programme).

## Phase 1 exit

Phase 1 is complete when `REAK_PHASE1_GATE.json` records `ARCHITECTURE_FROZEN` and all ten deliverables are present. Implementation begins only after admission freeze.
