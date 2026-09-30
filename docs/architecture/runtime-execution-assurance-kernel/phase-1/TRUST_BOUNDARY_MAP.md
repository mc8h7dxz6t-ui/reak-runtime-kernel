# Trust boundary map

## Zones

| Zone | Trust level | Actors | Data crossing |
|------|-------------|--------|---------------|
| **Z0 External provider** | Untrusted | SaaS APIs, banks, cloud control planes | Raw responses, callbacks |
| **Z1 Host orchestrator** | Semi-trusted | Temporal, Step Functions, K8s, MCP host | Schedules work; must not bypass commitment |
| **Z2 REAK privileged core** | Trusted computing base | All `reak-*` plane modules | Authoritative records only |
| **Z3 Evidence export** | Read-only egress | EvidenceLab, Qualification Kernel | `IF-EXPORT-01` bundles |
| **Z4 Operator** | Policy-bound human | Break-glass, pilot oversight | Art VI paths; non-normative |

## Privilege boundaries

```
Z0 ──(TLS, auth)──► reak-boundary (Z2)
Z1 ──(host token)──► reak-integration-host (Z2) ──► reak-lifecycle-shell
reak-dispatch ──(scoped creds)──► Z0   [only after IF-CMT-01 success]
Z2 ──(IF-EXPORT-01)──► Z3
```

| Module | May write durable records | May invoke external effect | May assert Established truth |
|--------|---------------------------|----------------------------|------------------------------|
| reak-boundary | Refusal, normalized intent | No | No |
| reak-commitment | Commitment | No | No |
| reak-dispatch | Effect attempt | **Yes (sole)** | No |
| reak-observation | Observation | No | No |
| reak-reconciliation | Reconciliation | No | No |
| reak-truth | Truth | No | **Yes (under policy)** |
| reak-progression | Progression | No | No |
| reak-recovery | Recovery intent | No | No |
| reak-replay | Replay report | No | No |
| reak-registry | Registry pointer | No | No |
| reak-evidence-hooks | Admissibility verdict | No | No |

## Tenant isolation

- Lineage identifiers are namespaced per tenant at ingress (`IF-BND-01`).
- Policy epoch resolution is tenant-scoped (`IF-POL-01`).
- Durable record partitions are logical per tenant; physical isolation is implementation (Phase 2+).
- Cross-tenant reads are forbidden at contract level.

## Epoch / version control

- Every decision record carries `policy_epoch` and `schema_version`.
- Host adapters declare `host_contract_version` in `IF-HOST-01` callbacks.
- Upgrades: new epoch mandatory for new behavior; old epochs remain replayable.

## Immutable records

All Z2 mutations go through `IF-DR-01` append or supersede. No in-place updates to authoritative history.

## Failure containment

- Plane failure returns explicit record types (Hold, Denied, Violation) — never silent pass-through to dispatch.
- Z1 cannot inject dispatch without commitment digest verification.

## Secure defaults

| Check | Default |
|-------|---------|
| Missing commitment | Dispatch denied |
| Missing observation timeout | Reconciliation emits explicit gap; truth non-established |
| Ambiguous progression | Hold (Art IV G3) |
| Replay tool invoked | Cannot call IF-DSP-01 (forbidden edge) |
