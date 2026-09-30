# REAK kernel architecture (Phase 1)

## Purpose

The Runtime Execution Assurance Kernel governs **one consequential operation lifecycle** embedded in an existing orchestrator. It is **not** a workflow engine, agent framework, or observability product. It enforces constitutional separation between permission, binding, sensing, comparing, concluding, progressing, and **sole** external effect release.

**Sources of truth**

| Layer | Source |
|-------|--------|
| Responsibilities | Programme Constitution (Articles I–VII) |
| Owners and CCS mapping | R0 Constitutional Traceability Matrix |
| Build sequence | Implementation & Qualification Roadmap v1.0 (R4–R17) |
| Commercial falsification | EVP v1.0 (external evidence only) |
| Continue/pivot | PRB v1.0 kill stack: authority → commitment → dispatch → observation → reconciliation → progression |

## Architectural centre

**Operation lineage** is the unit of coherence: a stable identifier for an attempted consequence class, with monotonic generation for retries, bound policy epoch, and append-only authoritative history.

```
Authority + Exposure
        ↓
    Boundary (ingress)
        ↓
   Commitment (bind-before-effect) ───┐
        ↓                              │ G1 traceability
   Dispatch (sole effect) ─────────────┘
        ↓
   Observation → Reconciliation → Truth
        ↓
   Recovery (intent only) ↔ Progression (class only)
        ↓
   Registry / Replay (non-normative read paths)
```

Recovery and Progression **never** invoke external effects. Dispatch **never** runs without a valid Commitment record. Truth **never** ingests raw provider payloads without Observation and admissibility path.

## Minimum runtime responsibilities (R0 + Roadmap)

Mapped from constitution to **reak-** modules (names are greenfield; semantics are frozen).

| Constitution | Roadmap stage | REAK module | Primary interface |
|--------------|---------------|-------------|-------------------|
| Art III.1 Authority | Pre-R4 (foundation) | `reak-authority` | `IF-AUTH-01` |
| Art III.2 Exposure | Pre-R4 | `reak-exposure` | `IF-EXP-01` |
| Art III.3 Boundary | R6 path (ingress) | `reak-boundary` | `IF-BND-01` |
| Art III.10 Commitment | **R4** | `reak-commitment` | `IF-CMT-01` |
| Art III.11 Dispatch | **R5** | `reak-dispatch` | `IF-DSP-01` |
| Art III.4 Observation | R6 | `reak-observation` | `IF-OBS-01` |
| Art III.5 Reconciliation | R7 | `reak-reconciliation` | `IF-REC-01` |
| Art III.6 Truth | R1 admitted (IQ-007) | `reak-truth` | `IF-TRU-01` |
| Art III.7 Evidence (admissibility) | R12 touchpoints | `reak-evidence-hooks` | `IF-EVD-01` |
| Art III.8 Recovery | R8 requal on path | `reak-recovery` | `IF-RCV-01` |
| Art III.9 Progression | R2 IQ-009 | `reak-progression` | `IF-PRG-01` |
| Art III.12 Registry | Foundation | `reak-registry` | `IF-REG-01` |
| Art III.13 Replay | Foundation | `reak-replay` | `IF-RPL-01` |
| Art IV G4 Append-only | Cross-cutting | `reak-durable-record` | `IF-DR-01` |
| Art IV G5 Reproducibility | Cross-cutting | `reak-policy-context` | `IF-POL-01` |
| Art V Uncertainty | Cross-cutting | `reak-uncertainty-ledger` | `IF-UES-01` |
| Host embedding | Integration-first | `reak-integration-host` | `IF-HOST-01` |
| Lifecycle wiring | R3 assembly pattern | `reak-lifecycle-shell` | `IF-SHELL-01` |

**Not runtime modules:** Qualification governance (Art IV G6 process), break-glass (Art VI ops), amendment (Art VII governance).

## Integration-first posture

| Host | REAK role | REAK does **not** |
|------|-----------|-------------------|
| Temporal | Activity/side-effect gate before external call; commitment ticket as workflow payload | Replace Temporal state machine |
| AWS Step Functions | Task token + dispatch ticket binding | Own step graph |
| Microsoft Agent Gateway | Tool invocation fence after commitment | Route agents |
| Kubernetes | Sidecar or admission hook exporting records | Schedule pods |
| MCP hosts | Capability call wrapper at boundary | Implement MCP server registry |
| HTTP APIs | Synchronous bind-then-dispatch handler | General API gateway |

All hosts receive the same **normative contracts** (`IF-HOST-01`, `IF-CMT-01`, `IF-DSP-01`).

## Security-first design (architecture level)

- **Trust boundaries:** external provider, host orchestrator, REAK privileged core, evidence export zone (read-only).
- **Tenant isolation:** lineage namespace + policy epoch scoped per tenant; no shared mutable progression state without explicit reservation (Exposure).
- **Replay protection:** dispatch tickets single-use per generation; commitment digest binds intent hash.
- **Failure containment:** default non-proceed (Art IV G3); plane errors surface as recorded classes, not silent retries.
- **Secure defaults:** deny dispatch without commitment; deny Established truth without admissible evidence path.
- **Least privilege:** each module receives only the record types and capabilities listed in TRUST_BOUNDARY_MAP.

## Production engineering (design constraints for implementation phases)

| Concern | Architectural response |
|---------|-------------------------|
| Crash recovery | Commitment and dispatch records durable before ACK to host; idempotent host callbacks keyed by lineage generation |
| Horizontal scaling | Single-writer per lineage generation; shard by lineage id (implementation choice) |
| Determinism where claimed | Policy epoch + ordered durable inputs; reducers declare deterministic subsets in QUALIFICATION_MATRIX |
| Bounded uncertainty | UES ledger enforced at plane boundaries |
| Observability | Export audit stream from durable records, not from side-channel metrics as truth |
| Upgrade compatibility | Interface major versions; policy epoch bumps explicit |
| Backwards compatibility | Record schema versioning with supersede records (G4) |

## External alignment (summary)

| Module cluster | EVP / market theme | Adjacent vendor ownership | Obsolescence trigger |
|----------------|-------------------|---------------------------|----------------------|
| Commitment + Dispatch | Bind-before-effect, single releaser | Workflow engines own orchestration, not liability binding | Hyperscaler ships normative commit+fence API with independent audit |
| Observation + Reconciliation + Truth | Execution vs outcome assurance | Observability vendors own telemetry, not epistemic truth | Regulators accept provider-only logs for consequential audit (EVP falsification) |
| Progression | Non-proceed default | Policy engines (OPA/Cedar) own static allow, not runtime class | — |
| Qualification (outside kernel) | Hostile conformance | Internal QA only | Enterprises mandate third-party qualification (supports separate Qualification Kernel) |

Detailed per-module rows: MODULE_ADMISSION_REPORT.md.

## Phase 1 scope boundary

This document defines **structure and contracts only**. No code, no new constitutional articles, no new CCS scenarios, no new commercial claims. Implementation follows BUILD_ORDER.md after `REAK_PHASE1_GATE.json` freeze.
