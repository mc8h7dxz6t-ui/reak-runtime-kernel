# Test catalogue (programme-level)

Tests are specified here. Execution waits on kernel source and intake manifest.

## Property tests (cross-cutting hypotheses)

| ID | Property | Modules | Oracle |
| --- | --- | --- | --- |
| P-01 | Same sealed inputs and rule yield same decision | decision / policy | Byte-identical output |
| P-02 | No release without prior durable authorization record | commitment / gate | Instrument or mock effect count ≤ 1 |
| P-03 | Append-only: replay never shortens history | log / store | Hash chain monotonic |
| P-04 | Unknown outcome does not increase release count | dispatch | Count stable under timeout |
| P-05 | Token or binding rejects bit flip | crypto / wire | Verify fails closed |

## Fuzz cases

| ID | Input surface | Strategy | Crash = fail |
| --- | --- | --- | --- |
| F-01 | Deserialization of records | Structured + arbitrary bytes | Yes |
| F-02 | Authorization blob | Mutate length, tags, nested depth | Yes |
| F-03 | Wire request after sign | AFL/libFuzzer on canonical encoding | Yes |
| F-04 | Config file | grammar-aware | Yes |

## Mutation tests

Run on decision and parsing code only after baseline tests exist. Target branches that gate release and verification.

## Concurrency tests

| ID | Scenario | Expected |
| --- | --- | --- |
| C-01 | Two threads release same authorization | At most one effect or one recorded success |
| C-02 | Release during crash after record commit | No second release on restart |
| C-03 | Parallel append to log | Linearizable order or detectable fork |
| C-04 | Read during write of checkpoint | No torn read without detection |

## Crash simulations

| ID | Kill point | Expected |
| --- | --- | --- |
| K-01 | After commit, before sign | No sign without record |
| K-02 | After sign, before effector return | Unknown, not second sign |
| K-03 | Mid-serialize | No partial record accepted as valid |

## Soak

24–72 h steady load at modest QPS with leak and handle checks. Not a pass/fail on throughput alone.

## Fault injection

Disk full, ENOSPC on fsync, clock jump, network partition to external mock, delayed responses, duplicate delivery.

## State corruption

Load store with one byte flipped; expect verify failure, no release.

## Resource exhaustion

Max message size, max open files, goroutine/thread storm on accept loop; must fail closed without unbounded allocation.
