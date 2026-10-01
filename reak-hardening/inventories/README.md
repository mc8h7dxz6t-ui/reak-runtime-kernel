# Inventories

**Phase 2B status:** **CONNECTED** (connection/intake evidence only; not production qualification).

These inventories are **generated after Phase 2B connection** and **before Phase 3**. They are **documentation and evidence only**. They do **not** copy REAK source into the hardening repository, do **not** modify the canonical REAK implementation, and do **not** start Phase 3.

## Canonical REAK pin (read-only reference)

| Field | Value |
|-------|--------|
| Canonical REAK repository | `https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git` |
| REAK revision | `v1.0.0` |
| REAK commit (peeled) | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| `v1.0.0` tag object | `953ee352ebca7f44c8f6394a528e47be5534bed7` |

## Programme evidence commits (hardening repo)

| Field | Value |
|-------|--------|
| Phase 2B connected evidence commit | `8be8fc392888040b7666b10be3eed969598989eb` |
| Phase 2B manifest/evidence SHA metadata commit | `0c68fb73eb12768d61d0915d3292812b718a1340` |

## Artefacts in this directory

| File | Purpose |
|------|---------|
| [REAK_MODULE_INVENTORY.md](REAK_MODULE_INVENTORY.md) | Fifteen `cargo metadata` workspace crates at pinned REAK `v1.0.0` |
| [REAK_PIN_INVENTORY.json](REAK_PIN_INVENTORY.json) | Machine-readable pin and module list |
| [PHASE3_PREREQUISITE_INVENTORY.md](PHASE3_PREREQUISITE_INVENTORY.md) | Preconditions and explicit non-claims before Phase 3 |

Additional inventories (for example dependencies, public API, trust boundaries) may be added under Phase 3 charter; they are **not** required for this pre–Phase 3 baseline.
