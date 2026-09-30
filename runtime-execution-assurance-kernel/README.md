# Runtime Execution Assurance Kernel (REAK)

Production runtime implementation. Architecture and admission: `../docs/architecture/runtime-execution-assurance-kernel/`.

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

```bash
cargo test --workspace
cargo test --workspace -- --ignored  # includes stress tests if marked
```

## Dependency policy

Workspace crates only, plus: `serde`, `serde_json`, `sha2`, `hex`, `thiserror`, `parking_lot`, `proptest` (dev/tests).
