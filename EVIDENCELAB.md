# EvidenceLab constitutional design

Version: 1
Status: FREEZE WITH LIMITATIONS
This document is the constitutional specification. Implementation may choose language and storage. It may not add a duty, a verdict, or a trust path that this text does not allow.

---

## 1. Purpose

EvidenceLab should exist.

Testing, CI, observability, monitoring, and the system’s own audit trail answer a different question from the one a relying party actually has. Those systems are useful. They are also operated by the same organisation, the same release process, or the same process that can produce the behaviour under dispute. A green suite, a dashboard, or an exported log is a statement by the system about itself. It can be skipped, stubbed, sampled into friendliness, delayed, or rewritten by anyone who already controls the system. None of those tools has a duty to refuse a stronger conclusion than the observations support.

The engineering problem is narrower than “quality” or “compliance”:

A relying party must be able to take a named claim about a specimen, a pre-committed rule, and a body of observations, and obtain a verdict that is recomputable, that stays `undetermined` when observations are missing or in conflict, and that the specimen’s operators cannot rewrite after it has been sealed.

That is not a test runner. It does not execute the claim into existence. It is not a monitor. It does not sit on a production control loop. It is not an auditor’s narrative. It is a seal plus a decision procedure with one authorised verdict vocabulary.

If the observations are allowed to be entirely self-reported, EvidenceLab adds nothing and must not be built. The product is justified only when at least one observation required for `supported` is produced by an instrument or a third party that does not share a trust root with the system under evaluation.

## 2. Constitutional responsibilities

Mandatory, permanent:

1. **Seal.** Accept observation bytes and store them so that any later change is detectable. Record who submitted them, when the lab received them, and the instrument or party identity that signed them.
2. **Rank.** Classify each observation by the trust root that produced it, using the fixed ranking in section 4. Ranking is not a judgement of whether the content is true.
3. **Assess.** Given a claim, a rule version, and a sealed corpus, emit exactly one of `supported`, `unsupported`, or `undetermined`. The function is pure. The same inputs always yield the same verdict.
4. **Cover.** Every verdict names the observations the rule required, which were present, which were missing, and which conflicted. Coverage is part of the verdict, not an attachment that may be dropped.
5. **Certify.** Sign the verdict bound to the corpus hash, the rule hash, the claim text, the specimen identity, and an expiry. Signing `undetermined` is a normal output, not a failure of the product.
6. **Suspend.** Append a suspension when a sealed certificate must cease to be relied upon. Do not delete the certificate.
7. **Verify.** Provide an offline checker that recomputes the verdict from the corpus and checks the signature and the suspension head. The checker does not need to trust a live EvidenceLab service.

Explicitly not responsibilities of EvidenceLab:

- Building, deploying, configuring, or operating the system under evaluation.
- Deciding whether an action in that system is allowed to happen.
- Generating product requirements, invariants for the system to obey, or tests for the system’s authors to run. Rules consumed here are inputs, written and versioned outside the seal, then hashed.
- Paging on-call staff or closing incidents.
- Declaring a system “safe”, “correct”, or “compliant” beyond the claim text in the certificate.
- Repairing, compensating, or replaying the system under evaluation.

Optional tooling, outside the constitution, deletable without weakening it:

- A human-readable rendering of a certificate.
- An index that can be rebuilt from the seal.
- Shelves, accounts, and access control for lab staff. Useful operationally. Not part of what a certificate means. The offline verifier ignores them.

## 3. Independence

EvidenceLab evaluates a specimen. It must not be part of the specimen.

Trust roots that may never be the same key, the same credential store, or the same administrative write path:

- The system under evaluation: its release keys, its log keys, its operator break-glass.
- Instrument keys: keys held in lab-operated collectors or in a named third party that signs observations.
- The issuer key: the key that signs certificates and suspensions.
- The relying party’s own copy of the suspension head and of certificates it has accepted. The lab must not be the only store of a certificate after issue.

Lab operators may have access to run instruments. They must not have a credential that can both create an observation and sign a certificate. Those are two duties. One person holding both is a standing suspension of independence for any certificate they touched, recorded as such.

Internal state of the system under evaluation:

- EvidenceLab may accept it only as an exhibit: bytes submitted for sealing, ranked as self-report.
- EvidenceLab must not be given a privileged channel into the live system and treat what comes back as ground truth. A direct read with the system’s own administrator credential is still self-report, because the system and its operators can shape the answer. Rank does not improve because the lab pulled rather than the vendor pushed.
- Rules are forbidden from treating self-report as sufficient for `supported`. The assessor enforces this even if a rule author asks.

Independence is demonstrated, not asserted:

- The certificate names the specimen by content hash of the artefact and configuration that were observed, not by a marketing name.
- Every observation required for `supported` carries an instrument or third-party signature whose key is not on the system’s trust root. The verifier checks that separation against a published key list signed by the issuer before the evaluation window.
- A party who was not in the lab recomputes the verdict with the offline verifier from the sealed blobs, the rule text, and the certificate. If they cannot, there is no qualification, regardless of what the lab claims.
- The network path used by lab instruments is documented in the corpus. An instrument that receives its inputs only through the system’s control plane is ranked as self-report.

## 4. Evidence

Evidence is a sealed byte string together with: a content hash, the lab’s receive time, a declared subject, a producer identity, a signature by that producer, and a rank. Nothing else is evidence. A graph, a ticket, a recollection, or a model output is not evidence until it is reduced to those bytes and sealed. After that, it is evidence of what was sealed, not evidence that the contents are true.

Not evidence:

- An unsigned dashboard, a slide, or a verbal assurance.
- A metric with no raw observation behind the hash.
- The system’s statement that a test passed, when the lab did not run or witness the run.
- A summary that replaces the bytes it summarises. Summaries may be stored as exhibits. They cannot satisfy a rule.
- Absence. Absence is a coverage gap, not an observation of a negative.
- The certificate itself. A certificate is a verdict about evidence, not evidence about the specimen.

Rank, highest first. Rank is about who can lie, not about how precise the bytes are.

1. **Lab-witnessed effect.** An instrument the lab operates submitted an input and recorded the output on a channel the system’s operators cannot rewrite. Example: the lab sent a request and retained the response, or observed an external side effect on a medium it controls.
2. **Independent instrument.** A collector whose signing key is not held by the system’s operators, observing without using the system’s administrator credentials. The instrument build hash is in the corpus.
3. **Third-party record.** A signed record from a party that is not the vendor and not the lab, whose own trust root is named. Example: a statement from an external system of record. This ranks below lab-witnessed because the third party can be wrong or in collusion, but it does not share the vendor’s key.
4. **Sealed specimen.** The artefact, configuration, or rule text hashed at intake. This establishes identity. It does not establish behaviour.
5. **Self-report.** Logs, metrics, traces, database dumps, or screenshots produced by the system or its operators. Admissible as exhibits. Incapable of supporting a claim by themselves.

Conflicts:

- Two sealed observations that the rule says are about the same subject and that disagree are a conflict record. The assessor does not pick a winner, average them, or drop the older one.
- Default rule, mandatory: if a required subject is in conflict, the verdict is `undetermined`. A rule may say that a specific contradiction means `unsupported` (the claim is false), but it must say so in its text, and that mapping is hashed. A rule may not say that conflicts are ignored.
- Later observations do not erase earlier ones. Both remain in the corpus.

Uncertainty is only the verdict `undetermined`, plus coverage that lists gaps, late arrivals outside the window, and conflicts. There is no confidence score. A number would invite a threshold that hides a gap.

## 5. Qualification

Qualification is a signed certificate whose verdict is `supported`, whose coverage shows every required observation present and unconflicted, whose specimen hashes match, whose rule hash matches a published rule, and whose suspension head does not list it, and whose expiry has not passed.

What it proves:

- At issue time, this corpus and this rule produced `supported` under the assessor.
- The corpus hash still matches the bytes the relying party holds.
- The issuer key signed that binding.

What it can never prove:

- That the claim remains true after expiry or outside the specimen hash.
- That no other claim about the same system is true.
- That instruments saw everything that mattered. Completeness is only completeness against the rule’s required list.
- That a malicious lab did not invent rank-1 bytes before they were first sealed. See limitations in the freeze decision.
- That production matches a specimen that was not the production artefact. If the hashes differ, the certificate does not speak about production.
- Safety, legality, or fitness for a purpose not written as the claim.

Suspension is appended, signed, and never deleted, when any of the following hold:

- A sealed observation inside the corpus is shown to be altered or its producer key revoked for the window in question.
- A required observation is later contradicted inside an evaluation the lab has accepted as additional evidence about the same specimen and window.
- The rule version is withdrawn.
- Independence was breached for that evaluation (shared trust root, dual-duty operator, instrument path through the system control plane).
- The issuer key is compromised. All unexpired certificates are suspended.

Re-earning is a new corpus, a new assessment, and a new certificate. The suspended certificate remains in the seal. Nothing is “reopened” in place. A relying party that cannot fetch a current suspension head must treat the certificate as not qualified. Silence from the lab is not continued qualification.

## 6. Failure

| Condition | Behaviour |
| --- | --- |
| Evidence conflicts | Keep both. Verdict `undetermined`, or `unsupported` only if the rule text says this contradiction falsifies the claim. |
| Evidence disappears from the lab store | Hash mismatch. Any certificate bound to the missing bytes is suspended. Reconstruction from a relying party’s copy is a new intake, not a silent repair. |
| Evidence arrives late | Append it. It does not alter an issued certificate. If the rule’s window has closed, it cannot complete coverage for that certificate. A new assessment may use it under a rule that allows the later window. |
| The system lies | Self-report is ranked so that the lie cannot reach `supported`. Independent contradictions produce `undetermined` or `unsupported` per the rule. |
| Observers disagree | Conflict, as above. No vote, no quorum of opinions. |
| Observations are incomplete | `undetermined`. Coverage lists the holes. |
| Instruments fail | The missing observation is a gap. The lab does not substitute a log from the system. |
| Network partition | Intake waits. There is no local “pass” cache. A certificate that cannot be checked against a suspension head is not currently qualified. |

The product must never guess. It must not impute a missing observation, interpolate a gap, prefer the newer reading, or treat “probably fine” as `supported`. If the rule does not decide, the verdict is `undetermined`.

## 7. Outputs

Canonical outputs are exactly these four. Anything else is a rendering and may be discarded.

1. **Manifest.** Ordered list of sealed objects: hash, rank, producer, receive time, subject, signature identity. The manifest’s own hash is the corpus id.
2. **Assessment.** Claim text, specimen hashes, rule hash, verdict, coverage (present, missing, conflicting). Recomputable from the manifest, the blobs, and the rule.
3. **Certificate or refusal.** A signed assessment. A refusal is an assessment whose verdict is `unsupported` or `undetermined`, signed the same way. The lab must not issue an unsigned narrative in its place.
4. **Suspension record.** Certificate id, reason, time, issuer signature. Append-only. The head hash is what verifiers fetch.

A proof pack is a bundle of the manifest, the blobs it names, the rule text, and the certificate. It is not a separate kind of truth. Replay means running the verifier on that bundle plus a current suspension head. Invariants of the system under evaluation are not an output of EvidenceLab. Failure reports are renderings of coverage and conflicts.

## 8. Invariants

1. Sealed bytes are not rewritten. Correction is a new object.
2. A certificate names one corpus hash, one rule hash, one claim, one specimen identity, one expiry.
3. The verdict is a pure function of those inputs and the ranking rules in this document.
4. `supported` is impossible if any required observation is missing, in conflict (unless the rule explicitly maps that conflict to `unsupported`), or ranked only as self-report.
5. `undetermined` is never rendered, exported, or re-signed as `supported`.
6. Coverage is inside the signed assessment. A certificate with no coverage is invalid.
7. Suspension does not delete the certificate or the corpus.
8. Failure to present a fresh suspension head means the certificate is not currently qualified.
9. Issuer keys, instrument keys, and the system under evaluation’s keys are distinct. A corpus that violates this cannot yield `supported`.
10. The assessor has no network and no clock of its own. Receive times are fields in the sealed records, not reads of “now” at assessment time.
11. Duplicate submission of the same bytes is the same hash. It is not a second independent observation. Freshness requires a distinct instrument record inside the rule’s window, with its own signature and nonce.
12. The offline verifier implements the same function as the assessor. Disagreement between them is a lab defect and suspends issue of further certificates until the defect is recorded.
13. Operators cannot satisfy a coverage requirement by writing a free-text note.
14. Optional tooling cannot create a canonical output. If it is not in section 7, it is not authoritative.

## 9. Architecture

Five components. Each has one responsibility. No component is added for scale, hosting, or product surface.

**Sealer.** Append-only store of blobs and manifest entries. Refuses overwrite. Emits corpus hashes. Does not interpret claims.

**Instruments.** Lab-operated or named third-party collectors. Produce signed observations. Do not assess, do not hold the issuer key, do not use the system’s administrator credentials.

**Assessor.** Pure function. Inputs: manifest, blobs, rule text, claim, specimen identity. Output: assessment. No disk authority beyond its inputs, no network, no issuer key.

**Issuer.** Signs assessments and suspensions with the issuer key. Refuses to sign an assessment that is not the assessor’s output for that corpus. Does not collect observations.

**Verifier.** Offline. Recomputes the assessment, checks issuer signatures, checks key separation, checks expiry, checks the suspension head. Used by relying parties. Does not contact the system under evaluation.

Durability of the seal is copies of the append-only log, including a copy held by the relying party or by a second administrative domain. That is backup, not a distributed protocol. Consensus, replication quorums, and a live highly available service are not required for the constitution. Qualification already fails closed when the suspension head cannot be fetched.

The assessor is not a service that must be up for the certificate to remain meaningful. Meaning lives in the signed bundle and the verifier.

## 10. Threat model

**Malicious or buggy system.** It can forge rank-5 exhibits. It cannot move them to a higher rank. Claims that depend on rank 1–3 remain `undetermined` if those observations are missing. A lie that contradicts a lab-witnessed effect becomes a conflict, not a pass.

**Dishonest vendor.** Same as a malicious system, plus pressure to weaken rules. The assessor, not the vendor, enforces the self-report ban. A weakened rule that marks self-report as sufficient is an invalid rule and cannot be signed into `supported`.

**Malicious lab operator.** Can refuse to observe, which yields `undetermined`, not a pass. Can attempt to invent instrument bytes before they are sealed. Split duties make this require both an instrument key and the issuer key. This is the residual the freeze does not remove: a lab that holds both, or that colludes across the split, can fabricate a corpus and sign it. Relying parties detect that only by re-running instruments they operate, or by requiring a third-party rank-3 record the lab cannot mint. EvidenceLab must say so on the certificate: independence is from the system under evaluation, not a proof that the lab is honest.

**Corrupted evidence.** Hash mismatch. Verifier rejects. Issuer suspends certificates bound to the corpus. No repair-in-place.

**Replay of old observations.** Same bytes are the same hash and do not add a new witness. A rule that requires an event inside a window accepts only records whose sealed receive time and nonce fall in that window. Replayed captures from an earlier window do not fill it.

**Delayed observations.** They append. They do not mutate certificates. Late contradiction of a live certificate causes suspension, then a new assessment if someone wants one.

**Network partition.** No guessed verdict. Intake stalls. Verification without a suspension head fails closed.

**Partial failure.** Missing blob, missing signature, missing rank, or crashed instrument: coverage gap, verdict `undetermined`. The issuer may sign that refusal. It may not fill the gap.

**Compromised issuer key.** Publish suspension of all unexpired certificates under that key, using a pre-committed successor key that was named before the incident and is not kept online next to the issuer key. Until the successor publishes that suspension head, verifiers that still trust only the old key must reject certificates they cannot confirm. Practical rule in the verifier: if the successor key is known and the head is unreachable, fail closed.

Survivability is fail-closed verdicts and recomputation. It is not high availability of a laboratory service.

## 11. What Absolutely Does Not Belong

- Any authority over the system under evaluation: deploy, config push, feature flag, incident command, or “stop the line” as a hidden side effect. EvidenceLab reports. Others act.
- Pass/fail of the vendor’s own test suite as a rank above self-report.
- Scores, percentages, risk ratings, and colour codes that compress `undetermined` into a maybe.
- Models that judge evidence, fill gaps, or write certificates. A model may sit outside the lab and draft a rule. The draft is inert until hashed as rule text and run through the assessor.
- Blockchains, zero-knowledge proofs, trusted execution, confidential computing, eBPF, and new consensus. None of them separates the lab from the vendor or stops a lab from lying before the first seal. The seal is an append-only hash log and a signature.
- Formal proof of the specimen as a substitute for observations. Proofs are about a model. They may be sealed as exhibits if someone wants them stored. They are not observations of behaviour.
- Runtime verification inside the system under evaluation. That instrument shares the system’s fate and is self-report unless the lab operates it on a copy the vendor cannot modify, in which case it is already just an instrument.
- Dashboards, chat bots, ticket sync, and workflow engines as constitutional components.
- A shared trust root, a shared database, or a plugin that runs inside the system under evaluation and “streams the truth”.
- Editing a certificate, a manifest, or a suspension.
- A verdict vocabulary larger than `supported`, `unsupported`, and `undetermined`.

## 12. Hostile self review

**“This is a folder with a script.”** A folder does not enforce rank, does not refuse self-report, and does not fail closed when a suspension head is missing. Those three behaviours are the product. Remove them and EvidenceLab should be rejected as unnecessary. They stay.

**“A malicious lab forges rank-1 evidence.”** True. Split keys raise the cost and make a single on-call login insufficient. They do not create an honest lab. Re-observation by the relying party is the only check, and it is often impractical. This is a fundamental limit, not a missing component. Adding trusted hardware or a chain does not remove a lab willing to sign false bytes. The architecture is not extended. The limit is recorded in the freeze.

**“Rules will be written to avoid hard observations.”** Addressed by an invariant: invalid rules cannot yield `supported`. The hostile remainder is a claim that was worded to be trivial. EvidenceLab does not judge whether the claim was worth making. The certificate quotes the claim in full so a relying party can see a vacuous sentence. No “materiality” engine. That would be a guess.

**“Late truth is ignored.”** Issued certificates are immutable, so a late contradiction must suspend rather than edit. That component is mandatory. Without the suspension head and fail-closed fetch, the design is unsound. It is in sections 5, 8, and 9.

**“Internal state would make assessments more accurate.”** Accuracy from a channel the adversary controls is how monitoring gets fooled. Direct consumption stays forbidden. Exhibits stay rank 5.

**“You need many services for millions of observations.”** No. Volume is copies of an append-only log. If the log is large, the manifest still hashes to one id. Distribution is not a constitutional requirement. Rejected as complexity.

**“Conflicts should be resolved by seniority of observer.”** That hides disagreement and manufactures certainty. Rejected.

**“The verifier will drift from the assessor.”** Invariant 12 suspends issuance on drift. One implementation is the intended construction: the issuer runs the same verifier binary that relying parties run, and signs its output. A second implementation is not required and is a likely source of false qualifications. The constitution therefore requires a single assessment implementation shared by issuer and verifier. Divergence is a bug, not a feature of diversity.

**“Operator notes will become a back door.”** Forbidden by invariant 13.

No further component survived review. The weaknesses that remain are limits of what observation can mean, not missing machinery.

## 13. Freeze decision

**FREEZE WITH LIMITATIONS**

EvidenceLab is a real product because the failure mode that matters is a relying party treating self-report as confirmation. CI, monitoring, and vendor audit do that by construction. A small seal, a ranked corpus, a pure assessor, a signed verdict, a suspension log, and an offline verifier close that gap and nothing wider.

Limitations, permanent:

- Qualification speaks only about a named claim, a named specimen hash, a named rule, and a sealed corpus inside an expiry window.
- It does not prove the lab honest. A party that controls both instrument keys and the issuer key can seal fiction. Relying parties who need to survive that must witness instruments themselves or require an independent third-party record.
- It does not prove completeness beyond the rule’s list.
- It does not operate, correct, or halt the system it evaluates.
- `undetermined` is a successful run of the product. Implementations that treat it as an error to be cleared will be in breach of this constitution.

Rejected alternative: no product, only a written procedure. A procedure does not stop a later editor from dropping a conflicting file or promoting a self-report to a pass. The seal and the verifier are small enough to be the whole architecture, so they are worth freezing.
