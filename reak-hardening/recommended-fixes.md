# Recommended fixes (priority order)

No kernel code changes are listed. Programme and intake only.

## P0 — Unblock adversarial work

### FIX-001

- **Problem.** Hardening cannot run.
- **Evidence.** Repository scan: zero implementation files.
- **Impact.** False confidence if programme is treated as complete.
- **Suggested change.** Publish kernel source (or submodule) and `reak-hardening/intake/manifest.json` with module list and SHA.
- **Risk.** Low.

### FIX-002

- **Problem.** No reproducible build for reviewers.
- **Evidence.** No `Cargo.toml`, `go.mod`, or equivalent in workspace.
- **Impact.** Findings cannot be reproduced.
- **Suggested change.** Add `reak-hardening/intake/BUILD.md` with exact commands and toolchain pins.
- **Risk.** Low.

## P1 — Wire hostile tests on first module

### FIX-003

- **Problem.** Test catalogue is inert.
- **Evidence.** `02-test-catalogue.md` without `tests/` tree.
- **Impact.** First modules ship without fuzz or concurrency coverage.
- **Suggested change.** For each new module in manifest, add at least one entry from F-01 or P-01 and C-01 before merge.
- **Risk.** Medium — slows first merge.

## P2 — CI gates (after baseline exists)

### FIX-004

- **Problem.** No regression detection on sign critical section.
- **Evidence.** `performance-report.md` PERF-R-001.
- **Impact.** Latency regressions become safety bugs.
- **Suggested change.** Benchmark job on pinned hardware or normalized CI runner; store baseline artifact.
- **Risk.** Flaky CI if environment not controlled.

## P3 — Documentation pointer

### FIX-005

- **Problem.** Normative REAK architecture unclear in repo layout.
- **Evidence.** `complexity-review.md` CX-001, CX-002.
- **Impact.** Implementation drift.
- **Suggested change.** README links frozen REAK architecture location; hardening ignores legacy programme docs unless referenced by REAK manifest.
- **Risk.** Low.

## Deferred until modules exist

- Per-module security findings
- Dependency removal
- Interface shrinking
- Cryptographic review

Do not prioritize speculative refactors.
