# REAK build guide (intake)

## Verification status

**NOT VERIFIED.** Phase 2B could not run build or tests. No REAK workspace exists in the connected repository at commit `f0c11c2f149a2e90abc7b4037ae8546bd710bcb4`.

## Repository identity

| Field | Value |
| --- | --- |
| Identity | `origin.cursor.com/git/philip-macleod-dev/tmp-db34ee8b37f8bab7` |
| URL | `https://origin.cursor.com/git/philip-macleod-dev/tmp-db34ee8b37f8bab7.git` |
| Branch | `main` |
| Commit SHA | `f0c11c2f149a2e90abc7b4037ae8546bd710bcb4` |

## Toolchain

| Item | Value |
| --- | --- |
| Toolchain | **UNKNOWN** — no `rust-toolchain.toml`, `go.mod`, or lockfile for REAK in tree |
| Compiler version | **NOT RECORDED** |
| Dependency lock | **NOT PRESENT** for REAK |

## Commands (to run after implementation is connected)

Replace `REAK_ROOT` with the path declared in `manifest.json` → `implementation.implementation_root`.

### Workspace verification

```bash
cd "$REAK_ROOT"
git rev-parse HEAD
# Must match manifest.json repository.commit_sha or a documented override for impl submodule
```

### Build

```bash
# Example only — use the actual build system once manifest lists crates:
# cargo build --workspace --locked
# or: go test ./...
```

### Tests

```bash
# cargo test --workspace --locked
```

### Property tests

```bash
# cargo test -p <crate> --features proptest
# Record exact flags in this file after first successful run.
```

## Reproducibility checklist (Phase 2B exit)

- [ ] `implementation_root` set in `manifest.json`
- [ ] Lockfile committed (`Cargo.lock` / `go.sum` / equivalent)
- [ ] Pinned toolchain file committed
- [ ] Build succeeds on clean checkout
- [ ] Tests succeed
- [ ] Commit SHA recorded in manifest

None of the above is complete for REAK in the current connected tree.
