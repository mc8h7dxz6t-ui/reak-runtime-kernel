# Hardening report

Programme: REAK hardening. Scope: structural quality under frozen architecture.

## Executive summary

**Status: NOT EXECUTED against kernel modules.**

Evidence: this repository contains no REAK implementation source. Per-module threat models, attack surfaces, and test results cannot be produced without intake (see `reak-hardening/README.md`).

## Module coverage

| Module | Review | Tests generated | Tests run |
| --- | --- | --- | --- |
| (none in repo) | NOT RUN | Catalogue only (`02-test-catalogue.md`) | NOT RUN |

## Cross-programme findings (evidence-backed)

### H-001 — No review target in scope

- **Problem.** Adversarial work cannot falsify the kernel if the kernel is not in the tree under review.
- **Evidence.** Zero implementation files in `/workspace`; `git log` shows documentation commits only.
- **Impact.** Any claim of “hardened” or “reviewed” is unauditable.
- **Suggested change.** Add `reak-hardening/intake/manifest.json` pointing at module paths and pin commit SHA; CI job `reak-hardening-smoke` runs build + unit tests on every push to that path.
- **Risk.** Low. Process only. Does not change kernel behaviour.

### H-002 — Test catalogue not wired

- **Problem.** Property, fuzz, and concurrency IDs exist only as specifications.
- **Evidence.** `02-test-catalogue.md` has no corresponding `tests/` or `fuzz/` directories.
- **Impact.** Regressions ship without hostile coverage.
- **Suggested change.** When first module lands, require one property test and one fuzz target before merge to `main` for that module.
- **Risk.** Medium. Early friction on velocity; reduces later incident cost.

## Architecture review (coupling, no redesign)

NOT RUN. No modules to map dependencies or public API size.

## Complexity review

NOT RUN.

## Priority ordering

See `recommended-fixes.md`.
