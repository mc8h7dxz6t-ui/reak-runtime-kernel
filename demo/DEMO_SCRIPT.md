# Side-by-Side Demo Scripts — LandlordDuty vs Ground8 · CreditDuty vs Planky

Run these **before writing production code**. Each test takes ~15 minutes per competitor.

---

## Setup checklist

| Item | LandlordDuty | CreditDuty |
|------|--------------|------------|
| Competitor trial | [ground8.co.uk](https://ground8.co.uk/) or [stemhq.co.uk](https://stemhq.co.uk/) free trial | [planky.com](https://planky.com/) book demo or sandbox |
| Your input files | `landlord-duty/fixtures/demo_uc_hearing_trap.csv` | `credit-duty/fixtures/demo_payday_partial_feed.json` |
| Expected output | `landlord-duty/expected/demo_uc_hearing_trap.expected.json` | `credit-duty/expected/demo_payday_partial_feed.expected.json` |
| Screen record | Yes — 3 min max | Yes — 3 min max |
| Score sheet | Bottom of this doc | Bottom of this doc |

---

# TEST A — LandlordDuty vs Ground8/STEMHQ

## Scenario name: **"UC Direct + Hearing Paydown Trap"**

### Story (say this to competitor rep / yourself)

> Monthly rent £1,200. Tenant fell behind from March 2026. Landlord received **£950 UC housing element direct from DWP** on 28 July — wrongly logged as tenant payment. Tenant paid **£2,500 on 4 August** — hearing is **6 August**.  
> Notice was served 1 July when arrears were £3,600 (3 months).  
> **Question:** Is Ground 8 still alive at hearing?

### Input files

- Ledger CSV: `landlord-duty/fixtures/demo_uc_hearing_trap.csv`
- Tenancy config: `landlord-duty/fixtures/demo_uc_hearing_trap.json`

### Step-by-step (competitor trial)

| Step | Action | Note result |
|------|--------|-------------|
| 1 | Create tenancy: £1,200/mo monthly, start 1 Jan 2026 | |
| 2 | Import or enter payments from CSV | |
| 3 | Set notice served: **2026-07-01** | |
| 4 | Set hearing date: **2026-08-06** | |
| 5 | Ask: **Ground 8 eligible at notice date?** | Expect YES (£3,600) |
| 6 | Ask: **Ground 8 eligible at hearing date?** | **KEY QUESTION** |
| 7 | Mark £950 on 2026-07-28 — can you tag as **UC direct**, not tenant? | |
| 8 | Does £2,500 tenant payment on 2026-08-04 trigger **urgent alert**? | |
| 9 | Show **UC excluded portion** separately in schedule | |
| 10 | Export evidence PDF — is UC exclusion + allocation shown? | |

### Correct answers (LandlordDuty must produce)

See `landlord-duty/expected/demo_uc_hearing_trap.expected.json`.

| Check | Expected |
|-------|----------|
| Arrears at notice (2026-07-01) | **£3,600** — Ground 8 **ALIVE** |
| UC £950 on 2026-07-28 | Source = `HOUSING_ELEMENT_DIRECT` — reduces arrears but **not** counted as tenant credit for UC timing window |
| After UC + partial tenant payments | Running balance **£2,150** before Aug paydown |
| After £2,500 on 2026-08-04 | Balance **£0** (overpaid £350) |
| Ground 8 at hearing (2026-08-06) | **DEAD** — below 3-month threshold (£3,600) |
| Alert level | **RED** — "Ground 8 projected dead at hearing" |
| Fallback | Prompt: ensure Ground 10/11 on notice |
| UC excluded from Ground 8 calc (statutory) | **£0** at hearing — full arrears cleared, but alert should have fired **before** hearing |

### Spreadsheet trap (what agents do wrong)

If agent treats UC £950 as **tenant payment** AND doesn't see Aug £2,500 paydown:
- Spreadsheet shows ~£2,150 arrears at hearing → **false Ground 8 ALIVE** → struck out or wasted court fee

If agent treats UC correctly but **no hearing projection**:
- They discover Ground 8 dead **in court** — [Shelter: tenant can pay down before case heard](https://england.shelter.org.uk/professional_resources/legal/possession_and_eviction/grounds_for_possession/ground_8_possession)

### Score sheet — LandlordDuty wins if competitor fails ANY of:

- [ ] **A1** Cannot tag payment as UC direct / housing element
- [ ] **A2** No separate UC exclusion line in rent schedule
- [ ] **A3** No hearing-date projection (only notice-date check)
- [ ] **A4** No RED alert when paydown kills Ground 8 before hearing
- [ ] **A5** Cannot show partial payment allocation (which rent periods £2,500 applied to)
- [ ] **A6** Evidence export omits rules_version or snapshot dates

**Win threshold:** 3+ boxes checked → **build LandlordDuty wedge confirmed**

---

# TEST B — CreditDuty vs Planky

## Scenario name: **"Payday Cycle + Partial OB Feed"**

### Story (say this to Planky rep)

> Applicant applying for £8,000 motor finance. 90 days bank data.  
> **Three payday loan cycles** (credit → repayment → overdraft fee).  
> **Gambling** = 18% of surplus.  
> **One bank account failed to connect** (partial feed).  
> **Question:** Can you export a CONC 5.2A decision record as-at decision time, with rule ID per transaction, single flag per payday cycle, and **block auto-approve** on partial feed?

### Input file

- `credit-duty/fixtures/demo_payday_partial_feed.json`

### Step-by-step (Planky demo/sandbox)

| Step | Action | Note result |
|------|--------|-------------|
| 1 | Load applicant transactions (or use sandbox customer) | |
| 2 | Run affordability / decisioning | |
| 3 | Ask for **monthly affordability number** | Note the figure |
| 4 | Ask: **Show rule ID for transaction `txn_0042` (WONGA CREDIT)** | |
| 5 | Ask: **How many vulnerability flags for 3 payday cycles?** | Must be **1 pattern**, not 9 txn flags |
| 6 | Disconnect one account — **partial feed** — re-run | |
| 7 | Does system **refuse auto-approve** or show confidence downgrade? | |
| 8 | Export **CONC 5.2A reconstruction PDF** for FOS | |
| 9 | Export shows: income → essential → commitments → surplus with **CONC paragraph refs**? | |
| 10 | Export includes: `rules_version`, `feed_status`, `decision_timestamp`? | |

### Correct answers (CreditDuty must produce)

See `credit-duty/expected/demo_payday_partial_feed.expected.json`.

| Check | Expected |
|-------|----------|
| `feed_status` | `partial` (1 of 2 accounts connected) |
| Auto decision | **BLOCKED** — `manual_review_required` |
| Payday pattern instances | **3** — flags emitted: **3** (not 9+) |
| Pattern ID | `payday_cycle_v1` linking credit+repayment+fee txns |
| Gambling | `gambling_pct_of_surplus` = **0.18** → elevated, not binary |
| Surplus (full feed hypothetical) | ~**£847/mo** (see expected file) |
| CONC record sections | income_verified, essential, commitments, surplus, decision |
| FOS bundle | One-click PDF with point-in-time hash |

### Score sheet — CreditDuty wins if Planky fails ANY of:

- [ ] **B1** Only returns affordability number — no CONC structured record
- [ ] **B2** Cannot cite rule_id per transaction on request
- [ ] **B3** Payday cycle creates **>3 flags** for 3 cycles (cascade)
- [ ] **B4** Partial feed still auto-approves without confidence gate
- [ ] **B5** No FOS / audit export with decision timestamp + rules version
- [ ] **B6** Cannot reconstruct decision **as-at** past date (only live re-run)

**Win threshold:** 3+ boxes checked → **build CreditDuty wedge confirmed**

---

# Quick kill / go decision

| Result | Action |
|--------|--------|
| LD: 3+ fails on Test A | **Go** — build drop alert + UC + allocation first |
| LD: 0–2 fails | **Pivot** — wedge weak; interview 5 agencies before code |
| CD: 3+ fails on Test B | **Go** — build CONC record + confidence gate + FOS export |
| CD: 0–2 fails | **Pivot** — partner/resell Planky; don't rebuild OB |
| Both go | Build LD first (7 days), CD second (8 days) |
| Both fail | Pick different wedge or market |

---

# 3-minute demo script (record this)

## LandlordDuty (90 seconds)

1. **Open** Project Runway → "42 Oak Street" — RED badge  
2. **Say:** "Notice served 1 July — Ground 8 was alive at £3,600."  
3. **Click** case → show payment 28 July tagged **UC HOUSING_ELEMENT_DIRECT**  
4. **Say:** "Spreadsheet treats this as tenant pay — we don't."  
5. **Show** 4 Aug £2,500 → simulator: "Ground 8 **dead** at hearing 6 Aug"  
6. **Click** Export → solicitor evidence pack PDF  
7. **Say:** "Agent switches to Ground 10/11 prep **before** court, not at the door."

## CreditDuty (90 seconds)

1. **Open** applicant → feed badge **PARTIAL** — auto-decision **BLOCKED**  
2. **Say:** "One bank failed — we don't silently score."  
3. **Show** transaction list → `txn_0042` → rule `payday_lender_credit_v1`  
4. **Show** patterns panel → **3 cycles, 3 flags** (not 9)  
5. **Show** CONC record → essential £1,840 → surplus £847 → gambling 18% of surplus  
6. **Click** FOS Export → PDF with rules_version + timestamp  
7. **Say:** "This is what CONC 5.2A.24R reconstruction looks like — not a number."

---

# Next step after scoring

1. Run Test A on Ground8 **and** STEMHQ (both free trials)  
2. Run Test B on Planky live demo — **get answers on record** (email follow-up OK)  
3. Fill score sheets  
4. If GO: implement fixtures as pytest first (`tests/statutory/test_demo_*.py`)  
5. Build UI second — engine must match expected JSON exactly
