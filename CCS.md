# Constitutional Conformance Suite (CCS)

Version: CCS-1
Binds to: Constitution Part 7, clauses 1–11, as frozen. CCS does not amend that text.
Status: conformance specification. Not an architecture. Not an implementation.

An implementation is conformant to a named clause set only when every mandatory scenario for those clauses has produced admissible evidence of the pass condition, and no admissible evidence of a fail condition, under the rules in Part 8.

---

## Part 1 — Conformance philosophy

**Purpose.** CCS falsifies claims that an implementation satisfies the constitution. It is a suite in the same sense as a language conformance suite or a protocol interoperability specification: named stimuli, named observations, named verdicts. It does not improve the constitution and it does not choose a runtime.

**Scope.** Clauses 1–10 are obligations. Clause 11 is a bound on what a pass may be said to mean. Scenarios that only a particular product topology can express are out of scope. Scenarios whose only subject is a research programme in Constitution Part 8 are recorded, not mandatory.

**Admissible evidence.** Evidence CCS will read is defined in Part 6. A verdict that depends on anything else is `INSUFFICIENT EVIDENCE`, not a pass.

**What CCS proves.** For a named implementation specimen, a named constitution version, a named CCS version, and a named scenario corpus: either each mandatory scenario’s pass condition is met by admissible evidence, or it is not. A pass means “these clauses were not observed to break under these stimuli”. It is a qualification in the constitutional sense: bounded, recomputable, withdrawable.

**What CCS explicitly does not prove.**

- That the external world applied an effect once (clause 11).
- That a party holding every certifying key told the truth before the first seal (clause 11).
- That a halt recalled an effect already released (clause 11).
- That untested scenarios hold.
- That a different specimen, build, rule, or external system holds.
- That operators will not later use a side path (assumption A3). CCS can show a side path was possible or used during the run. It cannot show they never will.
- Safety, legality, or fitness beyond the claim text in the qualification record.
- That an implementation is the best, or the only, way to meet the clauses.

**Relationship.**

```
Constitution   obligations and bounds (clauses 1–11). Unchanged by CCS.
     ↓
Conformance    CCS: stimuli and evidence rules that can falsify those obligations.
     ↓
Implementation the specimen under test. Topology and storage are invisible except
               where they change admissible observations.
     ↓
Qualification  a signed, bounded record that names specimen, constitution version,
               CCS version, corpus, and verdict. It is not a property of the
               specimen. It expires or is withdrawn by a new record (clause 9).
```

CCS is not above the constitution. A CCS verdict that claims more than the corpus supports is itself a constitutional failure of the qualifier, under clauses 7, 9, and 10.

## Part 2 — Constitutional coverage matrix

“Observable behaviour” means behaviour a party other than the specimen’s operator can see in admissible records, or in an external effect the scenario’s instrument recorded. Internal intent that never appears in a record is not observable.

| Article | Observable behaviour | Required evidence | Hostile evidence | Pass | Fail | Indeterminate |
| --- | --- | --- | --- | --- | --- | --- |
| 1. Durable authorization before a new external effect | No instrument-recorded new effect without a prior durable authorization naming it and the rule version | Authorization record, durability witness (a second holder or a crash that still shows the record), instrument record of the effect, rule id | Effect observed with no prior authorization; authorization written only after the effect; rule version absent | For every instrument-recorded new effect in the scenario, a prior authorization names it and the rule | An effect exists with no such prior record, or the record appears only after the effect | Effect may have happened and the records needed to order “before” are missing |
| 2. Append-only history | An attempt to alter or delete an authorization, release, observation, qualification, or withdrawal does not change the bytes a second holder already has; correction is a new record | Original bytes held by a second party, specimen’s later copy, hash or byte comparison | Second holder’s bytes differ and no new record explains a legitimate append; a record is gone | Prior bytes match; changes are new records | A prior record’s bytes changed or disappeared | The second copy was never taken, so rewrite cannot be distinguished from first write |
| 3. One releaser at a time per authorization | Two overlapping attempts to release a new effect under one authorization yield at most one new effect, or a detected refusal of the second | Two release attempts, their order or overlap, instrument count of effects, records of which authority released | Two new effects under one authorization; two authorities both show a successful new release | At most one new effect; any second attempt is refused in the record | Two new effects, or two recorded successful new releases, for one authorization | Overlap or effect-count cannot be shown |
| 4. Ambiguity does not authorize another release | After an outcome that is not determined, no second release under that authorization unless a still-valid qualified harmless-duplicate rule is in the record | Attempt record, observation set, second-release attempt, qualification record of the harmless-duplicate rule if one is claimed | Second release with no such qualification; second release after that qualification is withdrawn; “unknown” rendered as “did not happen” and then released again | No second release, or a second release only under a still-valid qualified rule that says duplicates are harmless | Second release without that rule, or with a withdrawn rule, or unknown rewritten as a known negative | It is not clear whether a second release occurred, or whether the outcome was in fact determined |
| 5. Further effect has its own authorization | A corrective or compensating external effect has an authorization that names it, distinct from the authorization of the effect it corrects | Both authorizations, instrument records of both effects, link if the implementation records one | A second external effect with no authorization; a record that calls the second effect “undo” of the first authorization without a new authorization | Each instrument-recorded consequential effect has its own prior authorization | An external effect is recorded as undo, retry, or recovery and has no authorization naming it | Whether the second world-change happened is unknown |
| 6. Decision is a function of sealed inputs and a named rule | Recompute of admit, satisfy, or qualify from the sealed inputs and the named rule matches the specimen’s decision; a hidden live change does not change a decision whose inputs did not change | Sealed inputs, rule text, specimen decision, recomputation result, and the live value that was perturbed | Recomputation differs; decision changes when only an unsealed live value changes; rule version missing | Recomputation matches; perturbed unsealed state does not change the decision | Mismatch, or the decision tracks an unsealed input | Inputs or rule bytes are not available to the recomputer |
| 7. Unknown stays unknown | Missing, late, and conflicting observations remain visible as not known; they are not displayed or exported as known | Observations, conflicts, the specimen’s stated outcome | A score, colour, or word that a reader can take as success while a required observation is missing or in conflict; a conflict deleted | The outcome a reader can take as the claim is “not known”, and both conflicting bytes remain | Unknown is rendered as known, or a conflict is dropped | The reader-visible outcome was not captured |
| 8. Recomputation without the issuer | A party who is not the issuing process, given records and rule, obtains the same decision | Records, rule, independent recomputation, statement that the issuer process was not used | Only the live service can answer; offline result differs | Offline result equals the recorded decision | Offline path absent, or result differs | Records required to recompute were not handed over |
| 9. Qualification is bounded and withdrawable | The qualification record names claim, specimen, rule, corpus, and limit; a withdrawal is a new record; if the withdrawal head cannot be fetched, a checker does not treat the qualification as currently valid | Qualification bytes, withdrawal record, checker result with head present and with head unreachable | Qualification with no limit; edited qualification; checker returns valid when the head is unreachable; withdrawal by mutation | Names and limit present; withdrawal appends; unreachable head yields not-currently-valid | Any hostile evidence in the previous column | The checker was not run, or the head’s reachability was not controlled |
| 10. Self-report is not sole support of independent qualification | An attempt to independently qualify the specimen using only records it produced under its own authority is refused | Authority of each supporting record, qualification attempt, refusal | A qualification of the specimen that cites only its own records | Refusal, and no qualification record that treats that corpus as sufficient | Such a qualification is issued | The authority of the records cannot be classified |
| 11. Bounds are not tested as obligations | Scenarios that are impossible or unpromised do not produce FAIL for an implementation that refuses the wider claim | The qualification text and the scenario result | A CCS or qualifier claim of exactly-once, of honesty before first seal, or of halt-recalls-bytes | The suite records the bound and does not score the implementation down for failing the unpromised property | The suite or the qualifier states that the implementation proved a clause 11 property | The suite’s wording was not inspected |

## Part 3 — Hostile scenario catalogue

Mandatory scenarios can falsify a clause with admissible evidence and do not depend on an unfrozen research question. Optional scenarios apply only when the implementation claims the relevant rule (for example a harmless-duplicate qualification). Research scenarios are outside the pass/fail gate.

| ID | Name | Class |
| --- | --- | --- |
| H01 | Release with no prior authorization | Mandatory |
| H02 | Authorization written after the effect | Mandatory |
| H03 | Lost acknowledgement after external commit | Mandatory |
| H04 | Lost acknowledgement before external commit | Mandatory |
| H05 | Duplicate release under one authorization | Mandatory |
| H06 | Parallel release, two authorities | Mandatory |
| H07 | Partial external commit | Mandatory |
| H08 | Permanent ambiguity | Mandatory |
| H09 | Unknown that never resolves | Mandatory |
| H10 | Late observation that contradicts a known outcome | Mandatory |
| H11 | Duplicate observation delivery | Mandatory |
| H12 | Divergent observers | Mandatory |
| H13 | Byzantine observer | Mandatory |
| H14 | Byzantine adapter | Mandatory |
| H15 | False observation | Mandatory |
| H16 | Missing observation | Mandatory |
| H17 | Conflicting observation | Mandatory |
| H18 | Delayed observation within an open decision | Mandatory |
| H19 | External reconciliation disagreement | Mandatory |
| H20 | Tampered evidence after a second party holds a copy | Mandatory |
| H21 | Forked history | Mandatory |
| H22 | Stale authority used to release | Mandatory |
| H23 | Policy or rule change mid-flight | Mandatory |
| H24 | Hidden live input changes a decision | Mandatory |
| H25 | Offline recomputation | Mandatory |
| H26 | Request mutation after authorization | Mandatory |
| H27 | Replay of an old release token or record | Mandatory |
| H28 | Compensation or undo without a new authorization | Mandatory |
| H29 | Manual intervention that causes an effect | Mandatory |
| H30 | Operator-forced retry from unknown | Mandatory |
| H31 | Direct provider bypass | Mandatory |
| H32 | Side-channel effect | Mandatory |
| H33 | Credential compromise of one duty | Mandatory |
| H34 | Leader or releaser failover during release | Mandatory |
| H35 | Region or site loss during release | Mandatory |
| H36 | Clock drift and leap | Mandatory |
| H37 | Recovery restart | Mandatory |
| H38 | Cold restore | Mandatory |
| H39 | Backup replay | Mandatory |
| H40 | Qualification expiry | Mandatory |
| H41 | Withdrawal head unreachable | Mandatory |
| H42 | Independent qualification from self-report only | Mandatory |
| H43 | Over-claim of exactly-once, pre-seal honesty, or halt-recall | Mandatory |
| H44 | Harmless-duplicate rule, second apply | Optional |
| H45 | Harmless-duplicate qualification withdrawn, then second apply | Optional |
| H46 | Aggregate exposure or budget exhaustion | Research |
| H47 | Budget race | Research |
| H48 | Duty collapse under one break-glass | Research |
| H49 | All certifying keys lie before first seal | Bound (clause 11) |
| H50 | Halt after bytes have left | Bound (clause 11) |

H49 and H50 are mandatory to run only as bound checks: the suite must record that they do not produce an implementation FAIL. They are not obligations.

## Part 4 — Scenario specification

Fields shared by every scenario unless overridden:

- Required assumptions: A1 when a second copy is needed; the scenario states any tighter assumption.
- Residual uncertainty: anything clause 11 already refuses; plus scenario-specific leftovers.
- Research implications: “none” means the result does not amend the constitution.

### H01 Release with no prior authorization

- Purpose: falsify clause 1.
- Threat model: buggy or hostile releaser.
- Initial conditions: no authorization for effect E.
- Sequence: stimulus asks the specimen to cause E.
- Expected: no E, or E only after a durable authorization naming E and a rule version exists.
- Forbidden: E with no such record.
- Evidence: instrument record of whether E happened; authorization store as held by a second party.
- Audit history: the refusal or the authorization, append-only.
- Qualification impact: FAIL withdraws any qualification that claims clause 1 for this specimen.
- Recovery: none required beyond the refusal remaining on record.
- Residual: if the instrument cannot see E, INSUFFICIENT EVIDENCE.
- Articles: 1. Class: Authority, Dispatch, Safety.

### H02 Authorization written after the effect

- Purpose: order of durability versus effect.
- Threat model: crash between send and record.
- Initial conditions: releaser can be frozen after the external send begins and before the record is durable.
- Sequence: freeze at that point; inspect instrument and second copy.
- Expected: either the record is already durable and names E, or E is not observed. A record that appears only after restart, dated as if it preceded E, is a fail if the second copy proves it was absent.
- Forbidden: rewriting timestamps so the record appears prior.
- Evidence: second-copy presence test taken while frozen; instrument; post-restart records.
- Audit: both the pre-restart copy and the post-restart append.
- Qualification: FAIL on clause 1 if E exists and the frozen copy lacks the authorization.
- Recovery: restart must not invent the prior record.
- Residual: freeze that is too late to know order → INDETERMINATE.
- Articles: 1, 2. Class: Safety, Dispatch, Recovery.

### H03 Lost acknowledgement after external commit

- Purpose: ambiguity after a real effect must not become a second effect.
- Threat model: network. External system applied E and the acknowledgement was lost.
- Initial conditions: authorization A exists; harmless-duplicate rule is not qualified.
- Sequence: instrument applies E; drop the response; let the specimen time out; offer it a chance to release again under A.
- Expected: outcome remains not known or known-by-observation if a real observation arrives; no second release under A.
- Forbidden: treating timeout as “did not happen” and releasing again; rendering the state as a clean failure that permits another send.
- Evidence: instrument apply log; dropped response; specimen outcome; count of applies.
- Audit: attempt, timeout, and the not-known outcome, all retained.
- Qualification: FAIL on clause 4 if a second apply occurs. Does not FAIL because the first apply happened (clause 11).
- Recovery: wait for observation or a new authorization. No silent resume of release under A.
- Residual: the world may already contain E. That is allowed.
- Articles: 4, 7, 11. Class: Dispatch, Observation, Safety.

### H04 Lost acknowledgement before external commit

- Purpose: a lost response before apply is still not knowledge of absence unless the record proves no bytes were accepted.
- Threat model: network. External system did not apply E.
- Initial conditions: as H03, and the instrument records that no apply happened and no request was accepted.
- Sequence: drop the connection before accept; let the specimen proceed.
- Expected: if the specimen’s own sealed transport record shows no acceptance, a later release under a still-valid A is not forbidden by clause 4, because the outcome is determined as “not accepted”, not unknown. If that determination depends on an unsealed guess, the outcome must stay unknown and no second release follows.
- Forbidden: guessing “not applied” from silence alone when the instrument’s accept point is after the drop window the specimen cannot see.
- Evidence: instrument accept point; specimen’s classification of the attempt; any second release.
- Audit: the classification and its inputs.
- Qualification: FAIL if silence alone is recorded as known-negative and a second effect is released when the accept point was ambiguous.
- Recovery: a determined “nothing accepted” may allow another release under the same A only if A still authorizes it and clause 3 holds. This is not a new effect beyond A; it is another attempt of the same authorized effect. A second successful apply would mean the “nothing accepted” determination was false: then clause 4 was violated.
- Residual: high. Many external systems cannot prove non-acceptance. Those runs are unknown, not known-negative.
- Articles: 4, 6, 7. Class: Dispatch, Observation.

### H05 Duplicate release under one authorization

- Purpose: one authorization, two releases, one releaser restarted.
- Threat model: retry bug.
- Initial conditions: A durable; no harmless-duplicate qualification.
- Sequence: complete one release; cause the specimen to retry the same release.
- Expected: second release refused; at most one effect.
- Forbidden: two effects; a retry button that bypasses the record.
- Evidence: two attempt records; instrument effect count.
- Audit: both attempts.
- Qualification: FAIL on clauses 3 or 4.
- Recovery: none that sends again under A.
- Residual: instrument may under-count. Then INSUFFICIENT EVIDENCE, not PASS.
- Articles: 3, 4. Class: Dispatch, Safety.

### H06 Parallel release, two authorities

- Purpose: clause 3 under overlap.
- Threat model: split, pause, or two processes.
- Initial conditions: A durable; two parties that might both release.
- Sequence: overlap both release paths; count effects.
- Expected: at most one new effect; the other attempt refused or unable to obtain release authority.
- Forbidden: two successful new releases in the record, or two effects.
- Evidence: overlap interval; both outcomes; instrument count.
- Audit: both attempts and which one was refused.
- Qualification: FAIL on clause 3. This is the R2 drill; it is mandatory because the clause is in the contract, even though the construction is not.
- Recovery: the loser must not retry into a second effect.
- Residual: if overlap was not actually achieved, INDETERMINATE, not PASS.
- Articles: 3, 4. Class: Authority, Dispatch, Distributed systems.

### H07 Partial external commit

- Purpose: a half-applied external effect is not a known success or a known clean failure.
- Threat model: external system applies part of E.
- Initial conditions: A names E as one effect.
- Sequence: instrument applies only a part; returns an error or nothing.
- Expected: not known, or a conflict if observations disagree; no second release under A; a completion or reversal, if it causes a further effect, has its own authorization.
- Forbidden: marking A satisfied; automatically sending the remainder or a reversal as “the same” authorization’s undo.
- Evidence: instrument partial state; specimen outcome; any further effect and its authorization.
- Audit: partial observation retained.
- Qualification: FAIL on clauses 4, 5, or 7.
- Recovery: new authorization if anything more must change outside.
- Residual: the partial world state remains. Clause 11.
- Articles: 4, 5, 7. Class: Dispatch, Truth, Safety.

### H08 Permanent ambiguity

- Purpose: unknown that will never be resolved still forbids another release.
- Threat model: external system that will not answer and cannot be read.
- Initial conditions: one attempt; no later observation channel.
- Sequence: wait past any specimen deadline; request another release under A.
- Expected: remains not known; no second release; no conversion into failure-that-may-retry.
- Forbidden: deadline expiry flipping unknown into “did not happen”.
- Evidence: absence of observation; deadline configuration; specimen outcome; effect count.
- Audit: the unknown record still present after the deadline.
- Qualification: FAIL on clauses 4 and 7.
- Recovery: a new authorization is the only way to send something else. A itself stays unresolved.
- Residual: permanent. The suite must not demand that it clear.
- Articles: 4, 7. Class: Observation, Qualification.

### H09 Unknown that never resolves

- Purpose: same obligation as H08 when the implementation offers a “resolve” operation.
- Threat model: operator or automation impatient to clear a queue.
- Sequence: invoke whatever the specimen exposes as resolve, close, or ack, without new admissible observation.
- Expected: the outcome a reader can take as the claim stays not known.
- Forbidden: resolve writing a known outcome from no new evidence.
- Evidence: operation invoked; inputs sealed before and after; reader-visible outcome.
- Audit: the attempt to resolve, appended, and the unchanged unknown.
- Qualification: FAIL on clauses 6 and 7 if the outcome changes with no new sealed input.
- Recovery: none.
- Residual: the unknown remains. That is pass, not a suite failure.
- Articles: 6, 7. Class: Governance, Observation.

### H10 Late authoritative truth

- Purpose: a late record does not rewrite an earlier decision; it appends.
- Threat model: delayed external statement that contradicts an earlier observation.
- Initial conditions: a decision was recorded from corpus C1.
- Sequence: deliver a conflicting authoritative record after the decision.
- Expected: old decision bytes unchanged; conflict or a new record exists; if the old decision was a qualification, withdrawal or a new qualification may be appended; the old one is not edited.
- Forbidden: in-place change of the old decision to match the late record; deleting the earlier observation.
- Evidence: pre-image of the decision held by a second party; late record; post-image.
- Audit: both observations and both decisions.
- Qualification: FAIL on clause 2 if bytes change. If this was a qualification and the implementation still presents it as currently valid after contradiction it was required to withdraw, FAIL on clause 9.
- Recovery: new decision record only.
- Residual: which observation is true of the world is not decided by CCS.
- Articles: 2, 7, 9. Class: Evidence, Truth, History.

### H11 Duplicate observation delivery

- Purpose: the same bytes twice are not two witnesses.
- Threat model: replay of a webhook or queue.
- Sequence: deliver the same signed observation twice.
- Expected: one observation for decision purposes; decision unchanged by the duplicate; history may note the duplicate delivery as delivery, not as a new fact.
- Forbidden: counting the duplicate as independent confirmation that flips unknown to known.
- Evidence: both deliveries; identity of the bytes; decision inputs.
- Audit: delivery records that show sameness.
- Qualification: FAIL on clauses 6 and 7 if the second delivery is what flips the outcome.
- Recovery: none.
- Residual: two distinct observations with different nonces are not this scenario.
- Articles: 6, 7. Class: Observation, Evidence.

### H12 Divergent observers

- Purpose: disagreement is conflict.
- Threat model: two honest observers, different views.
- Sequence: seal both; ask for a known outcome.
- Expected: not known, conflict retained, no winner picked.
- Forbidden: last-writer-wins, majority, or dropping either.
- Evidence: both records; outcome; both still present.
- Audit: both.
- Qualification: FAIL on clause 7.
- Recovery: none that deletes one.
- Residual: the world is still one way or the other. CCS does not pick.
- Articles: 7. Class: Observation, Truth.

### H13 Byzantine observer

- Purpose: a lying observer cannot delete a conflicting honest one, and cannot by itself satisfy independent qualification of the specimen if it shares the specimen’s authority.
- Threat model: observer sends a fabricated success.
- Sequence: seal a fabrication and, where the scenario has one, an independent contrary record.
- Expected: conflict if both exist; fabrication ranked or marked so that independent qualification cannot rest on it alone when it is the specimen’s own authority; no deletion of the contrary record.
- Forbidden: the lie becoming the sole support of a known outcome that erases the other record.
- Evidence: both payloads; producer identity; outcome.
- Audit: both retained.
- Qualification: FAIL on clauses 7 or 10.
- Recovery: none.
- Residual: a lie with no contrary record can remain an observation. Clause 6 then makes the decision follow the sealed lie. That is a bad input, assumption A5, not a hidden-input failure. The suite records the assumption. It is not FAIL under clause 6.
- Articles: 7, 10. Class: Evidence, Adversarial.

### H14 Byzantine adapter

- Purpose: the component that carries a release must not change the authorized payload, and must not invent a release.
- Threat model: adapter alters the request or sends without authority.
- Sequence: authorize payload P; adapter emits P′; or adapter emits P with no release authority.
- Expected: P′ is not sent as if it were P; if P′ is sent, it is a different effect and fails clause 1 unless a new authorization names P′. A send of P with no release record fails clause 1.
- Forbidden: silent mutation; a record that says P was sent when P′ was sent.
- Evidence: authorized bytes; bytes on the wire; instrument.
- Audit: authorization hash and wire hash.
- Qualification: FAIL on clauses 1 and 6.
- Recovery: do not “correct” P′ by another send under the original authorization.
- Residual: if the wire capture is missing, INSUFFICIENT EVIDENCE.
- Articles: 1, 5, 6. Class: Dispatch, Security, Adversarial.

### H15 False observation

- Purpose: an observation that is not signed by a producer the rule accepts does not satisfy a decision.
- Threat model: injected unsigned or wrongly signed success.
- Sequence: inject it; recompute.
- Expected: it does not move unknown to known.
- Forbidden: accepting it as the decisive input while the rule requires a signed producer.
- Evidence: signature check; decision inputs; outcome.
- Audit: the rejected object retained as rejected, not deleted, not silently used.
- Qualification: FAIL on clauses 6 and 7.
- Recovery: none.
- Residual: a correctly signed falsehood is H13, not H15.
- Articles: 6, 7. Class: Evidence, Security.

### H16 Missing observation

- Purpose: a required observation that never arrives stays a gap.
- Sequence: withhold it; ask for a known outcome.
- Expected: not known; the gap listed in whatever the reader can take as the claim.
- Forbidden: default value, interpolation, or “probably applied”.
- Evidence: rule’s required list; corpus; reader-visible outcome.
- Audit: coverage or equivalent that names the gap.
- Qualification: FAIL on clause 7.
- Recovery: none.
- Residual: the implementation may use different words. The test is whether a reader can mistake it for known.
- Articles: 7, 9. Class: Observation, Qualification.

### H17 Conflicting observation

- Purpose: explicit conflict path. Same as H12 with both records required by one rule.
- Expected and forbidden: as H12.
- Evidence: as H12.
- Qualification: FAIL on clause 7 if a known outcome is produced or a record is dropped.
- Articles: 7. Class: Observation, Evidence.

### H18 Delayed observation within an open decision

- Purpose: late but still-relevant evidence appends and may be an input to a new decision; it must not be ignored by rewriting history, and it must not be treated as present before it was sealed.
- Sequence: start a decision; seal a late observation; decide.
- Expected: the decision’s inputs include the late observation only if it was sealed before the decision; a decision already sealed does not change (see H10).
- Forbidden: back-dating the observation so an earlier decision appears to have used it.
- Evidence: seal time as held in the record; decision input list; second-copy timing.
- Audit: observation and decision order.
- Qualification: FAIL on clauses 2 and 6 if order is falsified.
- Recovery: new decision if the new input should count.
- Residual: wall-clock truth of “when it really happened” outside the seal is not required.
- Articles: 2, 6. Class: Observation, History.

### H19 External reconciliation disagreement

- Purpose: a pull of external state and a prior acknowledgement disagree.
- Sequence: seal an acknowledgement of success; seal a reconcile that says absent.
- Expected: conflict or not known; no automatic new release under A; no deletion of the acknowledgement.
- Forbidden: reconcile wins by overwrite; acknowledgement wins by dropping reconcile; automatic retry.
- Evidence: both records; outcome; effect count after the disagreement.
- Audit: both.
- Qualification: FAIL on clauses 4 and 7.
- Recovery: new authorization only if a further effect is required, and only once the implementation is not guessing which record was true.
- Residual: CCS does not decide which external view is right.
- Articles: 4, 7. Class: Reconciliation, Truth.

### H20 Tampered evidence after a second party holds a copy

- Purpose: clause 2.
- Sequence: second party stores bytes B; specimen is induced to alter or drop B.
- Expected: comparison fails closed; any decision bound to B is not treated as intact; a replacement is a new record, and decisions that cited B still cite B.
- Forbidden: silent replacement; issuing a qualification that hashes to the tampered bytes while presenting the old claim.
- Evidence: B and B′; decisions that name hashes.
- Audit: the detection itself appended.
- Qualification: FAIL on clause 2. Qualifications bound to the damaged corpus are not currently valid (clause 9).
- Recovery: restore by append of a new copy, not by pretending the tamper did not happen.
- Residual: tamper before any second copy exists is not detectable. That run is INDETERMINATE, assumption A1.
- Articles: 2, 8, 9. Class: Evidence, History, Security.

### H21 Forked history

- Purpose: two histories that both claim to be the continuation of one prefix.
- Sequence: partition so two holders append different successors; then let them meet.
- Expected: a reader can see two successors; the specimen does not silently pick one and discard the other; release authority does not exist on both forks for a new effect under the same authorization.
- Forbidden: discard of a fork with no record; two new effects, one from each fork.
- Evidence: both suffixes; effect count; what a reader is shown.
- Audit: both suffixes retained or a new record that identifies both and does not delete either.
- Qualification: FAIL on clauses 2 and 3.
- Recovery: a single continuation may be chosen only by a new record that points at both, and it must not release a second effect.
- Residual: which fork matched the external world may remain unknown.
- Articles: 2, 3, 7. Class: History, Distributed systems, Recovery.

### H22 Stale authority

- Purpose: an authorization that the rule says is no longer valid does not release.
- Sequence: expire or withdraw A; then attempt release.
- Expected: no new effect under A.
- Forbidden: release on cached A after withdrawal is durable.
- Evidence: withdrawal or expiry record; attempt; instrument.
- Audit: the refused attempt.
- Qualification: FAIL on clauses 1 and 9 if a withdrawn authorization still releases. Expiry is “no longer the authority the rule names”.
- Recovery: a new authorization if a release is still wanted.
- Residual: an effect already released under A when it was valid is not recalled (clause 11, H50).
- Articles: 1, 9. Class: Authority.

### H23 Policy or rule change mid-flight

- Purpose: an in-flight decision keeps the rule version it sealed; a new rule does not rewrite it.
- Sequence: seal a decision under rule R1; publish R2; inspect the decision; attempt to re-decide the same sealed inputs under R1 by editing them.
- Expected: original decision still names R1 and still matches recomputation under R1; R2 applies only to new decisions.
- Forbidden: the old decision’s rule id changing to R2; old inputs re-evaluated in place.
- Evidence: decision bytes before and after; R1 and R2 texts.
- Audit: both rule versions retained.
- Qualification: FAIL on clauses 2 and 6.
- Recovery: new decision under R2 if desired.
- Residual: none specific.
- Articles: 2, 6. Class: Authority, Governance.

### H24 Hidden live input

- Purpose: clause 6.
- Sequence: seal inputs I; change a live value the specimen might secretly read; recompute from I and the rule; compare to the specimen’s new decision.
- Expected: decision unchanged, or a new sealed input exists that explains the change.
- Forbidden: decision tracks the live value with no new sealed input.
- Evidence: I; live value; decision; recomputation.
- Audit: I and the decision.
- Qualification: FAIL on clause 6.
- Recovery: none.
- Residual: if “live value” cannot be perturbed, NOT APPLICABLE for that specimen path.
- Articles: 6. Class: Authority, Evidence.

### H25 Offline recomputation

- Purpose: clause 8.
- Sequence: hand records and rule to a checker that is not the issuing process.
- Expected: same admit, satisfy, or qualify result.
- Forbidden: checker required to call the live issuer to obtain the result.
- Evidence: checker output; process or environment showing the issuer was not used.
- Audit: the checker version recorded with the run.
- Qualification: FAIL on clause 8 if absent or divergent.
- Recovery: none.
- Residual: a second independent implementation of the checker is not required. Divergence between two implementations is a specimen defect only if one of them is the specified checker.
- Articles: 8, 6. Class: Qualification, Evidence.

### H26 Request mutation

- Purpose: bytes on the wire match the authorization.
- Sequence: as H14, limited to mutation rather than invention.
- Expected: mismatch is not recorded as a faithful release of the authorized effect.
- Forbidden: hash in the record not equal to captured bytes while the record claims success of that authorization.
- Evidence: authorization hash; capture.
- Audit: both hashes.
- Qualification: FAIL on clauses 1 and 6.
- Recovery: do not send a “fix” under the same authorization if the mutated effect may already have happened; that is unknown plus clause 4.
- Residual: capture gaps → INSUFFICIENT EVIDENCE.
- Articles: 1, 4, 6. Class: Dispatch, Security.

### H27 Replay attack

- Purpose: an old release or an old observation does not create a new effect or a new witness.
- Sequence: replay a captured release; replay an old observation into a new window.
- Expected: no second effect from the captured release; the old observation does not satisfy a rule that requires a fresh sealed input.
- Forbidden: a new effect; a window marked satisfied by the replay alone.
- Evidence: capture; instrument effect count; window’s input list.
- Audit: replay attempt appended.
- Qualification: FAIL on clauses 3, 4, or 6.
- Recovery: none that treats replay as new authority.
- Residual: if the external system itself applies the replay, and the specimen did not issue a new release, the specimen does not FAIL clause 1 for the external system’s behaviour. The specimen FAILs if it issued a second release.
- Articles: 3, 4, 6. Class: Security, Dispatch.

### H28 Compensation without a new authorization

- Purpose: clause 5.
- Sequence: after E, invoke undo, compensate, or saga reverse.
- Expected: either no further external effect, or a new authorization that names the corrective effect before that effect.
- Forbidden: an external correction justified only as the inverse of A.
- Evidence: instrument of the second effect; authorization store.
- Audit: the new authorization if any.
- Qualification: FAIL on clause 5.
- Recovery: the corrective path is just another governed effect.
- Residual: a correction done by a human side path is H31, not a pass for H28.
- Articles: 5. Class: Recovery, Authority, Safety.

### H29 Manual intervention

- Purpose: an operator act that causes an external effect is still an effect.
- Sequence: operator causes E through the specimen’s supported manual path.
- Expected: a prior authorization naming that operator-caused effect and the rule that admitted it.
- Forbidden: a manual path that sends with no authorization record.
- Evidence: operator identity; authorization; instrument.
- Audit: who caused the authorization, as a record, not as an unrecorded note.
- Qualification: FAIL on clause 1 if the supported manual path bypasses authorization. A note in a ticket is not an authorization.
- Recovery: none special.
- Residual: an operator acting outside the specimen is H31.
- Articles: 1, 5. Class: Human operations, Governance.

### H30 Operator-forced retry from unknown

- Purpose: the social bypass of clause 4.
- Sequence: leave an attempt unknown; invoke any operator control named retry, resend, or force complete.
- Expected: no second effect under the same authorization.
- Forbidden: the control existing as a release path; a record that the operator “accepted” a guess which then releases.
- Evidence: control invoked; effect count; records written.
- Audit: the invocation.
- Qualification: FAIL on clause 4. If the control is absent and no effect occurs, PASS.
- Recovery: new authorization if the operator must send something else, and that new authorization must not pretend the unknown is now known without new evidence (clause 7).
- Residual: a determined “nothing was accepted” is H04, not this scenario.
- Articles: 4, 7. Class: Human operations, Governance, Safety.

### H31 Direct provider bypass

- Purpose: assumption A3. An effect caused with a credential the specimen does not mediate.
- Sequence: using a credential the scenario planted at the external system, cause E with no specimen authorization.
- Expected: CCS does not call the specimen conformant for governing E. The qualification record, if it claimed governance of that external system during this run, is withdrawn or not issued. The specimen software does not FAIL clause 1 for an effect it did not perform, provided it did not hold or conceal that credential as a supported path.
- Forbidden: a qualification that says all effects on that system were authorized, while the bypass effect exists; the specimen treating the bypass as its own success.
- Evidence: instrument of E; absence of authorization; credential used.
- Audit: the bypass recorded as outside the specimen if the specimen learns of it; learning of it does not rewrite history.
- Qualification: the governance claim is not currently valid for the window. This is QUALIFICATION WITHDRAWN for that claim, not an implementation FAIL, unless the specimen’s own path performed the bypass.
- Recovery: the constitution does not require the specimen to undo E. Undoing would be a new effect (clause 5) and may be impossible.
- Residual: this is the highest risk in the constitution. CCS detects it only when the scenario includes the bypass. It does not detect an uninstrumented bypass.
- Articles: 1, 9, 11. Class: Governance, Security, Human operations.

### H32 Side-channel effect

- Purpose: an effect that is consequential but not the named payload (a second API call, a notification that moves money, a replica that applies as well).
- Sequence: authorize one named effect; instrument all channels the scenario lists; cause release.
- Expected: every channel that shows a new consequential effect is either named by the authorization or absent.
- Forbidden: an unlisted consequential effect the specimen’s release path caused.
- Evidence: per-channel instrument records; authorization text.
- Audit: authorization text that either names the channel or does not.
- Qualification: FAIL on clause 1 for an unnamed effect the specimen caused.
- Recovery: clause 5 for any corrective effect.
- Residual: channels the scenario did not instrument are outside the claim. The qualification must list the channels it covered (clause 9).
- Articles: 1, 5, 9. Class: Dispatch, Security.

### H33 Credential compromise of one duty

- Purpose: one stolen duty credential does not by itself both define rules and release effects, and does not by itself both mint the sole supporting observations and sign an independent qualification.
- Sequence: use only the release credential to publish a rule, or only the observation credential to sign a qualification of the specimen.
- Expected: the other duty refuses.
- Forbidden: one credential succeeding at both.
- Evidence: which credential was used; refusal or success of the crossed act.
- Audit: the refusal.
- Qualification: FAIL on the separation the constitution requires (Part 5 invariant 9’s supporting duty split, expressed by clauses 6 and 10 for qualification, and by the requirement that rule version and release are not the same unreviewed act). If the specimen has no independent-qualification path, the qualification half is NOT APPLICABLE and the release-versus-rule half still applies whenever both duties exist. If the specimen genuinely has one person and one key for everything, independent qualification under clause 10 cannot PASS.
- Recovery: withdraw any qualification the crossed credential managed to sign, by a new record from a key that was not the compromised one. If no such key exists, qualifications are not currently valid.
- Residual: compromise of all duties together is H49, a bound, not a fail.
- Articles: 9, 10. Class: Security, Authority, Qualification.

### H34 Leader or releaser failover during release

- Purpose: clause 3 across a crash of the releaser.
- Sequence: begin release; kill the releaser before and, in a second run, after the release record is durable; start a successor; count effects.
- Expected: at most one new effect; successor does not release again if a release was already recorded or if acceptance is unknown.
- Forbidden: successor always retries and produces a second effect.
- Evidence: crash point; durable records; instrument count.
- Audit: the attempt the successor saw.
- Qualification: FAIL on clauses 3 and 4.
- Recovery: successor may release only if no release was recorded and the authorization is still valid and the outcome is not unknown. If the crash point makes acceptance unknown, no release.
- Residual: a crash window that the test cannot control is INDETERMINATE, not PASS.
- Articles: 3, 4, 1. Class: Recovery, Distributed systems, Dispatch.

### H35 Region or site loss during release

- Purpose: same clauses when a whole copy set in one place disappears.
- Sequence: release path in progress; destroy the site that was releasing; continue from the remaining copies.
- Expected: no second effect; if durability of the authorization cannot be shown, no new release.
- Forbidden: remaining site inventing a release the destroyed site may already have performed; remaining site releasing when it cannot show durability.
- Evidence: what the remaining copies contain; instrument count.
- Audit: the halt of release and the reason.
- Qualification: FAIL on clauses 1, 3, and 4.
- Recovery: release resumes only under clauses 1, 3, and 4. Geographic layout is not specified.
- Residual: an effect the destroyed site released, with the acknowledgement lost, may exist. That is not a FAIL if the survivor does not release again.
- Articles: 1, 3, 4. Class: Disaster recovery, Dispatch.

### H36 Clock drift and leap

- Purpose: a wall-clock jump does not by itself admit, satisfy, or extend authority.
- Sequence: jump the specimen clock forward and backward across an authorization’s validity; compare decisions to sealed inputs.
- Expected: validity follows the sealed rule inputs, not the jumped clock as a hidden input; a jump does not create a second release.
- Forbidden: clock change alone flipping unknown to known, or resurrecting a withdrawn authorization.
- Evidence: sealed inputs before and after; decisions; releases.
- Audit: both decisions if two exist.
- Qualification: FAIL on clause 6 if the clock is a hidden input. An implementation may seal the clock as an input; then recomputation must use that sealed value.
- Recovery: if the implementation’s rule says a detected jump stops new releases, that is allowed and not required. CCS does not FAIL an implementation that keeps releasing through a jump when clauses 1–4 still hold on sealed inputs.
- Residual: true time in the world is not established.
- Articles: 6, 4. Class: Distributed systems, Authority.

### H37 Recovery restart

- Purpose: process restart is not a new source of effects.
- Sequence: restart all specimen processes after an unknown attempt and after a completed attempt.
- Expected: completed authorization is not released again; unknown attempt is not released again; history bytes preserved.
- Forbidden: replay of outbound effects from a queue that survived restart, without clause 4 allowing it.
- Evidence: pre-restart second copy; post-restart records; instrument count.
- Audit: continuity of the pre-restart records.
- Qualification: FAIL on clauses 2, 3, or 4.
- Recovery: this scenario is the recovery. Success is restraint.
- Residual: in-flight external applies remain possible. Clause 11.
- Articles: 2, 3, 4. Class: Recovery.

### H38 Cold restore

- Purpose: restore from a copy does not mint releases for effects the copy already includes, and does not discard records the copy contained.
- Sequence: take a second-party copy; destroy the live store; restore; compare; attempt new releases for authorizations in the copy that were unknown or already released.
- Expected: restored bytes match the copy; no new effect for those authorizations; if the copy itself is missing the tail a second party had, the specimen must not claim the shorter copy is complete. If it cannot show completeness, it does not release.
- Forbidden: restore that truncates and then releases into the gap; restore that re-sends the log.
- Evidence: copy; restored bytes; effect count after restore.
- Audit: the restore event appended, including which copy was used.
- Qualification: FAIL on clauses 1, 2, and 4.
- Recovery: incompleteness stays visible (clause 7).
- Residual: a copy that was already tampered before sealing is A1, INDETERMINATE.
- Articles: 1, 2, 4, 7. Class: Disaster recovery, History.

### H39 Backup replay

- Purpose: feeding a backup into a live external system as if it were a queue of work.
- Sequence: replay a backup that contains already-attempted releases toward the external instrument.
- Expected: no new effects for authorizations already released or unknown.
- Forbidden: the backup being an authority to send.
- Evidence: backup contents; instrument count; which authorizations were in the backup.
- Audit: the replay attempt refused or contained.
- Qualification: FAIL on clauses 3 and 4.
- Recovery: backup is a history, not a releaser.
- Residual: none beyond clause 11.
- Articles: 2, 3, 4. Class: Disaster recovery, History.

### H40 Qualification expiry

- Purpose: clause 9.
- Sequence: issue a bounded qualification; pass its stated end; run the checker.
- Expected: checker does not report currently valid.
- Forbidden: a checker that ignores the bound; editing the bound in place.
- Evidence: qualification bytes; checker output; second copy of the original bound.
- Audit: original qualification unchanged.
- Qualification: FAIL on clause 9.
- Recovery: a new qualification if the claim is re-established. The old one remains.
- Residual: a relying party who does not run the checker is A6, outside the specimen, and must be stated as a limit of the qualification claim.
- Articles: 9, 7. Class: Qualification.

### H41 Withdrawal head unreachable

- Purpose: silence is not continued validity.
- Sequence: issue a qualification whose rule requires a withdrawal head; make the head unreachable; run the checker.
- Expected: not currently valid.
- Forbidden: valid-by-default when the head cannot be fetched.
- Evidence: checker output; network control showing unreachability.
- Audit: the qualification bytes unchanged.
- Qualification: FAIL on clause 9.
- Recovery: when the head returns, validity follows the head, including any withdrawal that arrived.
- Residual: a qualification whose own bounded text says it is a historical snapshot that must not be re-checked is still subject to clause 9: the suite’s checker treats “cannot check withdrawal” as not currently valid. An implementation that wants a historical reading labels it as history, not as current qualification.
- Articles: 9. Class: Qualification, Evidence.

### H42 Independent qualification from self-report only

- Purpose: clause 10.
- Sequence: submit only records the specimen produced under its own authority; ask for an independent qualification of the specimen.
- Expected: refusal. No qualification that a reader can take as independent.
- Forbidden: a pass, a score, or a certificate whose support is only those records.
- Evidence: producer of each record; the refusal or the illegal certificate.
- Audit: the attempt and the refusal.
- Qualification: FAIL on clause 10 if a certificate is issued. If the specimen offers no independent-qualification function at all, this scenario is still mandatory for any claim of independent qualification, and NOT APPLICABLE only when the qualification claim under test does not include clause 10. A claim of clause 10 with no function that can refuse is FAIL, because the clause cannot be met by silence of a different product.
- Recovery: none.
- Residual: a corpus that adds even one observation not under the specimen’s authority leaves this scenario and must be judged under the rule’s required list (H16), not automatically passed.
- Articles: 10, 7, 9. Class: Qualification, Evidence, Governance.

### H43 Over-claim

- Purpose: clause 11 is a bound on CCS and on qualifications.
- Sequence: inspect the qualification text and the suite’s own report for the phrases or the claims: end-to-end exactly once; the certifying party cannot have lied before first seal; halt recalled an already released effect.
- Expected: those claims are absent. The report states the three bounds.
- Forbidden: PASS wording that implies any of the three.
- Evidence: the qualification text and the suite report.
- Audit: the report is itself a record.
- Qualification: if the wording over-claims, the qualification is not valid, because it violates clause 9’s limit. The implementation does not FAIL merely because the world applied something twice, because someone with all keys could have lied, or because a halt did not pull a packet back.
- Recovery: reissue the qualification with the limit stated. Do not edit the old text.
- Residual: readers can still misunderstand. That is A6.
- Articles: 9, 11. Class: Qualification, Governance.

### H44 Harmless-duplicate rule, second apply

- Class: optional. Applies only when a qualification claims duplicates are harmless.
- Purpose: the exception in clause 4 is itself qualified, not assumed.
- Sequence: apply twice under the conditions the rule names; record the external outcomes with an instrument that is not the specimen’s self-report.
- Expected: either the external outcome matches the rule, or the qualification of that rule is withdrawn by a new record and further automatic second releases stop.
- Forbidden: keeping the rule qualified after the instrument shows a second consequential effect the rule said could not happen.
- Evidence: instrument outcomes; rule text; qualification; subsequent release behaviour.
- Audit: the run and, on mismatch, the withdrawal.
- Qualification: FAIL on clause 4 if automatic second release continues after the rule is shown false. A mismatch with a prompt withdrawal is PASS of the specimen and QUALIFICATION WITHDRAWN for that rule.
- Recovery: stop using the exception.
- Residual: a rare external fault this run did not hit. The qualification of the rule must name the conditions actually tested.
- Research: this is R1. It is optional relative to specimens that do not claim the exception. It is mandatory for specimens that do.
- Articles: 4, 9, 10. Class: Qualification, Dispatch.

### H45 Withdrawn harmless-duplicate rule

- Class: optional, mandatory if H44’s qualification exists.
- Sequence: withdraw the rule; then cause a lost acknowledgement; see whether a second release happens.
- Expected: no second release.
- Forbidden: cached permission to duplicate.
- Evidence: withdrawal; second-release attempt; instrument.
- Qualification: FAIL on clauses 4 and 9.
- Articles: 4, 9. Class: Authority, Qualification.

### H46 Aggregate exposure exhaustion

- Class: research.
- Why it is not mandatory: the constitution does not define an exposure budget. A second effect or a release without authorization is already H01, H05, and H32.
- Sequence, if run: hit any implementation-local cap and attempt further releases.
- Expected constitutional behaviour: no conclusion from the cap itself. FAIL only if a clause 1–10 behaviour breaks.
- Forbidden: CCS marking FAIL solely because a cap the constitution does not require was absent, or PASS that claims “exposure is safe”.
- Qualification impact: NOT APPLICABLE to conformance. A local cap may be reported as an implementation fact.
- Research: remains outside CCS until a constitutional amendment names the obligation, which this suite does not propose.

### H47 Budget race

- Class: research. Same reasoning as H46. Two releases that race are H06 if they share an authorization. A race between two different authorizations is allowed by the constitution.

### H48 Duty collapse under one break-glass

- Class: research (R4). Not an implementation FAIL by itself.
- Sequence: one credential is granted both duties; attempt a crossed act as in H33.
- Expected: record which clauses still hold. Independent qualification under clause 10 cannot PASS while the collapse lasts.
- Forbidden: the suite describing the collapsed state as independently qualified.
- Qualification: clause 10 is not met for the collapsed window. Other clauses are scored only on their own evidence.
- Research: stays outside the mandatory gate. Adoption into the constitution is forbidden unless Part 7 changes, which CCS cannot do.

### H49 All certifying keys lie before first seal

- Class: bound.
- Sequence: a party with every relevant key seals a fiction and signs it before any second party has a contrary instrument record.
- Expected: CCS records that this is not detected. The implementation is not FAIL.
- Forbidden: a PASS that says the suite would have detected the lie.
- Evidence: the statement of the bound in the report.
- Qualification: any qualification of “honesty of the sealer” is an over-claim (H43).
- Residual: total. This is clause 11.
- Articles: 11. Class: Evidence, Adversarial.

### H50 Halt after bytes have left

- Class: bound.
- Sequence: release so the instrument has accepted E; then halt new releases; observe E.
- Expected: E may exist. The specimen is not FAIL for failing to recall E. It is FAIL if a new effect is released after the halt under the same authorization when the outcome is unknown (that is H03, not this bound).
- Forbidden: a qualification that says halt prevents E.
- Evidence: instrument timing of accept versus halt; any later release.
- Qualification: over-claim handled by H43. Implementation scored only on later releases.
- Articles: 11, 4. Class: Dispatch, Recovery.

## Part 5 — Assertions by scenario

| ID | Articles | Invariants | Kind |
| --- | --- | --- | --- |
| H01 | 1 | 2 | Safety, Authority, Dispatch |
| H02 | 1, 2 | 1, 2 | Safety, Recovery, History |
| H03 | 4, 7, 11 | 4, 5 | Safety, Dispatch, Observation |
| H04 | 4, 6, 7 | 4, 5, 6 | Dispatch, Observation |
| H05 | 3, 4 | 3, 4 | Safety, Dispatch |
| H06 | 3, 4 | 3, 4 | Safety, Authority, Dispatch, Distributed systems |
| H07 | 4, 5, 7 | 4, 5, 10 | Safety, Dispatch, Truth |
| H08 | 4, 7 | 4, 5 | Observation, Qualification |
| H09 | 6, 7 | 5, 6 | Governance, Observation |
| H10 | 2, 7, 9 | 1, 5, 8 | Evidence, Truth, History |
| H11 | 6, 7 | 5, 6 | Observation, Evidence |
| H12 | 7 | 5 | Observation, Truth |
| H13 | 7, 10 | 5, 9 | Evidence, Adversarial |
| H14 | 1, 5, 6 | 2, 6, 10 | Dispatch, Security |
| H15 | 6, 7 | 5, 6 | Evidence, Security |
| H16 | 7, 9 | 5, 8 | Observation, Qualification |
| H17 | 7 | 5 | Observation, Evidence |
| H18 | 2, 6 | 1, 6 | Observation, History |
| H19 | 4, 7 | 4, 5 | Reconciliation, Truth |
| H20 | 2, 8, 9 | 1, 7, 8 | Evidence, History, Security |
| H21 | 2, 3, 7 | 1, 3, 5 | History, Distributed systems, Recovery |
| H22 | 1, 9 | 2, 8 | Authority |
| H23 | 2, 6 | 1, 6 | Authority, Governance |
| H24 | 6 | 6 | Authority, Evidence |
| H25 | 8, 6 | 6, 7 | Qualification, Evidence |
| H26 | 1, 4, 6 | 2, 4, 6 | Dispatch, Security |
| H27 | 3, 4, 6 | 3, 4, 6 | Security, Dispatch |
| H28 | 5 | 10 | Recovery, Authority, Safety |
| H29 | 1, 5 | 2, 10 | Human operations, Governance |
| H30 | 4, 7 | 4, 5 | Human operations, Governance, Safety |
| H31 | 1, 9, 11 | 2, 8 | Governance, Security, Human operations |
| H32 | 1, 5, 9 | 2, 8, 10 | Dispatch, Security |
| H33 | 9, 10 | 8, 9 | Security, Authority, Qualification |
| H34 | 3, 4, 1 | 2, 3, 4 | Recovery, Distributed systems, Dispatch |
| H35 | 1, 3, 4 | 2, 3, 4 | Disaster recovery, Dispatch |
| H36 | 6, 4 | 4, 6 | Distributed systems, Authority |
| H37 | 2, 3, 4 | 1, 3, 4 | Recovery |
| H38 | 1, 2, 4, 7 | 1, 2, 4, 5 | Disaster recovery, History |
| H39 | 2, 3, 4 | 1, 3, 4 | Disaster recovery, History |
| H40 | 9, 7 | 5, 8 | Qualification |
| H41 | 9 | 8 | Qualification, Evidence |
| H42 | 10, 7, 9 | 5, 8, 9 | Qualification, Evidence, Governance |
| H43 | 9, 11 | 8 | Qualification, Governance |
| H44 | 4, 9, 10 | 4, 8, 9 | Qualification, Dispatch |
| H45 | 4, 9 | 4, 8 | Authority, Qualification |
| H46 | none | none | Research. Not a conformance kind. |
| H47 | none unless it reduces to H06 | 3 if it reduces to H06 | Research |
| H48 | 10 | 9 | Research, Governance, Security |
| H49 | 11 | bound | Evidence, Adversarial |
| H50 | 11, 4 | 4, bound | Dispatch, Recovery |

Invariant numbers refer to Constitution Part 5.

## Part 6 — Evidence requirements

CCS accepts a record only if a second party can hold the bytes and a later comparison can detect a change (clause 2). Labels:

**Operational logs.** The specimen’s own running records. Admissible as history of what the specimen claims it did. Not admissible as the sole support of an independent qualification (clause 10). Admissible for clauses 1–8 when corroborated by an instrument or a second copy as the scenario requires.

**System testimony.** A statement by the issuing process that is not recomputable from handed-over bytes. Inadmissible for a pass. Admissible only as a claim to be checked.

**External observations.** Records from the system the effect was aimed at, or from the wire. Admissible for effect counts and payload bytes when the scenario’s instrument captured them. They are not automatically “true”; two of them can conflict (H12, H19).

**Independent observations.** Captured by a party whose key is not the specimen’s release or self-report key. Required for any PASS of clause 10, and required for H44’s external outcome.

**Authoritative records.** A name some external parties use for their own statements. Inside CCS they are ordinary sealed observations. They do not outrank a conflict and they do not edit history (H10).

**Operator evidence.** An attributed sealed record of what an operator did through the specimen. Admissible. A ticket, chat message, or recollection that is not sealed is inadmissible for a pass.

**Qualification evidence.** The qualification bytes, the corpus hash they name, the checker output, and the withdrawal head. Required for clauses 9 and 10.

**Historical evidence.** A second party’s earlier copy. Required whenever the scenario claims order or non-rewriting (H02, H10, H20, H38).

**Admissible evidence.** Sealed bytes plus enough provenance to know who produced them, bound into the scenario record, and still matching a second copy. Recomputation uses these bytes and the named rule only.

**Inadmissible evidence.** Unsigned assurances, dashboards, scores, model output, operator memory, logs the specimen can replace before anyone else holds a copy, and any summary that replaces the bytes it summarises.

**Unknown evidence.** A gap, a conflict, a late record that misses the decision, or a capture that failed. It never counts as a pass. It yields `INSUFFICIENT EVIDENCE` or `INDETERMINATE` as Part 8 specifies. It is itself admissible proof of a gap when the scenario’s pass condition is “the gap remains visible”.

## Part 7 — Expected outcomes

The only outcomes CCS may write on a scenario are the Part 8 verdicts. The specimen’s own vocabulary is evidence, not a CCS verdict.

| ID | Permitted specimen behaviour | Forbidden specimen behaviour | CCS outcome if forbidden behaviour is shown |
| --- | --- | --- | --- |
| H01 | Reject the release, or hold until a durable authorization exists | Proceed to an effect | FAIL |
| H02 | Hold or reject across the freeze | Proceed, then invent a prior record | FAIL |
| H03 | Hold as unknown | Proceed to a second release; record a known negative from silence | FAIL |
| H04 | Hold as unknown, or proceed only if non-acceptance is a sealed input | Proceed from silence alone | FAIL |
| H05 | Reject the duplicate | Proceed | FAIL |
| H06 | Reject one side | Two proceeds | FAIL |
| H07 | Hold as unknown or disputed | Proceed to satisfy, undo, or complete under the same authorization | FAIL |
| H08 | Hold forever | Proceed, or record a known negative at the deadline | FAIL |
| H09 | No conclusion beyond the existing unknown | A known outcome from the resolve action | FAIL |
| H10 | Hold the old bytes; append; disputed if both remain | Rewrite; delete | FAIL |
| H11 | No conclusion changes | Duplicate flips unknown to known | FAIL |
| H12 | Disputed / unknown | Pick a winner | FAIL |
| H13 | Disputed if both exist; refuse sole-support qualification | Delete the contrary record; qualify from the lie alone | FAIL |
| H14 | Reject mutation or unsanctioned send | Proceed with P′ as if it were P | FAIL |
| H15 | Reject | Proceed to known | FAIL |
| H16 | Unknown, gap visible | Proceed to known | FAIL |
| H17 | Disputed | Proceed to known or drop a record | FAIL |
| H18 | New decision only after seal | Back-date | FAIL |
| H19 | Disputed; no second release | Proceed by picking one and deleting the other | FAIL |
| H20 | Detect; do not treat the decision as intact | Silent replace | FAIL |
| H21 | Show both forks; at most one new effect | Drop a fork; two effects | FAIL |
| H22 | Reject | Proceed on stale authority | FAIL |
| H23 | Old decision stands under the old rule | Rewrite the rule id | FAIL |
| H24 | Decision stable | Decision follows the live value | FAIL |
| H25 | Offline result matches | Live service required | FAIL |
| H26 | Mismatch visible; no false success | Record claims P while wire shows P′ | FAIL |
| H27 | Reject replay | Second effect or a satisfied fresh window | FAIL |
| H28 | Reject, or a new authorization then proceed | Undo without a new authorization | FAIL |
| H29 | Proceed only under an authorization | Manual proceed without one | FAIL |
| H30 | Reject | Force retry | FAIL |
| H31 | No specimen proceed; qualification must not claim governance of that effect | Qualification says the effect was governed | QUALIFICATION WITHDRAWN for that claim. Implementation FAIL only if the specimen performed the bypass |
| H32 | Proceed only on named channels | Unnamed consequential effect | FAIL |
| H33 | Reject the crossed act | One credential does both | FAIL |
| H34 | At most one effect | Successor proceeds into a second effect or into an unknown | FAIL |
| H35 | Hold releases that lack durability or risk a second effect | Invent the missing release | FAIL |
| H36 | Decisions follow sealed inputs | Clock alone changes the decision or resurrects authority | FAIL |
| H37 | Hold | Restart proceeds into a second effect | FAIL |
| H38 | Bytes match; hold further releases | Truncate and proceed | FAIL |
| H39 | Reject replay as authority | Backup proceeds | FAIL |
| H40 | Checker says not currently valid | Still valid, or edited bound | FAIL |
| H41 | Not currently valid | Valid because the head was silent | FAIL |
| H42 | Reject | Certificate from self-report only | FAIL |
| H43 | Limits stated | Over-claim | Qualification not valid. Not an implementation FAIL |
| H44 | Withdraw the rule and stop, or the external outcome matches the rule | Keep auto-releasing after a contradictory instrument result | FAIL, or QUALIFICATION WITHDRAWN when withdrawal is prompt |
| H45 | Reject | Proceed on a withdrawn rule | FAIL |
| H46 | No constitutional proceed/reject | CCS fail or pass based on the cap alone | NOT APPLICABLE |
| H47 | As H06 if it is the same authorization | Two effects for one authorization | FAIL only via H06 |
| H48 | Clause 10 not passed during collapse | Calling the collapsed window independently qualified | Clause 10 not met. Not a blanket implementation FAIL |
| H49 | Undetected lie recorded as a bound | A pass that claims detection | Suite failure if it claims detection. Implementation not FAIL |
| H50 | E may exist; no new release required to recall it | A pass that says halt prevented E; a later extra release | Over-claim is H43. Extra release is FAIL under clause 4 |

Specimen words such as Proceed, Reject, Hold, Unknown, and Disputed are descriptions of permitted behaviour. They are not extra constitutional states. Escalate, suspend automation, and record assumption failure are allowed only as append-only records. They must not be a hidden release path and they must not erase unknown.

## Part 8 — Qualification rules

CCS writes exactly one of these on each scenario:

**PASS.** Admissible evidence shows the pass condition, and no admissible evidence shows the fail condition.

**FAIL.** Admissible evidence shows the fail condition for a mandatory scenario, or for an optional scenario the specimen’s claim made mandatory.

Two kinds of FAIL, always labelled:

- **Implementation FAIL.** The specimen’s observable behaviour broke a clause. This withdraws any current qualification of that clause for that specimen. It does not amend the constitution.
- **Constitutional failure of the qualifier.** The specimen may be fine, but the qualification text or the suite report claims more than clauses 1–10 allow, or treats self-report as independent qualification, or hides unknown. H43 is this kind. A suite that emits it is not itself fit to qualify others until a new suite record says the report was corrected by a new report, not by editing the old one.

**INDETERMINATE.** The scenario ran, evidence conflicts or the order of events cannot be established, and neither pass nor fail is shown. This is not a pass. A qualification cannot cite the scenario as satisfied.

**NOT APPLICABLE.** The specimen does not claim the optional behaviour (H44, H45), or the constitution is silent (H46, H47), or the path does not exist (H24 when no live input can be perturbed). Not a pass of a clause the specimen does claim.

**INSUFFICIENT EVIDENCE.** The required capture was not made: no second copy, no instrument, no wire capture. Not a pass. Distinct from INDETERMINATE: here the test was not observed, rather than observed and ambiguous.

**QUALIFICATION WITHDRAWN.** A new record ends a named qualification. Used when a rule’s own qualification is falsified (H44) or a governance claim is falsified by a bypass (H31), without necessarily an implementation FAIL.

**QUALIFICATION SUSPENDED.** Used when validity cannot be checked (H41) or a bound has ended (H40) and the qualification must not be presented as current. The old bytes remain. Suspension is a CCS word for “not currently valid”; the constitutional mechanism is still a new record or an unreachable head, not a deletion.

A clause’s qualification is PASS only when every mandatory scenario mapped to that clause in Part 5 is PASS, and no such scenario is FAIL. One INDETERMINATE or INSUFFICIENT EVIDENCE on a mandatory scenario makes the clause INSUFFICIENT EVIDENCE, not PASS.

Differentiating failures: if a second copy and an instrument show a second effect, it is an implementation FAIL even if the specimen’s log says otherwise. If the log says otherwise and there is no instrument, it is INSUFFICIENT EVIDENCE, not a pass based on system testimony. If the specimen met clauses 1–10 and the report claims exactly-once, the FAIL is the qualifier’s, and the specimen’s clause results stand.

## Part 9 — Conformance corpus

Counts are the catalogue in Part 3. A scenario sits in every category Part 5 names; the count below is by primary category so the sum equals 50.

| Category | Scenarios | Mandatory | Optional | Research or bound, not a pass gate |
| --- | --- | --- | --- | --- |
| Authority | H01, H22, H23 | 3 | 0 | 0 |
| Dispatch | H03, H04, H05, H07, H14, H26, H32 | 7 | 0 | 0 |
| Observation | H08, H09, H11, H12, H16, H18 | 6 | 0 | 0 |
| Truth | H19 | 1 | 0 | 0 |
| Evidence | H13, H15, H17, H20 | 4 | 0 | H49 bound |
| Recovery | H28, H34, H37 | 3 | 0 | H50 bound, scored under dispatch if a later release happens |
| Qualification | H40, H41, H42, H43 | 4 | H44, H45 (mandatory if that claim is made) | 0 |
| Governance | H29, H30, H31 | 3 | 0 | H48 research |
| History | H02, H10, H21, H39 | 4 | 0 | 0 |
| Security | H27, H33 | 2 | 0 | 0 |
| Distributed systems | H06, H36 | 2 | 0 | 0 |
| Scale | none | 0 | 0 | H46, H47 research. Scale is not a clause |
| Human operations | covered as Governance | 0 extra | 0 | 0 |
| Adversarial behaviour | covered as Evidence and Security | 0 extra | 0 | H49 |
| Disaster recovery | H35, H38 | 2 | 0 | 0 |
| **Total distinct** | **50** | **43** | **2** | **5** (H46, H47, H48, H49, H50) |

H44 and H45 are optional in the base total and become mandatory when the specimen claims a harmless-duplicate qualification. H49 and H50 are mandatory bound checks on the report, not on the specimen’s ability to do the impossible.

No category is added for a product component. Scale has no mandatory scenario because the constitution states no throughput obligation.

## Part 10 — Versioning

CCS versions are CCS-n, bound to a constitution version. This document is CCS-1 bound to Constitution Part 7 as frozen in `CONSTITUTION.md`.

**New hostile scenarios.** Append them in a new CCS version. Old scenario identifiers are not reused and their pass conditions are not weakened. A qualification names the CCS version. It does not become a pass of new scenarios by silence. New mandatory scenarios apply to new qualifications. They do not rewrite old qualification bytes. If a new scenario shows an old specimen breaking a clause, the response is a new withdrawal record, not an edit of the old pass.

**Constitutional amendments.** CCS cannot amend the constitution. If the constitution version changes, CCS-1 does not stretch to cover it. A new CCS version maps the new clauses. Old qualifications remain qualifications of the old pair of versions only.

**Implementation regressions.** A later specimen build is a new specimen. The old qualification does not cover it. Re-run is a new corpus. A regression is an implementation FAIL of the new specimen, or INSUFFICIENT EVIDENCE if not re-run. It is not a retroactive edit.

**Research discoveries.** R1–R6 may add optional scenarios or move H44’s evidence standard. They become mandatory only by a new CCS version that says so, and only for claims that use the discovered rule. Discoveries do not add constitutional clauses. H46–H48 stay non-gating until a constitution that is not this one says otherwise.

**Compatibility rule.** A checker for CCS-1 accepts CCS-1 corpora only. Unknown scenario ids are INSUFFICIENT EVIDENCE, not pass. A report that omits a mandatory id is not a qualification.

## Part 11 — Independent assessment

**Can an implementation appear conformant while violating the constitution?** Yes, in three ways the suite must keep visible.

1. Uninstrumented channels and uninstrumented side credentials (H31, H32). A pass covers listed channels and listed paths only. Clause 9 requires the qualification to say so. A reader who drops that sentence will think the pass is total. That is a gaming path outside the specimen.
2. Tamper or fiction before any second party holds a copy (H20 residual, H49). CCS cannot see it. A pass that claims otherwise is a qualifier failure.
3. A harmless-duplicate claim qualified on a narrow test (H44) that is wrong for an untested failure mode. The qualification is only as wide as the instrumented conditions. Treating it as “this external system is idempotent” is an over-claim.

**Can CCS itself be gamed?** Yes. By running mandatory scenarios without the instrument and recording PASS from system testimony. Part 8 forbids that and requires INSUFFICIENT EVIDENCE. By editing an old report instead of appending a withdrawal. Clause 2 forbids that, and the suite fails the qualifier if a second copy of the report differs. By marking hard scenarios NOT APPLICABLE. Part 9’s mandatory list is the defence: an omitted mandatory id is not a qualification. The remaining game is collusion between the party who captures “independent” observations and the specimen, which is H49. CCS records it and does not pretend to close it.

**Scenarios still missing as obligations.** None that falsify a further sentence of Part 7, because Part 7 has no further sentences. Missing on purpose: throughput, exposure caps, multi-region product drills beyond H35’s clause test, passport interchange, witness interchange, and long-lived re-assessment schedules. Those are R-programmes or exclusions.

**Assumptions still untested.** A3 is tested only when H31 is actually run with a planted credential. A4 is not in the mandatory gate; H48 is research. A5 is recorded by H13 and not eliminated. A6 is outside the specimen. A1 is tested only for copies the scenario actually took. A2 is tested only when H44 is in scope. A7 is H06 and H34, and an indeterminate overlap must not be recorded as PASS.

**Research left outside CCS.** Constitution Part 8 programmes R1 through R6, except that R1 and R2 have a mandatory footprint already: R2 is H06 and H34, and R1 is mandatory only for specimens that claim the exception. R3’s refusal test is H42. R4 is H48. R5 and R6 have no scenario that can fail an implementation, because the constitution does not require a witness, a passport, or a re-assessment schedule.

CCS-1 is fit to use against the frozen constitution. It is not fit to use as a claim of safety, exactly-once execution, or honesty of a party who held every key before the first seal.
