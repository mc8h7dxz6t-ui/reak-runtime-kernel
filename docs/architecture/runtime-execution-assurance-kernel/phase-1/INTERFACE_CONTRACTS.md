# Interface contracts

Normative contract definitions live in [INTERFACE_CONTRACTS.json](./INTERFACE_CONTRACTS.json). This document states integration rules.

## Contract principles

1. **Versioned surfaces** — Hosts and qualification tooling bind to `interface_id` + major version. Minor versions add optional fields only.
2. **Dependency inversion** — Plane modules depend on `IF-DR-01`, `IF-POL-01`, `IF-UES-01` abstractions, not storage implementations.
3. **Records, not callbacks** — Cross-plane communication appends authoritative records; synchronous calls are facades over append+read.
4. **No qualification types in hot path** — Test hooks are compile-time or side-channel exports via `IF-EXPORT-01`, not production dispatch paths.

## Spine contracts

| Order | Interface | Guarantees |
|-------|-----------|------------|
| 1 | IF-BND-01 | Untrusted input never reaches commitment without normalization or explicit refusal record |
| 2 | IF-CMT-01 | Digest binds authority scope, intent hash, policy epoch, lineage generation |
| 3 | IF-DSP-01 | Ticket consumable once per generation; duplicates produce auditable rejection |
| 4 | IF-OBS-01 → IF-REC-01 → IF-TRU-01 | No truth without reconciliation; no reconciliation without observations |
| 5 | IF-RCV-01 / IF-PRG-01 | Outputs are classes/intents only; no `execute` operation on these interfaces |

## Evidence and qualification egress

`IF-EXPORT-01` is the **only** supported path for EvidenceLab and Qualification Kernel to consume runtime state. It must not expose mutable handles or internal reducers.

## Mapping from R3 assembly interfaces

Historical assembly used `IF-005-006`, `IF-006-007`, etc. REAK renames seams but preserves semantics:

| Legacy seam | REAK seam |
|-------------|-----------|
| IF-005-006 | IF-OBS-01 → IF-REC-01 |
| IF-006-007 | IF-REC-01 → IF-TRU-01 |
| IF-007-008 | IF-TRU-01 → IF-RCV-01 |
| IF-008-009 | IF-TRU-01 → IF-PRG-01 |

Commitment and dispatch seams are **new** relative to R3 manifest and align to R4/R5.

## Type definitions (Phase 2)

Concrete schema for `RecordEnvelope`, `CommitmentRecord`, and related types are **not** part of Phase 1. Phase 2 will publish OpenAPI/Protobuf/JSON Schema per interface with compatibility tests.
