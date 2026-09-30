# REAK Phase 2 — Foundation substrate

**Status:** Authorized (not started)  
**Governance:** [REAK_ENGINEERING_GOVERNANCE.md](../REAK_ENGINEERING_GOVERNANCE.md)

## Objective

Stabilise the infrastructure every later plane depends on. **Do not** implement Commitment or Dispatch in Phase 2.

## Module order (strict)

| Order | Module | Interface(s) | Exit signal |
|-------|--------|--------------|-------------|
| 2.1 | `reak-durable-record` | IF-DR-01 | Append/supersede contract stable; Qualification Kernel IQ-DR-01 stub |
| 2.2 | `reak-policy-context` | IF-POL-01 | Epoch pin/resolution; deterministic subset documented |
| 2.3 | `reak-uncertainty-ledger` | IF-UES-01 | Stage budgets enforced at API boundary |
| 2.4 | `reak-registry` | IF-REG-01 | Pointer semantics; no correctness assertions |
| 2.5 | `reak-replay` | IF-RPL-01 | Non-normative; forbidden edges to dispatch verified in CI |
| 2.6 | `reak-authority` | IF-AUTH-01 | Scoped grant/deny records |
| 2.7 | `reak-exposure` | IF-EXP-01 | Reservation/ceiling/derate records |

## Phase 2 gate (to be created)

Phase 2 completes when:

- All seven modules have implementation + public contracts at declared major version.
- `DEPENDENCY_GRAPH.json` lint passes (including forbidden edges).
- Each module has Qualification Matrix row with **draft** IQ in Qualification Kernel repo (may be separate repository).
- No `IF-CMT-01` or `IF-DSP-01` surface exposed to hosts.

## Chief Architect review checklist

For each PR / module increment, apply the [Module Admission Rule](../REAK_ENGINEERING_GOVERNANCE.md#module-admission-rule-permanent) and ask: **Does this belong in REAK?**
