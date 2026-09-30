# Programme triad and preservation boundaries

## Mission split

REAK Phase 1 defines **what the production runtime must own**. Everything else is a separate engineering programme with explicit egress/ingress contracts.

```
┌─────────────────────────────────────────────────────────────────┐
│ Host environments (Temporal, Step Functions, K8s, MCP, HTTP)    │
└────────────────────────────┬────────────────────────────────────┘
                             │ integration-host (adapters only)
┌────────────────────────────▼────────────────────────────────────┐
│ RUNTIME EXECUTION ASSURANCE KERNEL (REAK)                       │
│ Constitutional planes + durable record + policy epoch + UES     │
│ Emits: append-only records, evidence handles, progression     │
└────────────┬───────────────────────────────┬────────────────────┘
             │ read-only evidence export      │ qualification API
┌────────────▼──────────────┐    ┌─────────────▼──────────────────┐
│ EvidenceLab               │    │ Qualification Kernel           │
│ Verify bundles, reports,  │    │ CCS, HRT, IQ runners, mutants  │
│ certificates, ceilings    │    │ Never depends on runtime internals│
└───────────────────────────┘    └────────────────────────────────┘
```

## Inside REAK (production runtime only)

| Category | Included | Rationale |
|----------|----------|-----------|
| Constitutional planes | Authority, Exposure, Boundary, Observation, Reconciliation, Truth, Evidence **admissibility hooks**, Recovery, Progression, Commitment, Dispatch, Registry, Replay | R0 owners; FINAL constitution Part 1 R1–R13 |
| Cross-cutting invariants | Durable append record, policy epoch binding, per-stage uncertainty ledger | Art IV G4–G5, Art V |
| Host integration | Adapter façade, ticket/lineage types, no orchestration engine | Integration-first mandate; complements Temporal et al. |
| Operational telemetry | Structured audit events derived from authoritative records | Observability of **decisions**, not replacement of Observation plane |

## Outside REAK (explicit exclusions)

| Category | Owner programme | Why excluded |
|----------|-----------------|--------------|
| CCS corpus execution, mutants, injectors | Qualification Kernel | Roadmap R10–R11; Art IV G6 governance, not hot path |
| IQ test suites, HRT orchestration | Qualification Kernel | R9; must qualify multiple runtimes without internal coupling |
| Assurance reports, certificates, published claim ceilings | EvidenceLab | R12; consumes runtime evidence, must not execute effects |
| Evidence bundle authoring UI / federation research | EvidenceLab + Research Sandbox | C-05 catalogue semantics; federation not kernel-qualified |
| Passport, Witness, Knowledge planes | Research Sandbox (optional future) | Constitution Part 2: not elevated to unanimous owners |
| Break-glass procedures | Operations + governance | Art VI: non-normative, auditable exception path |
| Qualification governance process | Engineering programme (manifests) | R14: governance responsibility, not runtime module |
| Provider-specific SDK shims beyond boundary | Host integration adapters | Keep kernel provider-agnostic; R14 per-target qualification |

## Preservation model

| Zone | Rule | Contents |
|------|------|----------|
| **Legacy Archive** | Never modified | Consequence II tree, historical IQ gates, R3 manifest at `d8728cf…` |
| **Research Sandbox** | Unrestricted experimentation | Attestation, formal methods, scale, privacy proofs |
| **Production Kernel** | Admission-gated only | Greenfield REAK; imports from Legacy only via HISTORICAL_IMPORT_REPORT |

## Boundary justification tests

Every inbound dependency to REAK must pass:

1. **Constitutional owner** — mapped in R0 row or Part 1 responsibility.
2. **Roadmap owner** — R4–R8 runtime stages or cross-cutting invariant required before R9 assembly.
3. **External justification** — EVP capability theme or PRB kill-stack gap (no single vendor owns full stack per PRB charter).
4. **Commercial justification** — wedge requires bind-before-effect, independent evidence, reconciliation separate from observability (EVP tracks B, E, F).
5. **Qualification strategy** — IQ or assembly gate named before code (QUALIFICATION_MATRIX).
6. **Removal analysis** — documented in MODULE_ADMISSION_REPORT; rejected modules stay out.

## Dependency direction (normative)

- REAK **may not** call into Qualification Kernel or EvidenceLab at runtime.
- Qualification Kernel **may** drive REAK through public integration-host and record export APIs only.
- EvidenceLab **may** ingest exported records and qualification artefacts; never participates in dispatch.
