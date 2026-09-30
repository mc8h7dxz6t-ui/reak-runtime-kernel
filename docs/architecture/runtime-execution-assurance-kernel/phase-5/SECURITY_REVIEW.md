# Phase 5 security review

## Privilege

Observation engine cannot call dispatch or commitment APIs. Crate dependency graph excludes execution planes.

## Fail-closed

Invalid references, oversized payloads, replayed source events, and broken hash chains return errors. No silent drops.

## Unknown handling

No code path maps `ObservedUnknown` to success or failure.

## Concurrency

Append mutex; concurrent distinct `source_event_id` values succeed; concurrent replay of same event: one winner.

## Verdict

Aligned with Phase 1 sense → reconcile → truth ordering. No architectural contradiction identified.
