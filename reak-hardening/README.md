# REAK hardening programme

Runtime Execution Assurance Kernel. Architecture frozen. Foundation implementation is claimed to be underway.

This workstream maximises structural quality. It does not redesign architecture, add features, or expand scope.

## Evidence status (this repository)

FACT. Phase 2 scan (`intake/phase2-scan.md`): no kernel source tree and no `intake/manifest.json` in the connected repository after `git pull`.

**Gate Phase 2:** `HARDENING_PHASE2_BLOCKED` (`PHASE2-GATE.md`).  
**Gate Phase 2B (connect):** **BLOCKED** (`PHASE2B-GATE.md`). Intake: `intake/manifest.json`, `intake/BUILD.md`, `intake/connection-report.md`.

RECOMMENDATION. Land REAK implementation + populate `manifest.json` modules[], verify build in `BUILD.md`, then set Phase 2B to CONNECTED before Phase 3 review.

## Intake (required before module reviews)

1. **Manifest.** A machine-readable list of modules: name, path, public API surface, trust boundary, and owning team.
2. **Build.** Reproducible build instructions and a pinned toolchain file.
3. **Threat model v0.** One page per module from the implementers, listing assets, actors, and trust assumptions. The hardening team will attack these; they are not accepted as truth.
4. **Baseline.** Commit SHA, test command, and coverage or property-test inventory if any exists.

Deliver intake to this directory as `reak-hardening/intake/manifest.json` and `reak-hardening/intake/BUILD.md`.

## Deliverables

| Document | Purpose |
| --- | --- |
| [00-programme-charter.md](00-programme-charter.md) | Rules of engagement |
| [01-module-review-template.md](01-module-review-template.md) | Per-module adversarial review |
| [02-test-catalogue.md](02-test-catalogue.md) | Property, fuzz, mutation, concurrency, chaos |
| [reports/hardening-report.md](reports/hardening-report.md) | Consolidated structural findings |
| [reports/security-report.md](reports/security-report.md) | Security findings only |
| [reports/performance-report.md](reports/performance-report.md) | Performance and DoS |
| [reports/architecture-review.md](reports/architecture-review.md) | Coupling and interface size (no redesign) |
| [reports/complexity-review.md](reports/complexity-review.md) | Simplification candidates |
| [risk-register.md](risk-register.md) | Living register |
| [recommended-fixes.md](recommended-fixes.md) | Evidence-backed recommendations |

## Recommendation format

Every recommendation uses:

- Problem
- Evidence
- Impact
- Suggested change
- Risk (of applying the change)

No automatic repair. No automatic redesign.
