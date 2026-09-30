# REAK engineering governance

**Programme:** Runtime Execution Assurance Kernel (REAK)  
**Status:** Normative for all REAK engineering after Phase 1 acceptance  
**Supersedes:** Ad-hoc module debates; re-opens constitutional, EVP, or PRB work **only** when implementation exposes a genuine obligation that cannot be satisfied without amendment (expected: rare).

---

## Programme conversation role

This programme is led as **Chief Architect / Technical Director** for REAK.

**In scope**

- Kernel architecture and module graph
- Runtime engineering and production readiness
- Stable, versioned interfaces
- Security architecture and threat modelling
- Dependency discipline (acyclic, forbidden edges)
- Qualification strategy (what must be proven, not running Qualification Kernel)
- Performance architecture where it affects guarantees

**Out of scope (frozen inputs)**

- Product positioning and messaging
- Market research and EVP tracks
- Constitutional redesign and architecture debates
- EvidenceLab product design (except runtime touchpoints via contracts)

---

## Permanent review question

For every module, interface, dependency, threat model, and qualification package:

> **Does this belong in the Runtime Execution Assurance Kernel?**

Not: *Can we build it?*

That distinction preserves a small kernel over a multi-year programme.

---

## Engineering principle: kernel smaller than the ecosystem

**The Kernel Must Stay Smaller Than The Ecosystem.**

If AWS, Microsoft, Temporal, Kubernetes, or another mature platform already provides a capability **well** for its layer, **consume** it via `reak-integration-host` (or successor adapters). Build only what is genuinely the kernel’s constitutional runtime responsibility.

| Layer | Prefer ecosystem | Prefer kernel |
|-------|------------------|---------------|
| Orchestration / scheduling | Temporal, Step Functions, K8s controllers | Bind, fence, sole dispatch semantics |
| Identity / IAM | IdPs, cloud IAM | Scoped **attempt** permission for consequence class |
| Telemetry | Metrics/traces vendors | Canonical **observation** records for consequence |
| Storage / messaging | Managed DB, queues, logs | Append-only **authoritative** record contract |
| Assurance reports | EvidenceLab | Admissibility hooks + export only |

When a platform later ships an equivalent **normative** capability with independent verify, shrink the kernel module per removal analysis—do not duplicate.

---

## Module Admission Rule (permanent)

Every **future** `reak-*` module (including splits, merges, or renames) must pass **all six** tests before admission. Fail one → **stays outside** the kernel (Qualification Kernel, EvidenceLab, Research Sandbox, or host adapter only).

| # | Test | Question |
|---|------|----------|
| 1 | **Constitutional owner** | Which Article / R0 row owns this responsibility? If none → reject. |
| 2 | **Runtime responsibility** | Must execute or enforce on the hot path during consequential operation? If qualification-only or report-only → reject. |
| 3 | **External justification** | Why does the market/ecosystem not already own this at the right layer? (Frozen EVP/PRB may be cited; no new market research in REAK chat.) |
| 4 | **Qualification strategy** | Named IQ/gate, hostile themes, failure modes, claim ceiling, acceptance criteria **before** implementation. |
| 5 | **Security review** | Trust zone, privilege, threat scenarios updated; no new bypass of commitment/dispatch spine. |
| 6 | **Removal analysis** | What platform or module obsoletes this? What shrinks if we integrate instead of build? |

**Process**

1. Propose admission in a short **Module Admission Addendum** (append to phase-1 `MODULE_ADMISSION_REPORT.md` or new `phase-N/MODULE_ADMISSION_ADDENDUM_<id>.md`).
2. Update `KERNEL_MODULE_GRAPH.json`, `DEPENDENCY_GRAPH.json`, `INTERFACE_CONTRACTS.json`, `QUALIFICATION_MATRIX.json`, `TRUST_BOUNDARY_MAP.md`, `THREAT_MODEL.md` in the same change set.
3. No code in `reak-*` until admission row is merged and governance ack recorded.

Phase 1 admitted modules are grandfathered but remain subject to **removal** if ecosystem or qualification falsifies need.

---

## Frozen programme boundaries

| Programme | Owns |
|-----------|------|
| **REAK** | Production runtime planes + host integration |
| **Qualification Kernel** | CCS, HRT, IQ execution, fuzz/chaos/mutation |
| **EvidenceLab** | Reports, certificates, verification of exported evidence |
| **Legacy Archive** | Consequence II (read-only reference) |

See [phase-1/PROGRAMME_TRIAD_BOUNDARIES.md](./phase-1/PROGRAMME_TRIAD_BOUNDARIES.md).

---

## Phase status

| Phase | Status | Gate |
|-------|--------|------|
| Phase 1 — Architecture & admission | **ACCEPTED / FROZEN** | [phase-1/REAK_PHASE1_GATE.json](./phase-1/REAK_PHASE1_GATE.json) |
| Phase 2 — Foundation substrate | **AUTHORIZED** | Stabilise durable record, policy context, UES, registry, replay, authority, exposure **before** Commitment |

Phase 2 entry: this governance document + accepted Phase 1 gate. No Commitment (R4) implementation until foundation interfaces are stable and qualification stubs exist in Qualification Kernel.

---

## Related documents

- [REAK_ENGINEERING_GOVERNANCE.md](./REAK_ENGINEERING_GOVERNANCE.md) (this file)
- [phase-1/README.md](./phase-1/README.md)
- [phase-2/README.md](./phase-2/README.md)
