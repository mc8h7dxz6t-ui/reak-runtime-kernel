# REAK module inventory (Phase 2B pin)

**Source:** `cargo metadata` from pinned canonical REAK **`v1.0.0`** at commit **`65cea8f8921909bd44b970d1a69b2da362d7a4a8`** (recorded in `intake/manifest.json` and Phase 2B verification evidence).

**Scope:** Crate names and programme-facing role labels derived from module naming only. This document does not assert certification, production readiness, or API stability guarantees.

| Crate | Role (inventory label) | Source | Pin commit |
|-------|------------------------|--------|------------|
| `reak-types` | Shared REAK types and common definitions | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-durable-record` | Durable record handling | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-policy-context` | Policy context | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-ues` | Unified execution surface (UES) module | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-registry` | Registry | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-replay` | Replay | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-authority` | Authority | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-exposure` | Exposure | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-commitment` | Commitment | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-dispatch` | Dispatch | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-observation` | Observation | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-reconciliation` | Reconciliation | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-truth` | Truth | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-recovery` | Recovery | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `reak-progression` | Progression | cargo metadata from pinned REAK v1.0.0 | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |

**Count:** 15 workspace crates.

**Paths in canonical REAK repo** (not present in hardening repo): see `intake/manifest.json` → `modules[].path` and `intake/connection-report.md`.
