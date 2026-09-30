# Programme gate — Phase 2B (connect to implementation)

## Verdict

**BLOCKED**

## Reason (evidence)

| Check | Required | Actual |
| --- | --- | --- |
| REAK source in connected repo or submodule | Yes | **Absent** (`intake/connection-report.md`) |
| `manifest.json` with module entries | Yes | **`modules`: []** |
| `BUILD.md` with verified commands | Yes | **Placeholder only; build not run** |
| Build succeeds | Yes | **NOT RUN** |
| Tests execute | Yes | **NOT RUN** |
| Reproducible workspace | Yes | **NOT VERIFIED** |
| Commit SHA recorded | Yes | **`f0c11c2f149a2e90abc7b4037ae8546bd710bcb4`** (programme repo only) |

## CONNECTED criteria (all required)

1. `implementation.implementation_root` (or submodule) points at a tree that contains the kernel workspace.
2. `manifest.json` → `modules` is non-empty for every implemented crate/module.
3. `BUILD.md` documents toolchain, lockfile, and commands that were executed successfully on the recorded SHA.
4. Module, dependency, public API, and trust-boundary inventories exist under `inventories/`.

## Next phase

Phase 3 (security, concurrency, hostile review) **must not start** until this gate is **CONNECTED**.
