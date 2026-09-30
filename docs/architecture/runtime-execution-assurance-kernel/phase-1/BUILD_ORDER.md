# Build order

Aligned to Implementation & Qualification Roadmap v1.0 (R4–R17). Phase 2+ implementation only after `REAK_PHASE1_GATE.json` freeze.

## Phase 2 — Foundation (no external effects)

| Step | Module | Roadmap | Exit |
|------|--------|---------|------|
| 2.1 | reak-durable-record | Pre-R3 | IQ-DR-01 draft in Qualification Kernel |
| 2.2 | reak-policy-context | Pre-R3 | Epoch pin tests |
| 2.3 | reak-uncertainty-ledger | Pre-R3 | UES parity |
| 2.4 | reak-registry | Foundation | IQ-REG-01 |
| 2.5 | reak-replay | Foundation | IQ-RPL-01 |
| 2.6 | reak-authority + reak-exposure | R4 prereq | Scoped grant + reservation |

**Gate:** No IF-DSP-01 surface exposed to hosts.

## Phase 3 — R4 Commitment

| Step | Module | Exit |
|------|--------|------|
| 3.1 | reak-boundary (minimal ingress for bind path) | IQ-BND-01 subset |
| 3.2 | reak-commitment | IQ-CMT-01; HQ-CMT plan |
| 3.3 | reak-lifecycle-shell (bind path only) | No dispatch |

**Roadmap exit R4:** No external effect without commitment record.

## Phase 4 — R5 Dispatch

| Step | Module | Exit |
|------|--------|------|
| 4.1 | reak-dispatch | IQ-DSP-01 |
| 4.2 | reak-integration-host (single adapter, e.g. HTTP) | IQ-HOST-http |
| 4.3 | Shell wires CMT → DSP | Duplicate dispatch tests |

**Roadmap exit R5:** Single releaser per generation.

## Phase 5 — Sense spine (R6–R7)

| Step | Module | Exit |
|------|--------|------|
| 5.1 | reak-observation (production path) | Closes IQ-005 gap |
| 5.2 | reak-reconciliation | Closes IQ-006 gap |
| 5.3 | reak-truth (port + IQ-TRU-01) | Parity iq-007 |
| 5.4 | reak-evidence-hooks | Admissibility machine checks |

**Roadmap exit R7:** Observation → Reconciliation → Truth on production path.

## Phase 6 — Recovery & progression (R8, R2 carryover)

| Step | Module | Exit |
|------|--------|------|
| 6.1 | reak-recovery on real dispatch path | R8 requal |
| 6.2 | reak-progression | IQ-PRG-01 with iq-009 limitations |
| 6.3 | Full shell assembly 005–009 equivalent | IQ-ASM-01 vs R3 manifest |

## Phase 7 — Parallel programmes (do not block kernel merge)

| Programme | Roadmap | Dependency on REAK |
|-----------|---------|-------------------|
| Qualification Kernel | R10 → R11 | IF-EXPORT-01, public APIs only |
| EvidenceLab | R12 | IF-EXPORT-01 |
| Host adapters (additional) | R14 | IF-HOST-01 per target |

## Phase 8 — Hostile & independent qualification

| Step | Roadmap | Exit |
|------|---------|------|
| 8.1 | R9 HRT on full REAK assembly | Residual assumptions doc |
| 8.2 | R11 CCS via Qualification Kernel | Conformance report |
| 8.3 | R13 independent CCS | Signed statement |
| 8.4 | R14 per-provider | Provider matrix |

## Phase 9 — Operational progression

R15 shadow → R16 supervised pilot → R17 production (roadmap unchanged).

## Critical path

```
Foundation → R4 Commitment → R5 Dispatch → R6/R7 Sense spine → R8 Recovery → R3-class Assembly → R9 HRT
```

CCS (R10–11) and EvidenceLab (R12) run in parallel once IF-EXPORT-01 exists (after Phase 5 minimum).

## Repository bootstrap (Phase 2.0)

1. Create `runtime-execution-assurance-kernel` repo with `legacy/` pointer to frozen archive.
2. Create empty `qualification-kernel` and `evidencelab` repos with contract stubs referencing this Phase 1 package.
3. CI: dependency graph acyclic check + forbidden edge lint.
