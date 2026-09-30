# Risk register

| ID | Category | Description | Likelihood | Impact | Mitigation status | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| R-01 | Process | Kernel source not in review repo | **Certain** | Critical | **Open — Phase 2 blocker B-01** | `intake/phase2-scan.md` |
| R-02 | Process | No intake manifest | **Certain** | High | **Open — Phase 2 blocker B-02** | `manifest.json` absent |
| R-03 | Security | Unreviewed release path | Unknown | Critical | Blocked on R-01 | NOT RUN |
| R-04 | Concurrency | Double release under one authorization | Unknown | Critical | Blocked on R-01 | NOT RUN |
| R-05 | Safety | Panic in sign path leaves ambiguous state | Unknown | High | Blocked on R-01 | NOT RUN |
| R-06 | Supply chain | Dependencies unaudited | Unknown | Medium | Blocked on manifest | NOT RUN |
| R-07 | Performance | Retry storm on slow commit | Unknown | High | Blocked on benchmarks | NOT RUN |
| R-08 | Determinism | Hidden clock or env in decision | Unknown | High | Blocked on R-01 | NOT RUN |
| R-09 | Documentation | Wrong normative doc for implementers | Medium | Medium | Open — CX-R-001 | README layout |

Phase 2 gate: **HARDENING_PHASE2_BLOCKED** (`PHASE2-GATE.md`).
