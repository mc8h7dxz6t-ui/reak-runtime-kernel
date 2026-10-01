# Phase 3 test execution plan (tiers — plan only)

**Status:** Programme documentation. **No tier execution is implied by this file alone.**

**Adversarial review execution (Tier 3):** **NOT STARTED** until [PHASE3-GATE.md](../PHASE3-GATE.md) is **committed** with verdict **OPENED** **and** the operator issues an **explicit Tier 3 execution instruct**.

**Rigorous testing roadmap:** Tiers are ordered by dependency and trust boundary. Lower tiers may run without opening Tier 3. See [PHASE3-OPERATOR-CHARTER.md](../PHASE3-OPERATOR-CHARTER.md) and [STOP-LINES.md](STOP-LINES.md).

| Tier | Name | REAK checkout | Charter / gate |
| --- | --- | --- | --- |
| 0 | PROGRAMME_EVIDENCE_REPLAY | No | Programme repo only |
| 1 | PHASE2B-RERUN | Read-only at pin | Phase 2B CONNECTED sufficient |
| 2 | Phase 9 bootstrap branch (bounded slice) | Read-only at operator-recorded ref | Operator-recorded ref + bounded scope doc |
| 3 | Phase 3 adversarial review (HOSTILE templates) | Read-only at pin (unless instruct says otherwise) | **Committed OPENED gate + explicit execution instruct** |
| 4 | Qualification Kernel | External programme repo | Separate operator charter; not lane B |

---

## Tier 0 — PROGRAMME_EVIDENCE_REPLAY

**Purpose:** Replay the **doctrine / programme evidence contract** without touching REAK source — prove gates, pins, and inventories are internally consistent on the hardening `main` line.

### Entry criteria

- Hardening programme repo checked out at the SHA under test (e.g. post-inventory `10f9e45…` or post-charter commit when recorded).
- No REAK clone required.

### Commands (representative)

```bash
cd /workspace   # programme root; adjust to local clone

git rev-parse HEAD
test -f reak-hardening/PHASE2B-GATE.md
test -f reak-hardening/intake/manifest.json
test -f reak-hardening/inventories/REAK_PIN_INVENTORY.json
python3 -m json.tool reak-hardening/inventories/REAK_PIN_INVENTORY.json >/dev/null

# Pin contract (expected at Phase 2B inventories baseline)
python3 - <<'PY'
import json
d = json.load(open("reak-hardening/inventories/REAK_PIN_INVENTORY.json"))
assert d["canonical_reak_commit_sha"] == "65cea8f8921909bd44b970d1a69b2da362d7a4a8"
assert d["modules_count"] == 15
assert d["implementation_present_in_hardening_repo"] is False
PY

test ! -d runtime-execution-assurance-kernel
```

When Phase 3 charter is committed, extend replay to assert `PHASE3-GATE.md` documents **OPENED** and adversarial execution **NOT STARTED** until Tier 3 instruct.

### Pass artefacts

- Console log or CI job output with exit code **0**.
- Optional: `reak-hardening/phase3/evidence/tier0-programme-replay-<date>.md` (summary only; not required before charter commit).

### Fail artefacts

- Non-zero exit; recorded mismatch on pin SHA, `modules_count`, or unexpected REAK tree in programme repo.

### Non-claims

- Does **not** verify REAK builds, tests, or security properties.
- Does **not** open Phase 3 adversarial execution.
- No production, certification, customer deployment, adapter/provider/cloud, or Qual Kernel pass.

---

## Tier 1 — PHASE2B-RERUN (pinned mechanical verification)

**Purpose:** Re-run Phase 2B **connection/intake** mechanical bar on a **fresh read-only checkout** at the canonical pin (`PHASE2B-RERUN` contract).

### Entry criteria

- Phase 2B **CONNECTED** ([PHASE2B-GATE.md](../PHASE2B-GATE.md)).
- Toolchain meets or exceeds recorded verification (rustc/cargo **1.99.0** at last CONNECTED evidence; document actual versions in artefact).
- Checkout path **outside** programme repo (e.g. `/tmp/reak-runtime-phase2b-rerun-<id>`).

### Commands

Per [intake/connection-report.md](../intake/connection-report.md) and [intake/BUILD.md](../intake/BUILD.md):

```bash
REPO=https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git
PIN=65cea8f8921909bd44b970d1a69b2da362d7a4a8
WORKDIR=/tmp/reak-runtime-phase2b-rerun-$(date +%Y%m%d)

git ls-remote "$REPO" refs/heads/main refs/tags/v1.0.0 'refs/tags/v1.0.0^{}'
rm -rf "$WORKDIR"
git clone --no-checkout "$REPO" "$WORKDIR"
cd "$WORKDIR" && git checkout "$PIN"
cd runtime-execution-assurance-kernel
cargo metadata --no-deps --format-version 1
cargo build --workspace --locked
cargo test --workspace --locked
```

### Pass artefacts

- `git ls-remote` output showing `main` and peeled tag at `65cea8f…` / tag object `953ee352…`.
- **SUCCESS** for metadata, build, test (exit 0).
- Optional committed summary: `reak-hardening/phase3/evidence/tier1-phase2b-rerun-<date>.md` (toolchain versions, `WORKDIR`, command transcript reference).

### Fail artefacts

- Ref mismatch, build/test failure, or checkout not at `PIN`.

### Non-claims

- **Not** qualification, certification, or production readiness.
- **Not** Phase 3 adversarial review (Tier 3).
- **Not** authorization to modify REAK source.
- Does **not** set `phase3_started` or start HOSTILE module reviews.

---

## Tier 2 — Phase 9 bootstrap branch tests (bounded slice)

**Purpose:** Run a **bounded** test slice on an operator-recorded **bootstrap branch** on the canonical REAK repository (Phase 9 programme alignment), without expanding scope to full Phase 9 or Phase 10.

### Entry criteria

- Operator records **branch name** and **commit SHA** (and bounded test filter) in programme evidence — e.g. lane A preservation / doctrine board output. **No default branch is assumed in this plan.**
- Ref must be reachable read-only on `reak-runtime-kernel` (or `reak-runtime` remote).
- Bounded scope written: crate list, test name filter, or manifest snippet (max crates / max duration agreed with operator).
- Tier 1 **recommended** at pin `v1.0.0` before or in parallel; Tier 2 does not replace pin evidence.

### Commands (template — fill ref and filter from operator record)

```bash
REPO=https://github.com/mc8h7dxz6t-ui/reak-runtime-kernel.git
BRANCH=<operator-recorded-branch>
SHA=<operator-recorded-commit>
WORKDIR=/tmp/reak-phase9-bootstrap-$(date +%Y%m%d)

rm -rf "$WORKDIR"
git clone --no-checkout "$REPO" "$WORKDIR"
cd "$WORKDIR" && git checkout "$SHA"
cd runtime-execution-assurance-kernel
# Bounded slice only — example placeholders:
# cargo test -p reak-types --locked
# cargo test <operator-filter> --locked -- --test-threads=1
```

### Pass artefacts

- Recorded `BRANCH`, `SHA`, filter, toolchain versions, exit code **0** for bounded commands.
- Optional: `reak-hardening/phase3/evidence/tier2-phase9-bootstrap-<date>.md`.

### Fail artefacts

- Checkout failure, test failure, or scope creep beyond recorded bounded slice.

### Non-claims

- **Not** full Phase 9 completion, **not** Phase 10, **not** hardening qualification.
- **Not** Tier 3 HOSTILE review.
- Branch work does **not** authorize copying REAK into the hardening repo.

---

## Tier 3 — Phase 3 adversarial review (HOSTILE programme templates)

**Purpose:** Module-level adversarial review, hostile test design, and findings per [00-programme-charter.md](../00-programme-charter.md), [01-module-review-template.md](../01-module-review-template.md), and [02-test-catalogue.md](../02-test-catalogue.md) (property, fuzz, concurrency, crash, soak, fault injection, corruption, exhaustion — executed or recorded as **NOT RUN** / **HYPOTHESIS** with evidence).

### Entry criteria (gate)

1. [PHASE3-OPERATOR-CHARTER.md](../PHASE3-OPERATOR-CHARTER.md) **committed** and [PHASE3-GATE.md](../PHASE3-GATE.md) verdict **OPENED** on programme `main`.
2. **Explicit operator execution instruct** naming Tier 3 kickoff (module list, pin confirmation, artefact paths).
3. Read-only REAK checkout at pin `65cea8f8921909bd44b970d1a69b2da362d7a4a8` (or instruct-specified SHA with recorded pin revision).
4. Tier 0 **recommended**; Tier 1 **recommended** on same pin before first HOSTILE findings claim **CONFIRMED** on build/test behaviour.

### Commands / activities (plan)

- Per-module copies under `reak-hardening/modules/<crate>.md` from template.
- Reports under `reak-hardening/reports/` (security, concurrency, reliability, testing gaps as applicable).
- Test execution per catalogue IDs where implemented; otherwise document **NOT RUN** with blocker.
- Optional read-only checkout commands same as Tier 1 for code inspection (no writes to REAK).

### Pass artefacts

- Committed module reviews with verdict vocabulary (**CONFIRMED** / **HYPOTHESIS** / **NOT RUN** / **NOT APPLICABLE**).
- Phase 3 gate or progress table updated when milestones defined (no **COMPLETE** verdict unless separately chartered).
- Optional execution log: `reak-hardening/phase3/evidence/tier3-execution-kickoff-<date>.md` referencing operator instruct.

### Fail artefacts

- Tier 3 work attempted without committed charter + instruct.
- Findings stated as **CONFIRMED** without evidence citation.
- REAK source modified in hardening repo or unapproved write to canonical REAK.

### Non-claims

- **Not** production qualification, certification, or customer deployment.
- **Not** adapter, provider, or cloud integration.
- **Not** Qualification Kernel (Tier 4) or EvidenceLab execution.
- HOSTILE template work **does not** imply all catalogue tests were run.

### Tier 3 gate link

| Gate | Requirement |
| --- | --- |
| [PHASE3-GATE.md](../PHASE3-GATE.md) | **OPENED** (committed) — programme Phase 3 authorized |
| Adversarial execution row | Moves from **NOT STARTED** to **STARTED** only on operator Tier 3 instruct + kickoff artefact |
| [STOP-LINES.md](STOP-LINES.md) | Remain in force for all Tier 3 work |

---

## Tier 4 — Qualification Kernel (external programme)

**Purpose:** Hostile corpus, fuzz/chaos infrastructure, and qualification execution in the **Qualification Kernel** repository per [programme-layout.md](../programme-layout.md).

### Entry criteria

- Separate Qual Kernel programme charter and pin file (`templates/HARDENING_PIN.json` pattern).
- REAK consumed as specimen under test; hardening lane B does not substitute for Qual Kernel runs.

### Commands

- Defined in Qual Kernel repo (out of scope for this document).

### Pass artefacts

- Qual Kernel programme evidence (external repo commits / reports).

### Fail artefacts

- Claiming Qual Kernel pass from hardening repo artefacts only.

### Non-claims

- Hardening Phase 3 **OPENED** or Tier 3 **STARTED** does **not** mean Qual Kernel ran or passed.
- No certification or production claim from Tier 4 without EvidenceLab / issuer programme (separate).

---

## Summary

| Tier | Execution state (default) |
| --- | --- |
| 0–2 | May be planned or run under operator / CI instruct without Tier 3 adversarial **STARTED** |
| 3 | **NOT STARTED** until committed **OPENED** gate + explicit execution instruct |
| 4 | External programme; not started from this repo |

Update [PHASE3-GATE.md](../PHASE3-GATE.md) adversarial execution row when Tier 3 kickoff is recorded (on commit, when instructed).
