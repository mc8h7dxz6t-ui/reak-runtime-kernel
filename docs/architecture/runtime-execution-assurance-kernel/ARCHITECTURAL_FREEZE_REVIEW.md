# Architectural freeze review — REAK runtime kernel v1.0.0

**Date:** 2026-09-30  
**Scope:** `runtime-execution-assurance-kernel/` workspace after Phase 9 **ACCEPTED / FROZEN**  
**Method:** Questions only — no new implementation.

## Spine (normative)

```
Foundation (types, durable-record, replay, policy-context, ues, registry, authority, exposure)
    → Commitment → Dispatch → Observation → Reconciliation → Truth → Recovery → Progression
```

**Execution boundary:** only **Dispatch** (`execute_once` / ticket lifecycle) performs outbound execution. Everything after Dispatch is **reasoning** (classify, derive, propose, permit) on append-only durable history.

---

## Review questions

### 1. Is every dependency one-way?

**Yes** (production `Cargo.toml` graph).

- Spine flows forward: each phase crate depends on substrate and on **earlier** phase artefacts (types/records), not on later phases.
- `reak-progression` depends on `reak-truth` and `reak-recovery` only — not on dispatch.
- `reak-recovery` depends on `reak-truth` only (test fixtures may pull reconciliation/commitment; not shipped).
- `reak-reconciliation` **reads** dispatch and observation histories; it does not depend on `reak-truth`, `reak-recovery`, or `reak-progression`.

**Caveat (documented, not a cycle):** reconciliation takes read-only views of dispatch and observation streams — fan-in at the reasoning layer, not a reverse edge.

### 2. Are any cycles left?

**No** in production dependencies. Workspace resolver finds no `A → B → A` among `reak-*` engine crates.

### 3. Can every phase be replayed independently?

**Yes.** Each engine phase exposes `verify()` and `recover_from_log()` (or equivalent) over its own durable stream id:

| Phase | Stream / log | Replay API |
|-------|----------------|------------|
| Commitment | CMT | `verify`, `recover_from_log` |
| Dispatch | DSP | `verify`, `recover_from_log` |
| Observation | OBS | `verify`, `recover_from_log` |
| Reconciliation | REC | `verify`, `recover_from_log` |
| Truth | TRU | `verify`, `recover_from_log` |
| Recovery | RCV | `verify`, `recover_from_log` |
| Progression | PRG | `verify`, `recover_from_log` |

Empty stream is treated as valid where implemented (`StreamNotFound` → OK on verify).

### 4. Can every phase be qualified independently?

**Yes**, with **synthetic or fixture upstream history** (Qualification Kernel pattern):

- Lower phases can be qualified with generated durable envelopes only.
- Upper phases consume **immutable record types**; tests already use fixtures without live dispatch.

No phase requires a cloud provider or adapter to qualify.

### 5. Does any module own two responsibilities?

**No** at the constitutional phase boundary:

| Module | Single responsibility |
|--------|------------------------|
| Commitment | Bind intent / generation |
| Dispatch | Ticket + **sole execution** release |
| Observation | Append observations |
| Reconciliation | Compare dispatch vs observation evidence |
| Truth | Derive conclusion from reconciliation |
| Recovery | Propose strategy + authorization intent (no execution) |
| Progression | Classify next admissible step (no execution) |

**Shared substrate** (`reak-types`, `reak-durable-record`, `reak-replay`, policy/ues/authority/exposure) is foundation, not a second “phase.”

### 6. Does any module perform another module's work?

**No** in the accepted gates:

- Observation does not reconcile or derive truth.
- Truth does not reconcile or observe.
- Recovery does not dispatch or mutate truth.
- Progression does not recover, dispatch, or upgrade truth.

Dispatch does not reconcile or conclude truth.

### 7. Can every module be replaced independently?

**Yes in principle**, at **interface contracts** (`IF-CMT-01` … `IF-PRG-01`):

- Replacement requires preserving record schemas and stream semantics for downstream readers.
- Reconciliation is the tightest coupling: it depends on **shapes** of dispatch and observation records, not on their internal engines — swap engines if export formats stay versioned.

### 8. Can every stream be reconstructed from durable history?

**Yes.** Append-only `reak-durable-record` + `reak-replay` hash chain; each phase’s `recover_from_log` rebuilds in-memory indexes from the verified snapshot.

---

## Verdict

| Question | Answer |
|----------|--------|
| One-way dependencies | **Yes** |
| No cycles | **Yes** |
| Independent replay | **Yes** |
| Independent qualification | **Yes** |
| Single responsibility per phase | **Yes** |
| No cross-phase work theft | **Yes** |
| Replaceable modules | **Yes** (contract-versioned) |
| Streams reconstructible | **Yes** |

**Recommendation:** **FREEZE** the runtime reasoning kernel at **v1.0.0**. Do not add runtime modules, adapters, or providers in the golden kernel repo. Commercial and experimental work belong in **reak-platform** and **reak-research** clones only.

---

## Related

- [KERNEL_V1_GOLDEN_CLONE.md](./KERNEL_V1_GOLDEN_CLONE.md)
- [REAK_KERNEL_V1_FREEZE.json](./REAK_KERNEL_V1_FREEZE.json)
- [REPOSITORY_PROGRAMME_TOPOLOGY.md](./REPOSITORY_PROGRAMME_TOPOLOGY.md)
