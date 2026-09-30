# Runtime Execution Assurance Kernel (REAK)

**Canonical Rust repository** for REAK runtime implementation (`reak-*` workspace).

Programme architecture, gates, and governance: `../docs/architecture/runtime-execution-assurance-kernel/` (see `REPOSITORY_PROGRAMME_TOPOLOGY.md`).

Qualification Kernel, EvidenceLab, and Hardening **do not** duplicate this tree; they pin this repo read-only (Hardening) or call public APIs/exports only.

## Phase 2 — Foundation

| Crate | Module | Contract |
|-------|--------|----------|
| `reak-durable-record` | reak-durable-record | IF-DR-01 |
| `reak-policy-context` | reak-policy-context | IF-POL-01 |
| `reak-ues` | reak-uncertainty-ledger | IF-UES-01 |
| `reak-registry` | reak-registry | IF-REG-01 |
| `reak-replay` | reak-replay | IF-RPL-01 |
| `reak-authority` | reak-authority | IF-AUTH-01 |
| `reak-exposure` | reak-exposure | IF-EXP-01 |
| `reak-commitment` | reak-commitment | IF-CMT-01 |

```bash
cargo test --workspace
cargo test --workspace -- --ignored  # includes stress tests if marked
```

## Dependency policy

Workspace crates only, plus: `serde`, `serde_json`, `sha2`, `hex`, `thiserror`, `parking_lot`, `proptest` (dev/tests).
