# Risk register

| ID | Category | Description | Likelihood | Impact | Mitigation status | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| R-01 | Process | Kernel source not in review repo | High | High | Open — intake required | No impl files |
| R-02 | Security | Unreviewed release path | Unknown | Critical | Blocked on R-01 | NOT RUN |
| R-03 | Concurrency | Double release under one authorization | Unknown | Critical | Tests specified, not run | `02-test-catalogue.md` C-01 |
| R-04 | Safety | Panic in sign path leaves ambiguous state | Unknown | High | Tests specified, not run | K-02 |
| R-05 | Supply chain | Dependencies unaudited | Unknown | Medium | Blocked on manifest | NOT RUN |
| R-06 | Performance | Retry storm on slow commit | Unknown | High | Blocked on benchmarks | PERF-R-001 |
| R-07 | Determinism | Hidden clock or env in decision | Unknown | High | Property P-01 specified | NOT RUN |
| R-08 | Documentation | Wrong normative doc for implementers | Medium | Medium | CX-R-001 proposed | README layout |

Update after each module review. Close only with evidence (test name, commit, log).
