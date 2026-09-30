# Phase 6 security review

Reconciliation cannot invoke dispatch or truth APIs. Observation records are read-only inputs.

Fail-closed: invalid expected ticket, UES exhaustion, aligned emit attempts, and broken chains return errors.

**Verdict:** Consistent with OBS → REC → TRU spine; no architectural contradiction.
