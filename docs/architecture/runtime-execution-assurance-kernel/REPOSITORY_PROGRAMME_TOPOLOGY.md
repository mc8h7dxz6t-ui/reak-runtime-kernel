# Repository and programme topology

**Status:** Normative for engineering operations (post–Phase 2 acceptance).  
**Does not modify:** Phase 1 architecture artefacts (see [phase-1/](./phase-1/)).

## Canonical source of truth (post v1.0.0 freeze)

| Programme | Repository | Write access | Consumes |
|-----------|------------|--------------|----------|
| **REAK Runtime Kernel** | **`reak-runtime-kernel`** — golden clone at tag **`v1.0.0`**; workspace `runtime-execution-assurance-kernel/` | Kernel maintainers: **fixes only** | — |
| **REAK Platform** | **`reak-platform`** — clone of kernel + commercial adapters | Platform engineering | Pinned kernel tag/crates |
| **REAK Research** | **`reak-research`** — clone for experiments | Research engineering | Pinned kernel tag/crates |
| **REAK Runtime (legacy row)** | Same tree as kernel until remotes split; tag **`v1.0.0`** marks golden master | See kernel rules | — |
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

## Kernel v1.0.0 golden clone

After Phase 9 acceptance, the reasoning spine is **complete**. Tag **`v1.0.0`** freezes the kernel. Create three repositories from that tag (see [KERNEL_V1_GOLDEN_CLONE.md](./KERNEL_V1_GOLDEN_CLONE.md)). Do **not** add runtime modules or adapters to **reak-runtime-kernel**.

## Related

- [REAK_ENGINEERING_GOVERNANCE.md](./REAK_ENGINEERING_GOVERNANCE.md)
- [KERNEL_V1_GOLDEN_CLONE.md](./KERNEL_V1_GOLDEN_CLONE.md)
- [ARCHITECTURAL_FREEZE_REVIEW.md](./ARCHITECTURAL_FREEZE_REVIEW.md)
- [REAK_KERNEL_V1_FREEZE.json](./REAK_KERNEL_V1_FREEZE.json)
- [phase-1/PROGRAMME_TRIAD_BOUNDARIES.md](./phase-1/PROGRAMME_TRIAD_BOUNDARIES.md) (frozen triad; repository detail superseded by this doc for ops)
