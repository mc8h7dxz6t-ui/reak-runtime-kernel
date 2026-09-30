# Performance report

## Status

**NOT EXECUTED.** No benchmarks, profiles, or load tests run against REAK in this repository.

## DoS and collapse hypotheses (for future run)

| ID | Scenario | Failure mode to detect |
| --- | --- | --- |
| PERF-01 | Large authorization payload | OOM or unbounded alloc |
| PERF-02 | Flood of append requests | Log commit latency → mistaken timeout → wrong state |
| PERF-03 | Verification on huge history | CPU collapse blocking release path |
| PERF-04 | Parallel verify + sign | Lock contention deadlock |

## Resource exhaustion tests

Specified in `02-test-catalogue.md`. Not run.

## Recommendations

### PERF-R-001

- **Problem.** No baseline latency or memory budget for sign critical section.
- **Evidence.** No benchmark artefacts in repo.
- **Impact.** Cannot detect performance regressions that become safety bugs (timeout → retry).
- **Suggested change.** Record p99 sign+commit latency under CI with a fixed corpus size; fail CI on regression beyond agreed threshold once baseline exists.
- **Risk.** Low once baseline is stable; false positives if environment noisy.
