# Complexity review

## Status

**NOT RUN.** No lines of implementation code to measure.

## Programme-level complexity risk

| ID | Observation | Evidence | Impact |
| --- | --- | --- | --- |
| CX-001 | Large specification corpus without matching code | Many `*.md` architecture docs, zero `*.rs`/`*.go` | Teams may implement divergent interpretations |
| CX-002 | Multiple historical programme names in one repo | Consequence, CCS, EVP, REAK hardening folder | Onboarding cost, wrong mental model |

## Recommendations

### CX-R-001

- **Problem.** Implementers may not know which doc is normative for REAK.
- **Evidence.** Repo contains `ARCHITECTURE.md`, `CONSTITUTION.md`, `CCS.md`, and `reak-hardening/` without a REAK architecture pointer in README.
- **Impact.** Duplicate logic, missed invariants.
- **Suggested change.** Single `REAK-ARCHITECTURE.md` pointer in README: “frozen architecture lives at …”; hardening programme references only that plus intake manifest.
- **Risk.** Documentation churn only.
