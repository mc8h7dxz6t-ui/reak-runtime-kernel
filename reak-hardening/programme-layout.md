# Programme layout (recommended)

This is organisational guidance for how disconnected programme repos should be arranged. It does not change frozen REAK architecture.

## Canonical sources of truth

| Programme | Repository | Role |
| --- | --- | --- |
| **REAK Runtime** | One canonical **Rust** repository | Implementation, tests, lockfile, toolchain. Single place build and release run. |
| **Qualification Kernel** | Separate repository | CCS execution, hostile corpus runners, fuzz/chaos infrastructure. Consumes REAK as a specimen under test. |
| **EvidenceLab** | Separate repository | Independent qualification (sealed corpus, assessor, issuer, verifier). Does not operate the runtime. |
| **Hardening** | **No implementation repository** | Lives as a **programme** (charter, reports, gate) that **consumes REAK read-only**. |

## Why

- Hardening does not guess where implementation lives: it always targets the canonical REAK repo at a pinned SHA.
- Independence is preserved by **read-only** access (clone, submodule, or CI checkout at ref), not by copying source into a docs-only tree.
- Qualification and EvidenceLab stay separate duties and trust boundaries, aligned with the frozen constitutions.

## Hardening connection model

1. **Input:** canonical REAK repo URL + commit SHA (or tag) in `intake/manifest.json`.
2. **Access:** read-only clone or submodule at that ref. Hardening commits only under `reak-hardening/` in a **programme** repo (or the same monorepo’s docs path), never under `reak/` crate roots.
3. **Phase 2B:** verify `cargo build` / `cargo test` on the **REAK** tree at the pinned SHA; populate `manifest.json` → `modules` from that workspace.
4. **Phase 3:** adversarial review against that tree; no writes to REAK.

## Current gap

The agent workspace today is a **programme/documentation** repository, not the canonical REAK Rust repo. Phase 2B remains **BLOCKED** until `manifest.json` points at a real REAK repository and SHA with a verifiable build.

## Canonical remote name

Use git remote **`reak-runtime`** for the canonical Rust repository (URL per environment). Hardening and Qualification Kernel pin with `templates/HARDENING_PIN.json` (`reak_revision`).

## What to do next (one action)

Point intake at that repo and SHA, then re-run Phase 2B:

- `manifest.json` → `implementation.canonical_reak_repository` + `repository.commit_sha` (= `reak_revision`)
- `modules[]` from `cargo metadata` on a read-only checkout at that SHA

Gate **CONNECTED** when build and tests pass. See [REPOSITORY_PROGRAMME_TOPOLOGY.md](REPOSITORY_PROGRAMME_TOPOLOGY.md).
