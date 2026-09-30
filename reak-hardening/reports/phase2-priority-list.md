# Phase 2 — Priority list

Only evidence-backed programme actions. No runtime code changes by hardening team.

| Priority | Item | Evidence | Confidence |
| --- | --- | --- | --- |
| P0 | Publish `intake/manifest.json` | B-02 in `PHASE2-GATE.md` | High |
| P0 | Land kernel source in connected repo or submodule | B-01 in `PHASE2-GATE.md` | High |
| P0 | Add `intake/BUILD.md` and verify build on CI | Phase 1 FIX-002 | High |
| P1 | Re-run Phase 2 module reports for each manifest entry | Gate conditions in `PHASE2-GATE.md` | High |
| P1 | Wire P-01, P-02, C-01, F-01 on first two modules | `phase2-testing-gaps.md` | Medium (after P0) |
| P2 | Benchmark sign critical section | Phase 1 PERF-R-001 | Low until code exists |

No kernel fix list — no findings on implementation.
