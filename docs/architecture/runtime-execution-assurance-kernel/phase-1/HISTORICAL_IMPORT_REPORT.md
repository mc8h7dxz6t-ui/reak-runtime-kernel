# Historical import report

**Reference implementation:** Consequence II (`axiom-consequence-ii`, assembly revision `d8728cf…`).  
**Rule:** No automatic import. Each import requires mapped IQ evidence and admission row.

## Import policy

| Policy | Meaning |
|--------|---------|
| **COPY_SEMANTICS** | Reimplement against REAK contracts; may read reference for behaviour |
| **COPY_TESTS_AS_SPEC** | Port tests into Qualification Kernel, not REAK repo |
| **REFERENCE_ONLY** | Design comparison only |
| **DO_NOT_IMPORT** | Known harmful or obsolete pattern |

## By historical artefact

| Historical item | REAK target | Policy | Justification |
|-----------------|-------------|--------|---------------|
| `canonical_observation_runtime` (ID-005) | reak-observation | COPY_SEMANTICS | R0 III-4; IQ-005 gap — tests to Qualification Kernel |
| `canonical_reconciliation_runtime` (ID-006) | reak-reconciliation | COPY_SEMANTICS | R0 III-5; IQ-006 gap |
| `canonical_truth_runtime` (ID-007) | reak-truth | COPY_SEMANTICS + COPY_TESTS_AS_SPEC | IQ-007 IMPLEMENTATION_QUALIFIED |
| `canonical_recovery_runtime` (ID-008) | reak-recovery | COPY_SEMANTICS + COPY_TESTS_AS_SPEC | IQ-008 qualified |
| `canonical_safenext_runtime` (ID-009) | reak-progression | COPY_SEMANTICS + COPY_TESTS_AS_SPEC | IQ-009 PARTIALLY_QUALIFIED — carry limitations L-01..L-05 |
| `canonical_commitment_runtime` (ID-010) | reak-commitment | COPY_SEMANTICS | R4 definition package; IQ-010 not done — no qualification import |
| Dispatch (unimplemented) | reak-dispatch | REFERENCE_ONLY | R5 spec from roadmap + C-02 themes |
| id-001 durable epoch | reak-durable-record + reak-policy-context | COPY_SEMANTICS | G4/G5 |
| id-002 boundary | reak-boundary | COPY_SEMANTICS | IQ-002a |
| id-003 registry | reak-registry | COPY_SEMANTICS | IQ-003 |
| id-004 replay | reak-replay | COPY_SEMANTICS | IQ-004, c-04 |
| axiom-authority / mandate | reak-authority | COPY_SEMANTICS | III-1 |
| exposure patterns in kernel stack | reak-exposure | COPY_SEMANTICS | III-2, R4 binding |
| c-05 evidence constitution | reak-evidence-hooks | REFERENCE_ONLY | Hooks in kernel; catalogue outside |
| int-001 integration | reak-lifecycle-shell | COPY_TESTS_AS_SPEC | Assembly pattern R3 |
| ext-001 external adapter | reak-integration-host | COPY_SEMANTICS partial | Must not own dispatch |
| UES manifests | reak-uncertainty-ledger | COPY_SEMANTICS | Art V |
| CCS corpus | Qualification Kernel | DO_NOT_IMPORT into REAK | R10–11 |
| HRT-001 design | Qualification Kernel | DO_NOT_IMPORT into REAK | R9 |
| EvidenceLab code | EvidenceLab repo | DO_NOT_IMPORT into REAK | R12 |
| Blind campaign fixtures | Research Sandbox | REFERENCE_ONLY | Hostile themes inform THREAT_MODEL |
| ENGINEERING_PROGRAMME_STATE.json | Programme docs | REFERENCE_ONLY | Stage tracking, not runtime |

## IQ evidence admission status (R1)

| IQ | Admitted for REAK port? | Notes |
|----|-------------------------|-------|
| IQ-007 | YES | Truth ceiling frozen |
| IQ-008 | YES | Recovery non-executing |
| IQ-009 | YES_WITH_LIMITATIONS | Document in IQ-PRG-01 |
| IQ-005 | NO (gap) | Re-qualify on REAK module |
| IQ-006 | NO (gap) | Re-qualify on REAK module |
| IQ-010 | N/A | Not executed |

## Structural anti-patterns to avoid on import

1. **Monolithic `axiom-consequence-ii` crate** — REAK should use one crate per module or explicit feature flags with forbidden edges enforced by CI.
2. **Qualification tests inside runtime crate** — Move to Qualification Kernel driving IF-EXPORT-01 and public APIs.
3. **Implicit dispatch via adapter** — ext-001 pattern explicitly rejected.
4. **Policy epoch optional on specimen paths** — IQ-009 L-01: REAK must make epoch mandatory at commitment.

## Legacy Archive handling

- Tag Consequence II tree as `legacy/consequence-ii` read-only.
- REAK git history starts clean; no subtree merge without per-file admission row.

## Estimated reuse

| Category | Approx. share of reference LOC |
|----------|-------------------------------|
| Port with re-qualification | 45–55% (truth, recovery, progression, observation, reconciliation) |
| Rebuild clean (commitment, dispatch, host) | 25–35% |
| Discard (qualification, evidencelab, research) | 20–30% |

Percentages are planning estimates, not commitments to copy code.
