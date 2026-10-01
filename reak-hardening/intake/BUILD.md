# REAK build guide (intake)

## Verification status

**PARTIALLY VERIFIED — Phase 2B not complete.** Canonical REAK was verified on a **read-only temporary checkout** at pinned revision `v1.0.0`. `connection_status` remains **BLOCKED** (`BLOCKED_TEST_TOOLCHAIN`). **CONNECTED** has not been asserted. Qualification has not passed.

## Programme workspace (this agent)

| Field | Value |
| --- | --- |
| Programme repository root | `/workspace` |
| Programme repository HEAD | `c85271bc363bd92e4c7e92724d192460dce3c14a` |
| Programme repository status (before intake alignment edits) | **clean** |
| Programme repository role | Hardening / programme documentation only |
| REAK source tree in programme repo | **No** (`implementation.present` = false) |

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
| Location | `/tmp/reak-runtime-phase2b-checkout` |
| Mode | Detached at peeled commit `65cea8f8921909bd44b970d1a69b2da362d7a4a8` (not copied into programme repo) |
| `Cargo.lock` | `runtime-execution-assurance-kernel/Cargo.lock` (relative to checkout root) |

## Toolchain (verification environment)

| Item | Value |
| --- | --- |
| `rustc` / `cargo` (agent) | Cargo **1.83.0** (2024-10-29) |
| REAK `rust-toolchain.toml` in checkout | **Not present** at verification time |
| Lockfile | Present at `runtime-execution-assurance-kernel/Cargo.lock` |

## Commands executed (pinned checkout)

All commands run from:

```bash
cd /tmp/reak-runtime-phase2b-checkout/runtime-execution-assurance-kernel
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

**Result:** **NOT EXECUTED SUCCESSFULLY** — failure solely due to local Cargo **1.83.0** lacking stabilized `edition2024` support required when resolving test dependencies (e.g. `getrandom` 0.4.3 registry fetch during test build).

Example error excerpt:

```text
error: failed to parse manifest at `.../getrandom-0.4.3/Cargo.toml`
feature `edition2024` is required
The package requires the Cargo feature called `edition2024`, but that feature is not stabilized in this version of Cargo (1.83.0 ...)
```

## Condition before CONNECTED can be asserted

All of the following are required (programme gate; see `../PHASE2B-GATE.md`):

1. `cargo test --workspace --locked` (or programme-agreed equivalent) **succeeds** on the pinned REAK commit using a **compatible toolchain** (Cargo/Rust new enough for the locked test dependency graph, or REAK documents a pinned toolchain that satisfies the lockfile).
2. Intake evidence (`BUILD.md`, `connection-report.md`, `manifest.json`) records successful test execution on `65cea8f8921909bd44b970d1a69b2da362d7a4a8`.
3. Programme inventories under `inventories/` per gate criteria (not generated in this partial verification pass).

Until tests succeed under a compatible toolchain, gate remains **BLOCKED** (`BLOCKED_TEST_TOOLCHAIN`). Do not mark **CONNECTED** or claim qualification passed.

## Reproducibility checklist (Phase 2B exit)

- [x] Canonical REAK repository and revision recorded
- [x] Read-only checkout at pinned peeled commit
- [x] `cargo metadata` on pinned workspace
- [x] `cargo build --workspace --locked` on pinned workspace
- [ ] `cargo test --workspace --locked` on pinned workspace (**blocked — toolchain**)
- [ ] `connection_status` → CONNECTED (explicitly **not** set)
