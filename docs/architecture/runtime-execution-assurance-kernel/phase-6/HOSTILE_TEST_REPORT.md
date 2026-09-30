# Phase 6 hostile test report

| Test | Scenario | Result |
|------|----------|--------|
| `missing_observation_is_mismatch` | Zero observations | Pass |
| `conflicting_observations_not_collapsed` | Success + failure same ticket | Pass |
| `ues_exhaustion_fails_closed` | 101 emits | Pass |
| `mismatch_emitted_for_unknown_vs_success` | Unknown not promoted | Pass |
| `aligned_when_expected_matches_observed` | No emit on aligned | Pass |
