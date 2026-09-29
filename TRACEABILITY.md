# Constitutional traceability matrix

Status: frozen as a trace, not as a design.
Binds to: Constitution Part 7, clauses 1–11; CCS-1 scenario map; EvidenceLab constitutional design v1.
This matrix does not add articles, components, or duties.

Ownership rule used here: a component is named only if a frozen contract already names it. The constitution does not name runtime components. The only named components are EvidenceLab’s Sealer, Instruments, Assessor, Issuer, and Verifier. They do not authorize or release effects. Where no frozen contract names a component for an obligation, the status is NO OWNER. That is a fact about the contracts. It is not a request for a new component.

Status values:

- IMPLEMENTED: a specimen in the reviewable record meets the article under CCS evidence rules. None do.
- PLANNED: a frozen contract or the qualification roadmap assigns the work, and names the responsible contract component where one exists, but no execution evidence exists.
- NO OWNER: no frozen contract assigns a component.
- UNKNOWN: not used. The record is sufficient to distinguish “no component named” from “a component may exist off the record”. Off-record testimony is not an owner.

## Part 1 — Complete matrix

| Article | Obligation in one line | Responsible component | CCS scenarios | Qualification evidence required | Status |
| --- | --- | --- | --- | --- | --- |
| 1 | A durable authorization names the effect and the rule version before a new external effect | NO OWNER. No runtime component is named in the frozen contracts. EvidenceLab must not perform this release. | H01, H02, H14, H22, H26, H29, H31, H32, H34, H35, H38 | Second-party copy of the authorization showing it preceded the effect; rule id; instrument record of the effect; wire bytes where the scenario mutates or bypasses; manual path record if used | NO OWNER |
| 2 | Authorizations, releases, observations, qualifications, and withdrawals are append-only | NO OWNER for the specimen history. EvidenceLab Sealer is the named component for the qualification corpus only, and only once that lab is built. | H02, H10, H18, H20, H21, H23, H37, H38, H39 | A second party’s earlier bytes compared with the later copy; any correction present only as a new record; for a qualification corpus, the Sealer’s manifest hash | NO OWNER for specimen history. Sealer: PLANNED, not built |
| 3 | At most one authority releases a new effect for one authorization at a time | NO OWNER | H05, H06, H21, H27, H34, H35, H37, H39 | Overlap or crash point; both attempt records; instrument count of effects; successor’s view of the history | NO OWNER |
| 4 | An undetermined outcome does not authorize another release unless a still-valid qualified harmless-duplicate rule says so | NO OWNER | Mandatory: H03, H04, H05, H06, H07, H08, H19, H26, H27, H30, H34, H35, H36, H37, H38, H39, H50 as a bound on later releases. Optional, mandatory only if claimed: H44, H45 | Instrument apply count; sealed classification of the attempt; absence of a second release; if the exception is claimed, the rule qualification and an instrument that is not self-report | NO OWNER |
| 5 | A further consequential effect, including a correction, has its own authorization | NO OWNER | H07, H14, H28, H29, H32 | Instrument records of each effect; a prior authorization naming each one; channel list for side effects | NO OWNER |
| 6 | Admit, satisfy, and qualify are functions of sealed records and a named rule, with no hidden live input | NO OWNER for the specimen’s admit and satisfy decisions. EvidenceLab Assessor is the named pure function for a qualification verdict, not for admitting an effect. | H04, H09, H11, H14, H15, H18, H23, H24, H25, H26, H27, H36 | Sealed inputs, rule text, specimen decision, recomputation, and the perturbed unsealed value when H24 applies | NO OWNER for admit and satisfy. Assessor: PLANNED, not built |
| 7 | Unknown stays distinguishable from known; conflicts and gaps remain | NO OWNER for the specimen’s reader-visible outcome. EvidenceLab coverage inside the signed assessment is the named form for a qualification verdict. | H03, H04, H07, H08, H09, H10, H11, H12, H13, H15, H16, H17, H19, H21, H30, H38, H40, H42 | Observations, conflicts, the reader-visible claim, and coverage that lists gaps; both sides of a conflict still present | NO OWNER for specimen outcomes. Qualification coverage: PLANNED, not built |
| 8 | Someone other than the issuing service can recompute the decision | NO OWNER for recomputation of admit and satisfy. EvidenceLab Verifier is the named offline checker for a qualification verdict. The Issuer must not be required. | H20, H25 | Records and rule handed over; checker output produced without the issuer process; hash match after tamper | NO OWNER for admit and satisfy. Verifier: PLANNED, not built |
| 9 | A qualification names claim, specimen, rule, corpus, and limit; withdrawal is a new record; an unchecked withdrawal head is not current validity | EvidenceLab Issuer signs and withdraws. Verifier checks expiry and the head. No runtime component may issue this and have it count as independent. | H10, H16, H20, H22, H31, H32, H33, H40, H41, H42, H43, H44, H45 | Qualification bytes, corpus hash, limit text, withdrawal record, checker result with the head reachable and unreachable, second copy of the original bound | Issuer and Verifier: PLANNED, not built |
| 10 | The specimen’s own records cannot be the sole support of an independent qualification of that specimen | EvidenceLab Instruments (or a named third party) produce supports that are not the specimen’s authority. Assessor refuses a self-report-only corpus. Issuer must not sign one. | Mandatory: H13, H33, H42. Optional with a harmless-duplicate claim: H44. Research, not required for a pass: H48 | Producer identity of every supporting record; key separation; the refusal, or a certificate whose supports are not solely the specimen | Instruments, Assessor, Issuer: PLANNED, not built |
| 11 | The contract does not promise exactly-once external apply, honesty before first seal, or halt recalling a released effect | No runtime component. The qualification issuer, which for an independent claim is the EvidenceLab Issuer, and the CCS report, must not assert these. | H03 and H31 as residuals that must not be scored as implementation failures; H43, H49, H50 as bound checks on the wording | The qualification text and the CCS report. Pass is the absence of the three claims and the presence of the bound. | Report rule: PLANNED in CCS-1 text, not executed. Issuer: PLANNED, not built |

H46 and H47 are not rows. They verify no article. H48 does not add mandatory coverage to article 10.

Nothing in this table is IMPLEMENTED. Testimony that Truth, ID-008, or ID-009 is done does not name a component against an article and is not a status.

## Part 2 — Missing owners

NO OWNER, and no frozen contract supplies one:

- Article 1, the whole obligation.
- Article 2, specimen history. The Sealer does not own it.
- Article 3, the whole obligation.
- Article 4, the whole obligation.
- Article 5, the whole obligation.
- Article 6, admit and satisfy. The Assessor does not admit effects.
- Article 7, the specimen’s reader-visible outcome.
- Article 8, recomputation of admit and satisfy. The Verifier does not replace that handover.

These missing owners do not make the articles impossible. The constitution left runtime shape unnamed on purpose. An implementation assigns its own code. This matrix must not invent the names.

Articles 9, 10, and the qualification half of 2, 6, 7, 8, and 11 have owners in the EvidenceLab constitution. Those owners are not built. That is PLANNED, not NO OWNER, and not IMPLEMENTED.

## Part 3 — Missing CCS coverage

No article lacks a verifying scenario.

| Article | Mandatory coverage | Gap |
| --- | --- | --- |
| 1 | H01, H02, H14, H22, H26, H29, H31, H32, H34, H35, H38 | None |
| 2 | H02, H10, H18, H20, H21, H23, H37, H38, H39 | None |
| 3 | H05, H06, H21, H27, H34, H35, H37, H39 | None |
| 4 | H03–H08, H19, H26, H27, H30, H34–H39, H50 | None. H44 and H45 are required only when a harmless-duplicate rule is claimed. Their absence from a specimen that does not claim the exception is not a coverage hole. |
| 5 | H07, H14, H28, H29, H32 | None |
| 6 | H04, H09, H11, H14, H15, H18, H23–H27, H36 | None |
| 7 | H03, H04, H07–H13, H15–H17, H19, H21, H30, H38, H40, H42 | None |
| 8 | H20, H25 | None |
| 9 | H10, H16, H20, H22, H31–H33, H40–H45 | H44 and H45 apply only if that qualification exists. The article is still covered by H40–H43 without them. |
| 10 | H13, H33, H42 | None. H48 is research and is not required to trace the article. |
| 11 | H43, H49, H50, with H03 and H31 keeping residuals out of the fail column | None. These scenarios verify the bound. They do not verify an obligation to do the unpromised thing. |

No new CCS scenario is required to trace the constitution.

## Part 4 — Qualification evidence gaps

The required evidence is defined in Part 1. None of it has been produced for any article.

| Article | What is missing | What this is not |
| --- | --- | --- |
| 1 | No second-party authorization copy, no instrumented effect, no wire capture | Not a missing scenario |
| 2 | No before-and-after byte comparison for specimen history. No Sealer manifest. | Not a missing history mechanism in the constitution |
| 3 | No overlap or crash run with an effect count | Not a missing consensus article |
| 4 | No lost-ack or ambiguity run with an instrument count. No harmless-duplicate qualification either, so the exception is not in force. | Not a requirement to claim the exception |
| 5 | No paired authorizations for a corrective effect, because no such run exists | Not a requirement to build undo |
| 6 | No sealed input set and no matching recomputation | Not a second implementation of the checker |
| 7 | No reader-visible outcome captured beside a gap or a conflict | Not a new verdict word |
| 8 | No offline checker run | Not a live service |
| 9 | No qualification bytes, no withdrawal, no head-down checker result | Not a new certificate format |
| 10 | No corpus whose producer keys are shown to be distinct from the specimen, and no refusal on a self-report-only corpus | Not a pass by the specimen’s own log |
| 11 | No issued report whose wording can be checked for the three forbidden claims | Not a test that halt recalls a packet |

Until those captures exist, every article is unqualified. A roadmap stage that exits without the evidence named in its row has not qualified the article.

## Part 5 — Impossibility

No article is impossible to satisfy.

Clauses 1–10 are obligations an implementation can meet with durable records, one releaser, restraint after ambiguity, separate authorizations for further effects, sealed inputs, visible unknowns, a handed-over recomputation, and a bounded qualification. Clauses 9 and 10 are met by the EvidenceLab duties already frozen, once built, or by a refusal where independence is not claimed. Clause 11 is satisfied by not making the three promises. H49 and H50 are passes of the bound when the report tells the truth about what was not proved.

Missing owners are an assignment gap, not an impossibility. The constitution forbids this board from filling that gap with new components.

## Freeze

**FREEZE** this traceability matrix.

Every article has CCS coverage. Every article has a stated evidence requirement. No article requires a new component or a new constitutional sentence. Ownership gaps stay recorded as NO OWNER until an implementation exists and is tested. They are not a reason to reopen the constitution, CCS-1, or the EvidenceLab constitution.
