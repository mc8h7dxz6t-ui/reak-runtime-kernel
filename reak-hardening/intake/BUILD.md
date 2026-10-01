# REAK build guide (intake)

## Verification status

**VERIFIED — Phase 2B connection/intake bar satisfied** on a **read-only temporary checkout** at pinned revision `v1.0.0` / peeled commit `65cea8f8921909bd44b970d1a69b2da362d7a4a8`. `connection_status` is **CONNECTED** for Phase 2B only.

This is **not** a qualification, certification, or production-readiness claim. Phase 3 has **not** started.

## Programme workspace (this agent)

| Field | Value |
| --- | --- |
| Programme repository root | `/workspace` |
| Programme repository HEAD (at verification, before the CONNECTED documentation commit) | `e7d82de4acf2861041ef20684a038a16bf2087e9` |
| Programme repository role | Hardening / programme documentation only |
| REAK source tree in programme repo | **No** (`implementation.present` = false) |

`repository.commit_sha` in `manifest.json` records that **pre-CONNECTED-documentation** programme parent (`e7d82de…`), not the post-commit CONNECTED evidence SHA (recorded in a follow-up after that commit exists).

## Canonical REAK (pinned)

| Field | Value |
| --- | --- |
| Repository | `https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git` |
| Git remote name | `reak-runtime` |
| Branch | `main` |
| Main commit | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| Annotated tag | `v1.0.0` |
| Tag object SHA | `953ee352ebca7f44c8f6394a528e47be5534bed7` |
| Peeled commit (tag target) | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| Kernel workspace path (in checkout) | `runtime-execution-assurance-kernel/` |
| Workspace package version | `1.0.0` |

## Read-only verification checkout

| Field | Value |
| --- | --- |
| Location | `/tmp/reak-runtime-phase2b-connected-check` |
| Mode | Detached at peeled commit `65cea8f8921909bd44b970d1a69b2da362d7a4a8` (not copied into programme repo) |
| `Cargo.lock` | `runtime-execution-assurance-kernel/Cargo.lock` (relative to checkout root) |

## Toolchain (verification environment)

| Item | Value |
| --- | --- |
| `rustc` | **1.99.0** (b940084d7 2026-09-28) |
| `cargo` | **1.99.0** (5f94df478 2026-08-27) |
| REAK `rust-toolchain.toml` in checkout | **Not present** at verification time |
| Lockfile | Present at `runtime-execution-assurance-kernel/Cargo.lock` |

## Commands executed (pinned checkout)

All commands run from:

```bash
cd /tmp/reak-runtime-phase2b-connected-check/runtime-execution-assurance-kernel
```

### Metadata

```bash
cargo metadata --no-deps --format-version 1
```

**Result:** **SUCCESS** — 15 workspace packages (see `manifest.json` → `modules[]`).

### Build

```bash
cargo build --workspace --locked
```

**Result:** **SUCCESS** (dev profile).

### Tests

```bash
cargo test --workspace --locked
```

**Result:** **SUCCESS** (exit 0; workspace unit tests on pinned commit).

## Phase 2B bar

Metadata, locked build, and locked tests **succeeded** on the pinned REAK checkout using the toolchain above. Phase 2B **connection/intake** requirements for build/test evidence are satisfied.

Phase 3 (adversarial review), full programme inventories under `inventories/`, and any qualification programme remain **out of scope** for this document.

## Reproducibility checklist (Phase 2B)

- [x] Canonical REAK repository and revision recorded
- [x] Read-only checkout at pinned peeled commit
- [x] `cargo metadata` on pinned workspace
- [x] `cargo build --workspace --locked` on pinned workspace
- [x] `cargo test --workspace --locked` on pinned workspace
- [x] `connection_status` → CONNECTED (Phase 2B intake only)
