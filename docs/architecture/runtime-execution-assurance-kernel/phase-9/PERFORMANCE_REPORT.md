# Phase 9 performance report

**Command:** `cargo test -p reak-progression --test prg_performance`

## Scenario

`classify_one_thousand_under_budget`: 1000 invocations of `classify_only` on a fixed `ProgressionInput` (success + `NoAction`).

## Budget

Wall time &lt; **500 ms** on debug profile (CI/dev hardware).

## Result

Pass — classification is lightweight (validation + match on strategy/authorization/truth).

## Notes

- `determine_next_step` includes durable append and index updates; not benchmarked in this phase.  
- Production targets should be measured on release builds with configured durable backends (future adapter phases).
