# Phase 2B — Connection report

## Mission

Connect the Hardening Programme to the Runtime Execution Assurance Kernel implementation via **read-only** access to the canonical REAK repository at a pinned revision. No engineering review in this phase.

**No runtime source was modified** in the programme repository or in the canonical REAK repository during verification or intake alignment.

**No Phase 3 work was performed** (security, concurrency, or hostile review not started).

## Programme workspace

| Field | Value |
| --- | --- |
| Root | `/workspace` |
| HEAD | `c85271bc363bd92e4c7e92724d192460dce3c14a` |
| Status before intake alignment edits | **clean** |
| Role | Hardening / programme documentation only |
| REAK source tree in programme repo | **Absent** (`implementation.present` = false) |

## Canonical REAK (verified refs)

| Ref | SHA |
| --- | --- |
| `refs/heads/main` | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `refs/tags/v1.0.0` (annotated tag object) | `953ee352ebca7f44c8f6394a528e47be5534bed7` |
| `refs/tags/v1.0.0^{}` (peeled commit) | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |

| Field | Value |
| --- | --- |
| Repository URL | `https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git` |
| Remote name | `reak-runtime` |
| Pinned revision | `v1.0.0` |

## Commands executed

```bash
git remote set-url reak-runtime https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git
git fetch reak-runtime --tags
git ls-remote reak-runtime refs/heads/main
git ls-remote reak-runtime refs/tags/v1.0.0
git ls-remote reak-runtime 'refs/tags/v1.0.0^{}'
git clone --depth 1 --branch v1.0.0 https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git /tmp/reak-runtime-phase2b-checkout
cd /tmp/reak-runtime-phase2b-checkout/runtime-execution-assurance-kernel
cargo metadata --no-deps --format-version 1
cargo build --workspace --locked
cargo test --workspace --locked
```

Observed checkout `HEAD`: `65cea8f8921909bd44b970d1a69b2da362d7a4a8`.

## Module inventory (from `cargo metadata` only)

Paths are relative to the REAK repository root at the pinned commit.

| Crate | Module key | Path |
| --- | --- | --- |
| `reak-types` | types | `runtime-execution-assurance-kernel/crates/reak-types` |
| `reak-durable-record` | durable-record | `runtime-execution-assurance-kernel/crates/reak-durable-record` |
| `reak-policy-context` | policy-context | `runtime-execution-assurance-kernel/crates/reak-policy-context` |
| `reak-ues` | ues | `runtime-execution-assurance-kernel/crates/reak-ues` |
| `reak-registry` | registry | `runtime-execution-assurance-kernel/crates/reak-registry` |
| `reak-replay` | replay | `runtime-execution-assurance-kernel/crates/reak-replay` |
| `reak-authority` | authority | `runtime-execution-assurance-kernel/crates/reak-authority` |
| `reak-exposure` | exposure | `runtime-execution-assurance-kernel/crates/reak-exposure` |
| `reak-commitment` | commitment | `runtime-execution-assurance-kernel/crates/reak-commitment` |
| `reak-dispatch` | dispatch | `runtime-execution-assurance-kernel/crates/reak-dispatch` |
| `reak-observation` | observation | `runtime-execution-assurance-kernel/crates/reak-observation` |
| `reak-reconciliation` | reconciliation | `runtime-execution-assurance-kernel/crates/reak-reconciliation` |
| `reak-truth` | truth | `runtime-execution-assurance-kernel/crates/reak-truth` |
| `reak-recovery` | recovery | `runtime-execution-assurance-kernel/crates/reak-recovery` |
| `reak-progression` | progression | `runtime-execution-assurance-kernel/crates/reak-progression` |

## Verification results

| Step | Result |
| --- | --- |
| HTTPS `git fetch reak-runtime --tags` / `git ls-remote` | **SUCCESS** — SHAs match expected values above |
| Read-only checkout | **SUCCESS** — `/tmp/reak-runtime-phase2b-checkout` |
| `runtime-execution-assurance-kernel/Cargo.toml` | **Present**; workspace version `1.0.0` |
| `cargo metadata --no-deps --format-version 1` | **SUCCESS** |
| `cargo build --workspace --locked` | **SUCCESS** |
| `cargo test --workspace --locked` | **NOT SUCCESSFUL** — local Cargo 1.83.0 lacks `edition2024` for locked test graph |

### Test blocker

**Gate:** `BLOCKED_TEST_TOOLCHAIN`

`cargo test --workspace --locked` did not complete successfully in this Cloud Agent environment. Cargo **1.83.0** does not provide stabilized `edition2024` support required when building test-only dependencies from the lockfile resolution path (e.g. `getrandom` 0.4.3). This is an **environment toolchain limitation**, not evidence that the pinned REAK commit failed review.

**Required to clear blocker:** Re-run `cargo test --workspace --locked` on commit `65cea8f8921909bd44b970d1a69b2da362d7a4a8` with a compatible Rust/Cargo toolchain (or documented REAK toolchain pin that satisfies the lockfile), then update intake evidence before asserting **CONNECTED**.

## Inventories (programme gate)

Full inventories under `../inventories/` are **not** generated in this pass (programme rule: after successful build/test). Module list above is metadata-only for intake alignment.

## Programme gate

| Field | Value |
| --- | --- |
| `connection_status` | **BLOCKED** |
| Programme gate document (`../PHASE2B-GATE.md`) | Verdict **BLOCKED** (no CONNECTED sub-status) |
| Intake-specific blocker | **BLOCKED_TEST_TOOLCHAIN** |
| CONNECTED | **Not asserted** |
| Qualification | **Not passed** |
| Phase 3 | **Not started** |

See `../PHASE2B-GATE.md` for CONNECTED criteria. Intake files record verified facts to date. Phase 2B is **not complete**. Gate remains **BLOCKED** pending successful `cargo test --workspace --locked` on the pinned REAK commit under a compatible toolchain.

## Distinction (programme vs REAK pin)

`manifest.json` → `repository.commit_sha` is the **programme documentation repository** snapshot (`c85271bc363bd92e4c7e92724d192460dce3c14a`). It is **not** the REAK kernel pin. The REAK pin is `implementation.canonical_reak_*` and revision `v1.0.0` / peeled commit `65cea8f8921909bd44b970d1a69b2da362d7a4a8`.
