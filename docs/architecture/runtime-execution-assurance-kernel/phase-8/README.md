# REAK Phase 8 — Recovery engine

**Status:** Implementation complete — gate **PROPOSED**  
**Contract:** IF-RCV-01 (`reak-recovery`)

Consumes immutable `TruthRecord` only; emits immutable recovery records and strategy. Never dispatches or mutates truth.

**Verify:** `cd runtime-execution-assurance-kernel && cargo test -p reak-recovery && cargo test --workspace`
