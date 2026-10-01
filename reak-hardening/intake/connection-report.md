# Phase 2B — Connection report

## Mission

Connect the Hardening Programme to the Runtime Execution Assurance Kernel implementation via **read-only** access to the canonical REAK repository at a pinned revision. No engineering review in this phase.

**No REAK/runtime source was modified** in the programme repository or in the canonical REAK repository during verification or intake updates.

**No Phase 3 work was performed** (security, concurrency, or hostile review not started).

## Programme workspace

| Field | Value |
| --- | --- |
| Root | `/workspace` |
| HEAD (programme repo at verification, before the CONNECTED documentation commit) | `e7d82de4acf2861041ef20684a038a16bf2087e9` |
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
git ls-remote https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git refs/heads/main
git ls-remote https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git refs/tags/v1.0.0
git ls-remote https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git 'refs/tags/v1.0.0^{}'
rustc --version
cargo --version
rm -rf /tmp/reak-runtime-phase2b-connected-check
git clone --no-checkout https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git /tmp/reak-runtime-phase2b-connected-check
cd /tmp/reak-runtime-phase2b-connected-check
git checkout 65cea8f8921909bd44b970d1a69b2da362d7a4a8
cd runtime-execution-assurance-kernel
cargo metadata --no-deps --format-version 1
cargo build --workspace --locked
cargo test --workspace --locked
```

Observed checkout `HEAD`: `65cea8f8921909bd44b970d1a69b2da362d7a4a8`.

Toolchain: **rustc 1.99.0** / **cargo 1.99.0** (stable).

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
| HTTPS `git ls-remote` | **SUCCESS** — SHAs match expected values above |
| Read-only checkout | **SUCCESS** — `/tmp/reak-runtime-phase2b-connected-check` |
| `runtime-execution-assurance-kernel/Cargo.toml` | **Present**; workspace version `1.0.0` |
| `cargo metadata --no-deps --format-version 1` | **SUCCESS** |
| `cargo build --workspace --locked` | **SUCCESS** |
| `cargo test --workspace --locked` | **SUCCESS** |

## Programme gate

| Field | Value |
| --- | --- |
| `connection_status` | **CONNECTED** |
| Phase 2B scope | Connection / intake evidence only |
| Qualification | **Not passed** (no qualification programme claim) |
| Production / certification / adapters / providers / cloud | **Not claimed** |
| Phase 3 | **Not started** |
| REAK source in programme repo | **Not present** (`implementation.present` = false) |

## Distinction (programme vs REAK pin)

`manifest.json` → `repository.commit_sha` is the **programme documentation repository** snapshot at verification (`e7d82de4acf2861041ef20684a038a16bf2087e9`) — the **pre-CONNECTED-documentation** parent used for the successful rerun, **not** the post-commit CONNECTED evidence SHA (not invented until that commit exists). It is **not** the REAK kernel pin. The REAK pin is `implementation.canonical_reak_*` and revision `v1.0.0` / peeled commit `65cea8f8921909bd44b970d1a69b2da362d7a4a8`.
