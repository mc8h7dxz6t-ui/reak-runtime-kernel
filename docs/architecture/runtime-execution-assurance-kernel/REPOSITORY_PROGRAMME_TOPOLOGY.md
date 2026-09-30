# Repository and programme topology

**Status:** Normative for engineering operations (post–Phase 2 acceptance).  
**Does not modify:** Phase 1 architecture artefacts (see [phase-1/](./phase-1/)).

## Canonical source of truth

| Programme | Repository | Write access | Consumes |
|-----------|------------|--------------|----------|
| **REAK Runtime** | **One canonical Rust repository** (workspace root: `runtime-execution-assurance-kernel/`) | REAK engineering only | — |
| **Qualification Kernel** | Separate repository | Qualification engineering | REAK **public APIs + `IF-EXPORT-01`** only; never runtime internals |
| **EvidenceLab** | Separate repository | EvidenceLab engineering | REAK exported evidence; never hot path |
| **Hardening** | **No repository of its own** | N/A (read-only) | **REAK canonical repo at pinned revision** (tag/SHA) |

## Rationale

- **One runtime repo** avoids drift between “implementation,” “hardening copy,” and “qualification fork.”
- **Hardening independence** is preserved by **read-only** checkout or CI clone of the canonical REAK revision—not by a duplicate codebase.
- **Qualification Kernel** and **EvidenceLab** stay separate so assurance and reports never merge into the runtime binary.

```
                    ┌─────────────────────────────┐
                    │  REAK Runtime (canonical    │
                    │  Rust repo — sole writer)   │
                    └──────────────┬──────────────┘
                                   │ pin rev (tag/SHA)
           ┌───────────────────────┼───────────────────────┐
           │ read-only             │ public API / export   │ read-only
           ▼                       ▼                       ▼
   ┌───────────────┐     ┌─────────────────┐     ┌──────────────┐
   │  Hardening    │     │ Qualification   │     │ EvidenceLab  │
   │  (programme;  │     │ Kernel repo     │     │ repo         │
   │   no repo)    │     │                 │     │              │
   └───────────────┘     └─────────────────┘     └──────────────┘
```

## Hardening programme rules

1. **Pin** an explicit REAK git revision (commit SHA or signed tag) per campaign or CI matrix row.
2. **Read-only** working tree: no commits back to REAK; findings become issues or Qualification Kernel test additions.
3. **No second runtime tree** for “hardening convenience”; use worktrees or ephemeral clones if isolation is needed.
4. Hostile/fuzz/chaos **execution** may live in Qualification Kernel; **hardening** owns campaign design, revision pins, and residual-risk reports against that pin.

## REAK repo layout (current)

- **Code:** `runtime-execution-assurance-kernel/` Cargo workspace (`reak-*` crates).
- **Architecture & gates:** `docs/architecture/runtime-execution-assurance-kernel/` (agent store mirror or monorepo docs path—same content, programme docs not split per phase repo).

When the canonical remote is created, it should contain **both** the Rust workspace and programme documentation, or document a single submodule policy—default: **monorepo** with runtime at repo root workspace path above.

## Legacy

**Consequence II / Legacy Archive** remains read-only reference; it is not the canonical REAK repository.

## Related

- [REAK_ENGINEERING_GOVERNANCE.md](./REAK_ENGINEERING_GOVERNANCE.md)
- [phase-1/PROGRAMME_TRIAD_BOUNDARIES.md](./phase-1/PROGRAMME_TRIAD_BOUNDARIES.md) (frozen triad; repository detail superseded by this doc for ops)
