# Threat model (Phase 1)

**Scope:** REAK embedded in hostile multi-tenant enterprise environments.  
**Out of scope:** Qualification tooling compromise (Qualification Kernel TM), report forgery (EvidenceLab TM).

## Assets

| Asset | Impact if compromised |
|-------|------------------------|
| Commitment digest chain | Double spend, forged intent |
| Dispatch ticket store | Duplicate external effects |
| Truth records | False Established conclusions |
| Policy epoch pins | Non-reproducible decisions |
| Authority grants | Unauthorized consequential attempts |
| Evidence export bundles | False third-party assurance |

## Threat actors

| Actor | Capability | Primary targets |
|-------|------------|-----------------|
| Malicious provider | Lie in callbacks, replay old acks | Observation, reconciliation |
| Compromised host workflow | Skip steps, call APIs directly | Integration host, dispatch bypass |
| Tenant attacker | Cross-tenant probe | Boundary, durable record partition |
| Insider operator | Break-glass abuse | Art VI path (ops, not kernel) |
| Network adversary | Replay, MITM on Z0 | Boundary, dispatch credentials |

## STRIDE summary

| Threat | Mitigation (architectural) | Qualification hook |
|--------|---------------------------|-------------------|
| Spoofing | Authority + commitment bind identity | IQ-AUTH, IQ-CMT, CCS Art III-10 |
| Tampering | Append-only durable record | IQ-DR, CCS G4 scenarios |
| Repudiation | Export audit slice + external EvidenceLab verify | R12, IF-EXPORT-01 |
| Information disclosure | Tenant partition; least privilege per module | Hostile tenant tests |
| Denial of service | Exposure ceilings; explicit capacity collapse records | ID-005 class gaps, CCS |
| Elevation of privilege | Single dispatch owner; forbidden dependency edges | R9 HRT, blind campaign themes |

## Attack scenarios (must fail closed)

1. **Dispatch without commitment** — Host calls provider API directly → host adapter must not expose credentials without ticket; qualification proves bypass impossible for supported adapters.
2. **Double dispatch same generation** — Second ticket consumption rejected with auditable record (Art IV G2).
3. **Provider ack → Established** — Truth rejects without reconciliation + admissibility (Art II).
4. **Recovery loop executes effect** — Recovery interface has no execute operation; static forbidden edge to dispatch.
5. **Replay reconstruct → redispatch** — Replay non-normative; cannot invoke IF-DSP-01.
6. **Stale policy epoch** — Commitment rejects epoch skew beyond tolerance window.
7. **Zombie worker after failover** — Generation monotonicity + ticket single-use; HRT R9 scenarios.

## Trust assumptions (explicit)

Aligned with constitution Part 4 assumptions A1–A10:

- Host will invoke REAK hooks if integrated (malicious host = out of TCB; detected via missing records).
- Durable store meets availability targets (DR is implementation).
- Cryptographic primitives in boundary TLS are sound.

## Residual risk (accepted for Phase 1)

- Colluding provider + observer (A9) — not solved in kernel alone; federation research outside REAK.
- Break-glass social engineering — operational controls Art VI.

## Obsolescence of controls

If hyperscalers ship **independently auditable** commit+fence APIs with third-party verify, `reak-commitment` / `reak-dispatch` may shrink to verification shims — tracked in MODULE_ADMISSION_REPORT removal analysis.
