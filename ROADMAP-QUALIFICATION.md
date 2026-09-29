# Implementation roadmap qualification

Board role: Architecture Qualification Board.
Normative contracts, treated as immutable: Constitution Part 7 (`CONSTITUTION.md`), CCS-1 (`CCS.md`), EvidenceLab constitutional design v1 (`EVIDENCELAB.md`).
This document does not amend them and does not add runtime duties.

## Facts and testimony

Facts the board can inspect in this programme record:

- The constitution, CCS-1, and the EvidenceLab constitution are written and frozen as contracts.
- CCS-1 states that system testimony cannot pass a scenario, and that a clause passes only when every mandatory scenario mapped to it has admissible evidence.
- No runtime, no test corpus, no CCS execution report, no EvidenceLab certificate, and no records named Truth, ID-008, ID-009, or IQ-009 are in the reviewable tree.

Testimony, not evidence: Truth is qualified; Canonical Recovery (ID-008) is implemented and qualified; SafeNext (ID-009) is implemented and IQ-009 is not complete; assembly, CCS execution, 005→009 through Commitment/Dispatch, and independent EvidenceLab qualification have not been done.

The board uses the testimony only as a statement of what is claimed. It does not treat those claims as CCS passes.

## Part 1

**YES.** Implementation roadmaps replace architectural debate.

Evidence for stopping architecture:

- The constitution already says an implementation that keeps clauses 1–10 may choose language, storage, and topology, and that research programmes are not on the critical path.
- CCS-1 already defines how those clauses are falsified. The missing work is execution of that suite, not a new suite and not a new runtime shape.
- No inspected gap makes a clause impossible to implement. The gaps are absent evidence and absent stages the contracts already name: durable authorization before release, single releaser, unknown not cleared by retry, append-only history, recomputation, bounded qualification, and independent qualification not resting on self-report.
- Architecture changes are not indicated. None of the claimed gaps require a new duty.

What this yes does not say: it does not say Truth, ID-008, or ID-009 are qualified. Those claims have no admissible record here. A roadmap may list them only as unverified inputs.

Architectural work stops. Implementation and qualification of the existing contracts begin.

## Part 2 — Blockers before a roadmap freeze

**Constitutional blockers.** None. Nothing in the claimed gaps makes clauses 1–10 impossible. Clause 11 remains a bound, not a missing feature.

**CCS blockers.** None in the specification. CCS-1 is frozen. There is no executed CCS report. That is a qualification blocker, not a reason to edit CCS.

**Implementation blockers.**

- Commitment before any new external effect is not shown in any artifact the board can read. Dispatch cannot be qualified ahead of it.
- A single releaser for one authorization is not shown.
- The behaviour that an undetermined outcome does not release again is not shown against an instrument.
- ID-009 has no completed IQ-009, by the programme’s own testimony, so SafeNext is not a qualified input.

**Qualification blockers.**

- No admissible evidence for the Truth qualification or the ID-008 qualification. Isolated testimony does not compose into an assembly pass (CCS Part 8).
- No hostile end-to-end assembly run.
- No CCS-1 execution against a specimen.
- No 005→009 run through Commitment and Dispatch. The board does not have a definition of 005 in the contracts; the missing integrated run through Commitment and Dispatch is the blocker, whatever 005 names.
- No independent qualification corpus that refuses self-report (clause 10, CCS H42, EvidenceLab constitution).

**Research items.** Not blockers. Constitution Part 8, R1–R6, and CCS H46–H48, H49–H50 as bounds. They do not gate the roadmap freeze. R1 is mandatory inside CCS only for a specimen that claims harmless duplicates. R2’s drills are already mandatory CCS scenarios (H06, H34), not optional research.

## Part 3 — Pipeline order

The proposed order is not correct as a qualification order.

```
Truth → Recovery → SafeNext → Commitment → Dispatch → Observation
→ Reconciliation → Assembly → CCS → EvidenceLab → Qualification
```

What is wrong:

- CCS-1 is already specified. Placing “CCS” only after assembly treats the suite as a late product. Execution of the scenarios that belong to a stage is that stage’s exit. A single terminal CCS run is still required. It is not the first time a clause is tested.
- Commitment is clause 1. It cannot sit after Release-like work (SafeNext, Recovery) if those stages cause or re-issue external effects. Recovery qualified without Commitment and Dispatch has not met H34, H35, H37, H38, or H39.
- Observation is required before any decision that a rule is satisfied (clauses 6 and 7). Reconciliation is not a later constitutional stage. CCS already treats push and pull as one append. A separate Reconciliation stage may exist as implementation work, but it cannot be the first time disagreement is admissible, and it must not follow Assembly.
- EvidenceLab’s constitution forbids it from operating or authorizing the specimen. It is a qualification activity for clause 10, after a specimen and a sealed corpus exist. It is not an implementation predecessor of Commitment or Dispatch.
- “Truth” is not a constitutional stage. The constitution’s wording is that a rule is satisfied by records, not that the world is settled. A module of that name must not be read as proving clause 11. The board does not reorder or rename that module. It refuses to treat a Truth qualification as evidence about later stages.

Corrected qualification order, using existing names only:

1. Contracts already frozen: Constitution, CCS-1, EvidenceLab constitution.
2. Evidence admission of claimed work: Truth, ID-008, ID-009 / IQ-009. Admit, or mark unqualified. No new design.
3. Commitment, with the CCS scenarios for clauses 1, 2, and 6 that do not need a second effect.
4. Dispatch bound to that commitment, including single releaser and no second release from unknown (clauses 3 and 4). Scenarios include H01–H08, H26, H27, H34.
5. Observation intake, including conflict, gap, delay, and replay of the same bytes (clauses 2, 6, 7). Reconciliation scenarios (H19) run in this intake, not after assembly.
6. Recovery and restart re-run against that dispatch (H35, H37, H38, H39). Prior ID-008 qualification, even if later admitted, does not substitute for this run.
7. Hostile assembly of the path the programme calls 005→009 through Commitment and Dispatch.
8. CCS-1 mandatory execution on that assembly, one report, second copy retained.
9. Independent qualification under the EvidenceLab constitution: H42 and a corpus that is not solely the specimen’s self-report. Failure to qualify independently is a failed clause 10 claim, not a failed dispatch implementation.
10. One bounded qualification record. It withdraws by a new record. It does not edit earlier reports.

SafeNext stays where the programme put it only as an implementation task whose IQ-009 exit is still open. It is not a constitutional predecessor of Commitment. It must not release a second effect from an unknown outcome. Until IQ-009 exists as admissible evidence, the roadmap lists ID-009 as implemented-by-testimony and unqualified.

## Part 4 — Remaining gaps

**Claimed Truth qualification has no admissible record.**
Exists because the board was given testimony and no corpus. Risk: later stages inherit a pass the constitution would call knowledge-of-records, or worse a claim of world-truth. Blocks implementation: no. Blocks qualification of anything that cites it: yes. Blocks a commercial pilot that cites it: yes. Blocks production: yes, if production claims rest on it.

**ID-008 Recovery qualified in isolation, by testimony.**
Exists because failover and restore were not run through Commitment and Dispatch in any artifact here. Risk: a successor releaser sends a second effect (clauses 3 and 4). Blocks starting Commitment and Dispatch: no. Blocks assembly qualification and any production claim of recovery: yes. Blocks a pilot that restarts or restores: yes. Blocks a pilot that never restarts: no, and that limit must be written on the pilot qualification (clause 9).

**ID-009 implemented, IQ-009 not done.**
Exists by testimony. The contracts do not define SafeNext or IQ-009, so the board cannot invent the missing test. Risk: an unqualified “next” step clears unknown or releases again. Blocks other implementation: no, unless that code path releases under an existing authorization. Blocks qualification of ID-009: yes. Blocks a pilot that uses that path: yes. Blocks production use of that path: yes.

**No Commitment / Dispatch assembly (005→009).**
Exists because those stages are not in the tree and are reported as not qualified. Risk: this is clause 1 and clause 3 unimplemented or at least unshown. Blocks implementation of later qualification: yes, in the sense that qualification cannot skip them. Blocks writing the code: no; this is the implementation. Blocks pilots and production that cause external effects: yes.

**No hostile end-to-end assembly.**
Exists because module claims, even if later admitted, do not compose (CCS Part 8). Risk: interactions H03, H06, H07, H21, H34 fail only when combined. Blocks a roadmap that stops at module exits: yes. Blocks production: yes. Blocks a pilot that claims the assembly: yes. Does not block starting the implementation of Commitment.

**No CCS execution.**
Exists because CCS-1 is a specification. Risk: conformance is claimed from the document rather than from evidence. That would be a qualifier failure (CCS Part 11). Blocks qualification: yes. Blocks production: yes. Blocks pilots that claim the constitution: yes. Blocks coding: no.

**No independent EvidenceLab qualification.**
Exists because clause 10 and the EvidenceLab constitution require a trust root the specimen does not hold, and no such corpus is present. Risk: operational logs are presented as independent qualification. Blocks dispatch implementation: no. The EvidenceLab constitution says the lab does not operate the specimen. Blocks any claim of independent qualification, including a production claim of that kind: yes. A pilot that does not make that claim is not blocked by this gap; the qualification must say so.

## Part 5 — Minimum remaining roadmap

No research programmes. Only work the contracts already require and the record does not show.

**M1. Admit or drop the claimed passes.**
Prerequisite: the frozen contracts. Deliverable: for Truth, ID-008, and ID-009, either the sealed records CCS Part 6 would accept, or an explicit statement that the claim is not qualified. Exit: no roadmap line cites an unadmitted pass. Qualification: this is an evidence check, not a new scenario. IQ-009, if it exists only as a name, is produced here or ID-009 stays unqualified.

**M2. Commitment.**
Prerequisite: M1, so unqualified recovery and SafeNext are not silent inputs. Deliverable: durable authorization naming the effect and the rule version, before any release; history append-only; decision recomputable from sealed inputs. Exit: CCS H01, H02, H20, H22, H23, H24, H25 pass on admissible evidence for this specimen, or the clause is insufficient, not passed. Qualification: those scenarios. Not the full suite.

**M3. Dispatch on that commitment.**
Prerequisite: M2. Deliverable: at most one releaser; no second release when the outcome is not determined; payload on the wire matches the authorization; a further effect has its own authorization. Exit: H03, H04, H05, H06, H07, H08, H14, H26, H27, H28, H30, H34 pass or are insufficient. H44 only if this specimen claims harmless duplicates. Qualification: those scenarios.

**M4. Observation intake, including reconciliation cases.**
Prerequisite: M2. May proceed in parallel with M3. Must finish before assembly qualification. Deliverable: gaps and conflicts stay not known; duplicate bytes are not a second witness; a late record does not edit a sealed decision; acknowledgement and reconcile may disagree without a retry. Exit: H09–H13, H15–H19 pass or are insufficient. Qualification: those scenarios.

**M5. Recovery against the real releaser.**
Prerequisite: M3 and M4. Deliverable: restart, site loss, cold restore, and backup replay do not mint a second effect or truncate history. Exit: H35, H37, H38, H39 pass. An admitted ID-008 report does not meet this exit. Qualification: those four scenarios on the assembly, not on a stub.

**M6. Hostile assembly.**
Prerequisite: M3, M4, M5. Deliverable: one run of the programme’s 005→009 path through Commitment and Dispatch under lost ack, overlapping releasers, partial apply, forked history, and releaser death. Exit: no clause 1–8 fail in that run; residuals listed, including any effect already accepted before halt (H50), without calling halt a recall. Qualification: the assembly record is a second-party copy.

**M7. CCS-1 mandatory report.**
Prerequisite: M6. Deliverable: one report covering every mandatory id in CCS Part 9, plus H44/H45 if claimed, plus H43, H49, and H50 as bound checks on the wording. Exit: no mandatory id omitted, marked not applicable without the Part 9 rule, or passed on testimony. One insufficient or indeterminate mandatory scenario means the clause is not passed. Qualification: the report itself is sealed, and a second copy is held outside the issuer.

**M8. Independent qualification.**
Prerequisite: M7 specimen and its sealed corpus. Deliverable: an attempt under the EvidenceLab constitution. Self-report alone is refused (H42). A pass names claim, specimen hash, rule, corpus, channels covered, and the clause 11 limits. Exit: either a bounded certificate whose supports are not solely the specimen’s authority, or a recorded refusal. A refusal does not fail M3. A certificate that over-claims fails the qualifier (H43). Qualification: EvidenceLab verifier recomputation and a withdrawal-head check (H40, H41).

**M9. Qualification record.**
Prerequisite: M7, and M8 if independent qualification is claimed. Deliverable: one record naming specimen, constitution version, CCS-1, corpus, verdict, and limits. Exit: clause 9. It is not edited later. Withdrawal is a new record.

Commercial pilot and production are not extra roadmap items. A pilot that causes external effects does not start before M3’s exit, and its qualification lists every unrun stage. Production that claims the constitution does not start before M7. Production that claims independent qualification does not start before M8’s certificate.

## Part 6 — Attempt to break the roadmap

Genuine issues found and how the roadmap treats them:

- **Qualification cycle, not a code cycle.** ID-008 cannot be an input to assembly qualification and also the proof of recovery after dispatch. M5 closes this. There is no circular prerequisite among M2–M9.
- **Missing integration stage.** Module exits do not imply assembly. M6 is that stage. CCS execution (M7) stays after it, not instead of it.
- **CCS-at-the-end.** Corrected by making each of M2–M5 exit on its own scenario subset, with M7 the full report. The specification is not rescheduled.
- **Architectural assumption.** That a Truth qualification, a recovery qualification, or SafeNext is a constitutional stage. Not supported by the contracts. The roadmap does not depend on them being true.
- **Qualification assumption.** That partial passes compose. They do not. Stated in M5 and M6.
- **CCS assumption.** That a frozen suite has been run. It has not. M7 is mandatory.
- **EvidenceLab on the implementation critical path.** Rejected. M8 cannot block M2 or M3.
- **IQ-009 undefined in the contracts.** Real gap in the testimony. Not filled by invention. M1 either produces the record or leaves ID-009 unqualified. That is sufficient. It is not an architecture change.

No further genuine break. No hidden dependency requires a new component.

## Part 7 — Freeze

Freeze now:

- Constitution Part 7.
- CCS-1 as the conformance specification.
- EvidenceLab constitutional design v1.
- The decision that architectural debate is over.
- The qualification order M1–M9 as the implementation roadmap’s sequence and exits.

Do not freeze:

- Any claim that Truth, ID-008, or ID-009 is qualified.
- IQ-009, until M1 admits a real record.
- Research programmes R1–R6, except where CCS already made a scenario mandatory (H06, H34, and H44 only if harmless duplicates are claimed).
- A production or independent-qualification claim. Those wait on M7 and M8.
- Topology, storage, and language.

Architectural work stops. The next work is M1, then Commitment and Dispatch under the existing clauses. No constitutional change is required to do that.
