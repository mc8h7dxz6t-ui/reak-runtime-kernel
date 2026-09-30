# Module admission report

**Permanent rule (post–Phase 1):** see [REAK_ENGINEERING_GOVERNANCE.md](../REAK_ENGINEERING_GOVERNANCE.md#module-admission-rule-permanent). New modules use: constitutional owner, runtime responsibility, external justification, qualification strategy, security review, removal analysis.

Phase 1 candidates below were scored against the Phase 1 exercise (including roadmap/commercial columns). Grandfathered admissions remain valid; new work must follow governance.

**Verdict enums:** ADMIT | ADMIT_WITH_LIMITATION | REJECT (stay outside kernel)

## Admitted modules (18)

| Module | Verdict | Constitutional | Roadmap | External (EVP/PRB) | Commercial | Qualification | Removal analysis |
|--------|---------|----------------|---------|----------------------|------------|---------------|------------------|
| reak-durable-record | ADMIT | G4 invariant | Foundation | Audit/replay demand (E,F) | Regulated append-only audit | IQ-DR-01 | If host stores authoritative log — kernel shrinks to semantics layer only |
| reak-policy-context | ADMIT | G5 | Foundation | Reproducibility disputes (D,E) | Contract dispute reduction | IQ-POL-01 | Vendor policy engines own static policy; runtime epoch still required for **decision** replay |
| reak-uncertainty-ledger | ADMIT | Art V | Foundation | Overconfidence incidents (E) | Fail-closed culture (A5) | IQ-UES-01 | Could merge into planes — rejected: cross-stage budget enforcement needs owner |
| reak-registry | ADMIT | III-12 | Foundation | Lineage traceability (G1) | Audit pointer stability | IQ-REG-01 | CMDB/artifact repos own storage; kernel keeps **index semantics** |
| reak-authority | ADMIT | III-1 | R4 prereq | IAM adjacent, not sufficient (B) | Scoped permission product gap | IQ-AUTH-01 | IdP owns identity; kernel owns **attempt permission** for consequence class |
| reak-exposure | ADMIT | III-2 | R4 prereq | Blast radius (E) | Risk ceiling contracts | IQ-EXP-01 | Rate limiters replace only numeric ceiling, not reservation semantics |
| reak-boundary | ADMIT | III-3 | R6 | Untrusted providers (A9) | Adapter safety | IQ-BND-01 | API gateway owns transport; kernel owns **semantic** normalization |
| reak-commitment | ADMIT | III-10, G1 | **R4** | Bind-before-effect gap (B,I) | PRB kill-stack core | IQ-CMT-01 | Obsolete if normative commit API + independent verify ships platform-wide |
| reak-dispatch | ADMIT | III-11, G2 | **R5** | Duplicate effect failures (E) | Single releaser | IQ-DSP-01 | Workflow engines retain orchestration; dispatch module may thin to ticket verifier |
| reak-observation | ADMIT | III-4 | R6 | Telemetry ≠ outcome (B) | Independent sense path | IQ-OBS-01 | Observability vendors own metrics; not canonical sense records |
| reak-reconciliation | ADMIT | III-5 | R7 | Payment/ops mismatch (E) | Operational assurance | IQ-REC-01 | Reconciliation in data pipelines insufficient for **consequence** compare |
| reak-truth | ADMIT | III-6, G3 | R1/R7 | Epistemic vs structural (D) | Audit defensibility | IQ-TRU-01 | BI/analytics tools — not constitutional truth owner |
| reak-evidence-hooks | ADMIT_WITH_LIMITATION | III-7 | R12 hooks | Independent evidence (I) | Compliance evidence | IQ-EVD-HOOK-01 | Full catalogue in EvidenceLab; hooks only in kernel |
| reak-recovery | ADMIT | III-8 | R8 | Saga/retry confusion (D) | Controlled recovery | IQ-RCV-01 | Orchestrators own retry mechanics; kernel owns **intent** |
| reak-progression | ADMIT_WITH_LIMITATION | III-9, G3 | R2/R3 | Proceed ≠ execute (I) | Safe hold default | IQ-PRG-01 | Policy engine static allow — does not replace runtime class |
| reak-replay | ADMIT | III-13 | Foundation | Forensics (E) | Incident recon | IQ-RPL-01 | Log platforms store bytes; kernel owns **verify/reconstruct semantics** |
| reak-lifecycle-shell | ADMIT | I, II | R3/R9 | Separation of powers (I) | Integration wedge | IQ-ASM-01 | Monolith hosts may inline wiring — must re-prove non-collapse |
| reak-integration-host | ADMIT | II | R14 | Temporal/StepFns adjacency (B,C) | Embed in existing stacks | IQ-HOST-* | If all hosts native commit+fence — adapters shrink |

**Admission rate:** 18 admitted / ~25 historical Consequence II top-level concepts ≈ **72%** — intentional reduction.

## Rejected from kernel (remain outside)

| Candidate | Verdict | Reason |
|-----------|---------|--------|
| CCS harness / mutants / fuzz | REJECT | Roadmap R10–11 → Qualification Kernel |
| IQ runners, HRT orchestration | REJECT | Qualification governance; not runtime |
| EvidenceLab reports / certificates | REJECT | R12; consumes IF-EXPORT-01 |
| `canonical_*` crate layout | REJECT | Naming not justified; semantics admitted under `reak-*` |
| ext-001 as dispatch owner | REJECT | R0: adapter not Art III-11 owner |
| Passport / Witness / Knowledge | REJECT | Constitution Part 2 — optional / research |
| Break-glass runtime module | REJECT | Art VI ops path |
| AI agent plane | REJECT | Constitution exclusion |
| Programme dependency taxonomy engine | REJECT | Qualification programme metadata, not runtime |
| Research protocols (HIBS, blind campaigns) | REJECT | Research Sandbox |
| Merkle/Kafka/Raft as modules | REJECT | Part 6 exclusions — implementation choices |

## Cross-cutting decisions

1. **Evidence plane split** — Runtime keeps admissibility hooks; EvidenceLab owns catalogue volume and report generation (C-05 alignment).
2. **SafeNext naming dropped** — Progression responsibility admitted; name is implementation (Part 2).
3. **Assembly manifest** — R3 remains reference evidence for IQ-ASM-01, not structural mandate for crate names.
4. **Qualification inside repo** — Forbidden; REAK repo contains zero test harness beyond minimal smoke (deferred to Phase 2 policy).

## Freeze recommendation

Freeze admitted module set at **18 modules** for Phase 2 implementation unless constitutional amendment or R0 matrix change.
