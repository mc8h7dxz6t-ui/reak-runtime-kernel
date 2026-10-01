# Phase 3 operator charter (programme opening)

## Charter identity

| Field | Value |
| --- | --- |
| Programme | REAK Hardening |
| Phase | **3** — review hardening (adversarial engineering against pinned REAK) |
| Lane | **B** — hardening workspace (`reak-hardening/`) |
| Operator instruct | Priority 1 lane B: open Phase 3 hardening (charter in hardening repo) |
| Doctrine board context | `@88aac94` (programme coordination; no Phase 3 prompt file in doctrine folder) |
| Hardening programme origin (pre-charter) | `10f9e451d5de534ba65769f165e1597508bd8a5d` (`docs(hardening): add phase 2b inventories`) |
| Charter document status | **Draft for review** — effective only after operator acceptance and committed gate record |
| Adversarial execution | **Not started** by this charter alone (see [PHASE3-GATE.md](PHASE3-GATE.md)) |

## Mission (Phase 3 bounded)

Execute **review hardening** against the **pinned canonical REAK** implementation: adversarial analysis, findings, and programme evidence under [00-programme-charter.md](00-programme-charter.md) review dimensions. Work product lives in this **programme repository** (reports, module reviews, gates). It does **not** redesign frozen architecture, add product features, or imply qualification or release.

## Prerequisites (satisfied before opening)

| Prerequisite | Evidence |
| --- | --- |
| Phase 2B **CONNECTED** | [PHASE2B-GATE.md](PHASE2B-GATE.md), [intake/manifest.json](intake/manifest.json) |
| Pre–Phase 3 inventories | [inventories/README.md](inventories/README.md), [REAK_PIN_INVENTORY.json](inventories/REAK_PIN_INVENTORY.json), [PHASE3_PREREQUISITE_INVENTORY.md](inventories/PHASE3_PREREQUISITE_INVENTORY.md) |
| REAK pin recorded | `v1.0.0` / `65cea8f8921909bd44b970d1a69b2da362d7a4a8` (tag object `953ee352ebca7f44c8f6394a528e47be5534bed7`) |
| REAK source in hardening repo | **Absent** — read-only consumption model preserved |

Phase 2B connected evidence commits remain: `8be8fc392888040b7666b10be3eed969598989eb`, manifest evidence metadata `0c68fb73eb12768d61d0915d3292812b718a1340`.

## Canonical REAK pin (review target)

| Field | Value |
| --- | --- |
| Repository | `https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git` |
| Revision | `v1.0.0` |
| Peeled commit | `65cea8f8921909bd44b970d1a69b2da362d7a4a8` |
| Tag object | `953ee352ebca7f44c8f6394a528e47be5534bed7` |
| Workspace crates in scope | **15** — see [inventories/REAK_MODULE_INVENTORY.md](inventories/REAK_MODULE_INVENTORY.md) |
| Access mode | **Read-only** checkout or CI equivalent at pin; **no** copying REAK source tree into this repository |

Parallel programme work (e.g. REAK-V1-PRESERVATION lane A) does not change this pin unless a **separate** operator pin revision is recorded.

## What Phase 3 includes

1. **Module-level adversarial review** per [01-module-review-template.md](01-module-review-template.md) and charter dimensions in [00-programme-charter.md](00-programme-charter.md).
2. **Evidence-backed findings** with verdict vocabulary (**CONFIRMED**, **HYPOTHESIS**, **NOT RUN**, **NOT APPLICABLE**).
3. **Programme artefacts** under `reak-hardening/reports/`, `reak-hardening/modules/`, and Phase 3 gate updates as review progresses.
4. **Optional extended inventories** (dependencies, public API surfaces, trust boundaries) when chartered — not required to open Phase 3.

## What Phase 3 excludes (stop lines)

See [phase3/STOP-LINES.md](phase3/STOP-LINES.md). Summary:

- **No Phase 10** (or other unchartered programme phases) — Phase 3 does not authorize Phase 10 work.
- **No REAK source** committed into the hardening repository.
- **No writes** to canonical REAK kernel source unless a **future** operator instruct explicitly scopes a patch lane (not in this charter).
- **No production** deployment, **no production qualification**, **no certification**, **no customer deployment**.
- **No adapter**, **provider**, or **cloud integration** deliverables from this phase.
- **No** Qualification Kernel / EvidenceLab execution claims — those are separate programmes ([programme-layout.md](programme-layout.md)).

## Execution boundary

| State | Meaning |
| --- | --- |
| **Phase 3 programme OPENED** | Operator charter accepted; review hardening is **authorized** at programme level. |
| **Adversarial execution NOT STARTED** | **Tier 3** module review, HOSTILE template execution, and CONFIRMED adversarial findings are **not** in scope until [PHASE3-GATE.md](PHASE3-GATE.md) is committed **OPENED** and the operator issues an **explicit Tier 3 execution instruct** (see [phase3/TEST-EXECUTION-PLAN.md](phase3/TEST-EXECUTION-PLAN.md)). |
| **Mechanical / programme tiers (0–2)** | Tier 0 programme evidence replay, Tier 1 PHASE2B-RERUN, and Tier 2 bounded Phase 9 bootstrap tests may be planned or run per the test plan **without** starting Tier 3 adversarial execution. |

Opening this charter **does not** amend Phase 2B **CONNECTED**. Phase 2B remains the connection/intake gate; Phase 3 is the review gate ([PHASE3-GATE.md](PHASE3-GATE.md)).

## Rigorous testing (plan)

Tiered test execution is defined in [phase3/TEST-EXECUTION-PLAN.md](phase3/TEST-EXECUTION-PLAN.md) (plan only; no execution implied by charter text alone).

## Operator acceptance

| Role | Action |
| --- | --- |
| Operator (Philip) | Accept charter text and commit gate package when satisfied |
| Programme coordinator | Track lane A (preservation) vs lane B (hardening) without cross-claiming |

**Signature / date:** _Pending operator acceptance on commit._
