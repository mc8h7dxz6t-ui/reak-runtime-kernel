# Repository programme topology (normative)

Aligned with programme governance: one canonical runtime, separate qualification and evidence repos, hardening without a shadow copy of REAK.

This copy lives in the hardening programme tree for agents working in this workspace. If a fuller topology doc exists in the REAK programme workspace, that document wins on conflict; update this file to match.

## Table

| Programme | Repository | Access |
| --- | --- | --- |
| **REAK Runtime** | One canonical **Rust** repository (`runtime-execution-assurance-kernel/` workspace) | REAK team writes implementation |
| **Qualification Kernel** | Separate repository | Public APIs / export surfaces only; pins REAK at a SHA |
| **EvidenceLab** | Separate repository | Exported evidence only; does not operate REAK |
| **Hardening** | **No repository of its own** | **Read-only** clone or worktree of REAK at a **pinned SHA or tag**; findings → issues or Qualification Kernel tests, **not** runtime forks |

## Rules

1. All `reak-*` implementation stays in the canonical Rust repo.
2. Hardening campaigns record `reak_revision` (see `templates/HARDENING_PIN.json`).
3. Qualification Kernel and EvidenceLab depend on REAK; they do not copy the workspace into their trees.
4. Phase 1 triad boundaries remain frozen; ops and repo detail is here and in engineering governance on the programme side.

## Canonical remote name (recommended)

**`reak-runtime`** — git remote used by CI and hardening for read-only checkouts. URL is environment-specific; the pin file carries the SHA.

## Machine-checkable pin

Qualification Kernel (when created) should hold `HARDENING_PIN.json` at repo root. Template: `reak-hardening/templates/HARDENING_PIN.json`.

Check: `reak_revision` resolves in the canonical remote; hardening and qual CI fail if pin is missing or SHA is unknown.

## This cloud workspace

The repository connected to this agent (`tmp-db34ee8b37f8bab7`) is **not** the canonical REAK Rust repo unless `intake/manifest.json` says otherwise. Phase 2B stays blocked until intake points at `reak-runtime` (or equivalent) and a verified SHA.
