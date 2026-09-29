# Constitutional contract

Status: frozen as the contract implementations must satisfy.
Corpus reviewed: the consequential-action runtime RFC and its hostile review; the EvidenceLab RFC and its hostile review. Both are implementation candidates, not truth.
Not in the corpus, and therefore not adopted: Passport, Witness, continuous-governance machinery, and any runtime design that is not written in those two documents.

This document outlives implementations. Language, topology, storage, messaging, and deployment are unspecified on purpose.

---

## Part 1 — Constitutional consensus

Only responsibilities that both designs require, or that one design requires and the other does not contradict, and that fail the deletion test in Part 2.

**C1. Record authority before a stronger act.**
An authorization to cause an external effect is durable before any byte of that effect is sent. A qualification verdict is bound to a sealed corpus that already exists. Survived because both hostile reviews treat a claim issued before the record as fabrication. Guarantee: no effect and no qualification floats free of a prior durable record. Removing it allows a sender or an issuer to act, then write history to match.

**C2. Append-only history.**
Corrections are new records. Authorizations, observations, verdicts, and withdrawals are not edited or deleted. Survived because rewrite makes replay and audit refer to a different system than the one that acted. Guarantee: a later operator cannot make the past agree with them. Removing it collapses audit into a story.

**C3. Uncertainty is an explicit outcome.**
Missing, late, ambiguous, or conflicting observations stay visible. They are not converted into success, failure-that-may-be-retried, or a score. Survived because both reviews found that guessing is how external ambiguity becomes a second effect or a false qualification. Guarantee: a reader can tell “not known” from “known”. Removing it makes the record lie by omission.

**C4. Decisions are functions of recorded inputs.**
Admission, settlement, and qualification use a named rule version and records already sealed. They do not depend on a hidden read of the live world or on an unrecorded clock. Survived because hidden inputs make the same log replay to a different decision. Guarantee: the decision can be recomputed. Removing it makes the log incomplete on purpose.

**C5. Separation of duties that change claims.**
The credential that can cause an external effect cannot be the credential that defines the rule. The credential that can mint the observations used to support a claim cannot be the only credential that signs that claim. The system under evaluation does not hold the keys that certify it. Survived because a single credential that both acts and certifies makes the record self-report. Guarantee: forging a stronger claim requires collusion across duties, not one login. Removing it makes independence unauditable.

**C6. Replay by someone other than the actor.**
A party who holds the records and the rule text can recompute the decision or the verdict without trusting the live service that issued it. Survived because both designs treat “the service is up and says so” as insufficient. Guarantee: audit does not depend on the operator’s continued cooperation beyond the bytes already handed over. Removing it leaves only testimony.

**C7. Claims are bounded.**
Every authorization names the rule version and the intent it covers. Every qualification names the claim, the specimen identity, the rule, the corpus, and the limit of what is not proved. Expiry, suspension, or an explicit residual-uncertainty record ends reliance without erasing the original. Survived because unbounded claims (“exactly once”, “the system is safe”) were rejected under hostile review in both RFCs. Guarantee: a reader cannot honestly promote the record into a wider promise. Removing it turns a narrow truth into marketing.

**C8. A new external effect is a new authorization.**
Compensation, correction, and a second attempt after ambiguity are not undo. Where a duplicate effect is not known to be harmless under a qualified rule, ambiguity does not authorize another send. Survived because automatic undo and retry-from-uncertainty were deleted under review and never replaced. Guarantee: the log cannot hide a second consequential effect inside recovery. Removing it reintroduces a second effect that the original authorization did not name.

**C9. Self-report cannot independently qualify.**
Records produced by the system under evaluation, or by its operators using that system’s authority, may be stored. They cannot by themselves support an independent qualification of that system. Survived because otherwise a lab or an auditor is a copy of CI. Guarantee: qualification means someone other than the specimen’s trust root witnessed what the rule required. Removing it makes EvidenceLab and any external audit theatre. This duty does not forbid the governing runtime from using its own log to settle its own actions. Settlement is not independent qualification. See Part 3.

## Part 2 — Constitutional elimination test

| Candidate | Delete it? | Result |
| --- | --- | --- |
| Durable record before effect or before certification | No | Send-then-log and certify-then-seal become possible. C1. |
| Append-only history | No | Past authorizations and verdicts can be aligned with a later story. C2. |
| Explicit uncertainty | No | Ambiguity is cleared by retry or by a pass. C3. |
| Decisions from recorded inputs only | No | Replay diverges. C4. |
| Duty separation across act, rule, observation, and signature | No | One credential can invent the claim it certifies. C5. |
| Offline recomputation | No | Only the live operator can say what was decided. C6. |
| Bounded, withdrawable claims | No | Exactly-once and “safe” return as implied promises. C7. |
| New effect requires new authorization | No | Recovery performs a second consequential act unnamed in the first authorization. C8. |
| Self-report ban for independent qualification | No | Qualification collapses into the specimen’s own telemetry. C9. |
| A particular state-machine diagram and state names | Yes | Any progression that preserves C3 and C8 holds the guarantee. Implementation. |
| Idempotent / compare-and-set / single-shot labels | Yes | They are one way to record whether a duplicate is harmless. The rule and its qualification are C7 and C8. The labels are implementation. |
| Separate observer and reconciler roles | Yes | Both only append attributed observations. One intake duty suffices. |
| Partition log as the only evidence store | Yes | C2 and C6 require a seal and a copy a relying party can hold. They do not require that seal to be the governor’s log. |
| A separate laboratory product | Yes, as a product | C9 requires an independent trust root for independent qualification. It does not require a product named EvidenceLab. |
| Raft, quorum size, leader leases | Yes | C1 requires durability before send. How durability is obtained is implementation. A single copy fails the durability guarantee in practice, but the constitution does not name the algorithm. |
| One active signer versus a specific gate process | Partial | Two simultaneous authorities that can both release a new effect for the same authorization break C8. The ban on a second releaser is constitutional. The process layout is not. |
| Local halt flag, budgets, ramps | Yes | Halt-before-new-release is an operational control that implements C8’s refusal to send. Timing budgets are implementation. The constitution does require that loss of the durability needed by C1 stops new releases. |
| Five-level evidence ranking | Yes | The constitutional cut is C9: self-report is not enough. Finer ranks are a lab policy. |
| Hash chains, Merkle trees, WORM buckets, HSMs | Yes | They are ways to detect rewrite and to hold keys. C2 and C5 require the properties, not the mechanisms. |
| Specific certificate file formats and verdict spellings | Yes | Three outcomes are required in meaning: holds, does not hold, not determined. Spelling is implementation. |
| Human approval of every action | Yes | Both RFCs govern by reviewed rules. Per-action approval is an implementation of a rule, not a duty. |
| Global total order | Yes | Neither guarantee needs it. Cross-domain atomicity was rejected. |
| End-to-end exactly-once | Already absent | Deleting the refusal would break C7. The refusal stays. |
| Passport, Witness | Not present | Nothing to delete. Not constitutional. |
| Continuous governance loop | Not present | Adding it is not required for C1–C9. Research. |

## Part 3 — Remaining genuine disagreements

**Knowledge versus truth.**
The runtime RFC says an action becomes `settled` when a settlement rule is satisfied. The lab RFC says a certificate is not truth about the world. Reasonable engineers hear “settled” as “the external system did this”. The hostile reviews agree the records cannot prove more than what was observed under a rule. Evidence favours writing every terminal success as “the named rule is satisfied by these records”, not “the world is so”. No further implementation is required to adopt that wording. Implementations that use “settled” must define it that way.

**Commitment as a protocol versus a plane.**
One RFC binds commitment to a gate, a log quorum, and a leader. The other binds it to “seal, then sign” with no topology. Reasonable engineers want a visible plane so a sender cannot skip the record. The deletion test keeps the protocol (C1) and the single-releaser rule, and drops the plane. Further implementation evidence is required only to show a chosen deployment actually refuses to send when the record is not durable. That is qualification of an implementation, not a constitutional gap.

**Progression as a responsibility versus an implementation.**
The runtime RFC freezes named states. The lab RFC has no action progression. Engineers who have operated retries will want the diagram in the constitution so a later team cannot add an edge. The diagram is still one implementation. The constitution keeps the banned edge (C8), not the picture. Implementation evidence required: tests that the banned edge does not exist in that codebase.

**Observation versus reconciliation.**
Push and pull are the same append in both RFCs. Splitting them feels cleaner and creates two writers. Evidence does not favour a split. No research programme. An implementation may split them if both still only append and neither can settle by itself.

**Evidence ownership.**
The runtime RFC stores evidence in the same log as authority. The lab RFC says that log is self-report if used to qualify the runtime. Both statements hold for different questions. Unresolved only at the boundary: whether a deployment that does not seek independent qualification may treat the governor’s log as its only record. Evidence favours yes for operation, no for independent qualification. C9 writes that down. A deployment must not describe operational settlement as independent qualification.

**Recovery ownership.**
Who runs restore, and whether a supervisor role exists, is unspecified once C1 and C8 hold: no new release until durability is back, and no silent second effect. Runbooks are implementation. No constitutional owner.

**Passport and Witness.**
They are not specified in the corpus. Reasonable engineers may believe a portable action identity or a third-party witness is required for C5 or C6. Current evidence does not show a guarantee that fails without those names. Adopting them now would invent an architecture. They stay out until a design shows a broken guarantee.

**Continuous governance.**
A standing process that re-opens decisions forever is not in either freeze. Engineers may want it for long-lived risk. Evidence says qualification is a bounded statement (C7). Continuous re-assessment is research, and it must not mutate old records.

**Qualification scope.**
One RFC qualifies a domain’s permission to release a class of effect. The other qualifies a historical claim about a specimen. Engineers will try to use one certificate for both. They are the same pattern and different questions. Substituting them breaks C7. No further research is required to keep them apart. Research is required only to decide the rule text for a particular domain or claim.

## Part 4 — Assumptions register

**A1. Durability means a later reader sees the same bytes.**
Exists because C1 and C2 are otherwise undefined. Fails if the only copies are destroyed or silently replaced together. Guarantees become testimony. Qualification can show restore from an independent copy. It cannot stop an administrator who controls every copy.

**A2. A qualified rule that says “duplicate is harmless” is true of the real external system.**
Exists because C8 allows another release only under that rule. Fails when the external system applies twice despite the contract. The architecture still records both attempts; the external harm happens. Qualification that double-submits reduces the risk. It does not eliminate a rare external fault. On failure, that class of effect must lose permission for further automatic release.

**A3. Operators will not keep a side credential that causes the effect outside the record.**
Exists because C1 governs only paths that go through the record. Fails when incident response uses a direct login. The record no longer governs. Qualification can show credentials were removed. It cannot stop a later organisational exception. On failure, claims of governance for that domain are false.

**A4. Duty separation is real under pressure.**
Exists because C5 and C9 assume distinct trust roots. Fails when one break-glass holds instrument keys and issuer keys, or policy keys and release keys. Independent qualification and insider tamper-evidence weaken. The fault and mistake guarantees of C1–C4 can still hold. Qualification can show the keys are different people today. It cannot guarantee the next reorganisation.

**A5. Recorded inputs are the inputs the operator thinks they are.**
Exists because C4 makes bad rules faithfully deterministic. Fails when a stale or hostile exhibit is sealed and a rule treats it as sufficient. The decision is replayable and still wrong. Qualification of rule text reduces this. The constitution cannot judge whether a claim was worth making.

**A6. Relying parties will treat missing withdrawal information as “not qualified”.**
Exists because C7’s withdrawal is useless if old copies are trusted forever without a check. Fails if consumers cache a success and never look again. The bound evaporates in practice. Implementation can make the check mandatory in the verifier. It cannot stop a human with a PDF.

**A7. “One releaser at a time” is actually enforced.**
Exists because two releasers break C8 even if the log is perfect. Fails if two processes can both emit a new effect for one authorization. Implementation qualification must try to make both release. The constitution does not name the lock.

## Part 5 — Non-negotiable invariants

1. **Immutable history.** No record that authorized, released, observed, qualified, or withdrew is edited or deleted. Fundamental because audit and replay require the original bytes.
2. **Durable authority before external effect.** No release of a new effect without a durable authorization that names it. Fundamental because otherwise the effect is outside governance.
3. **One new release authority at a time per authorization.** Two parties must not both be able to release a new effect under the same authorization. Fundamental because C8 is unenforceable if they can.
4. **Ambiguity does not authorize another effect.** Unless a qualified rule states that a duplicate is harmless, an ambiguous outcome forbids another release under that authorization. Fundamental because retry is a second consequential act.
5. **Uncertainty stays written.** Missing, conflicting, and late facts are not rendered as success. Fundamental because hiding them is a false claim.
6. **No hidden decision inputs.** Recomputing from the sealed inputs and the named rule yields the same decision. Fundamental because otherwise the log is not the decision.
7. **Replay without the live actor.** A holder of the records and the rule can recompute. Fundamental because audit that needs the actor’s word is not audit.
8. **Bounded claims.** A qualification or an authorization states its subject, rule, and what it does not prove. Withdrawal is append-only. Fundamental because unbounded wording is how both RFCs failed review when they over-claimed.
9. **Self-report is not independent qualification.** The specimen’s own records cannot be the sole support of a claim to qualify it. Fundamental because otherwise qualification is the system praising itself.
10. **A further effect is a further authorization.** Undo, compensation, and manual correction that change the external world are new governed acts, or they are outside the constitution and must be described as outside it. Fundamental because implicit undo hides effects.

Invariants that did not survive as constitution: a named state set, a global order, a particular consensus, a requirement that every deployment include a separate lab product, a promise that halt recalls bytes already sent, a promise that a malicious holder of all certifying keys cannot lie before the first seal.

## Part 6 — Constitutional exclusions

These may appear in an implementation. They are not the contract.

- **Databases, Raft, actor runtimes, microservices, languages, containers.** Storage and process shape. C1 cares that the record is durable before release, not which engine did it.
- **Merkle trees and hash chains.** One way to detect rewrite. Another way that detects rewrite satisfies C2.
- **Passport and Witness implementations.** Not in the corpus. Naming them in the constitution would freeze an unreviewed design.
- **A gate process, a sealer service, an issuer service.** Duties may be modules, processes, or one binary, provided the credentials in C5 stay split and the single-releaser rule holds.
- **AI.** Neither RFC gives a model a key. A model may draft a rule outside the record. The draft means nothing until it is a sealed rule.
- **eBPF, blockchains, trusted execution, zero-knowledge proofs, confidential computing.** They do not establish C1–C9. The lab review and the runtime review both rejected them as substitutes for a seal and a bounded claim.
- **Observability tooling.** Telemetry operated by the specimen is self-report. It may be sealed as an exhibit. It is not a constitutional component.
- **Exactly-once protocols, two-phase commit across external systems, automatic saga undo.** They claim guarantees C7 and C8 refuse.
- **Scores that compress “not determined”.** They break invariant 5.
- **Per-action human approval and global workflow products.** Optional policy, not the contract. A workflow engine must not be where authority lives.

## Part 7 — Final constitution

This is the whole contract.

1. Before a system causes a new consequential effect outside itself, a durable authorization record exists that names that effect and the rule version used to admit it.
2. History of authorizations, releases, observations, qualifications, and withdrawals is append-only.
3. At most one authority can release a new effect for a given authorization at a time.
4. If it is not determined whether an effect occurred, the system does not release another under that authorization, unless a qualified rule records that a duplicate is harmless and that qualification is still valid.
5. Any further consequential effect, including one intended to correct an earlier effect, has its own authorization.
6. A decision to admit, to treat a rule as satisfied, or to qualify is a function of sealed records and a named rule. It has no hidden live input.
7. “Not known” remains distinguishable from “known”. Conflicts are retained. Missing observations are not filled in.
8. Someone other than the issuing service can recompute that decision from the records and the rule.
9. A qualification states the claim, the specimen, the rule, the corpus, and its limit. It can be withdrawn only by a new record. If withdrawal status cannot be checked, the qualification is not currently valid.
10. Records produced under the specimen’s own authority cannot be the sole basis for an independent qualification of that specimen.
11. The contract does not promise that the external world applied an effect once, that a party holding every certifying key cannot lie before first seal, or that a halt recalls an effect already released.

An implementation that violates any of 1–10 does not satisfy this constitution, whatever else it contains.

## Part 8 — Research roadmap

Research is not required to implement the constitution. Nothing here is on the constitutional critical path.

**Programme R1. Harmless-duplicate rules.**
Objective: decide, for one external system, whether a second apply is actually harmless. Success: a written rule and a double-submit test that either supports the rule or withdraws automatic re-release. Hostile qualification: inject timeout after apply, failover, and partial write. Evidence required: observed external outcomes, not the vendor’s claim. Adoption means a qualification record under clause 4, not a constitutional change.

**Programme R2. Single-releaser constructions.**
Objective: show a deployment enforces clause 3 under crash, pause, and split. Success: two deliberate attempts to release produce one new effect or a detected violation with no second effect. Hostile qualification: kill and freeze the releaser during the release critical section. Evidence required: those test results. Adoption stays inside implementation.

**Programme R3. Independent qualification practice.**
Objective: perform clause 9 and clause 10 on one governor without sharing its release keys. Success: a recomputed verdict whose supporting observations are not self-report, plus a withdrawal drill. Hostile qualification: attempt to qualify using only the governor’s log and show refusal. Evidence required: the corpus and the refusal. Adoption does not create a new constitutional product name.

**Programme R4. Insider and key-collapse drills.**
Objective: measure what remains true when one person holds both observation keys and signing keys, or both policy keys and release keys. Success: a clear statement of which clauses still hold. Hostile qualification: simulated break-glass. Evidence required: the drill record. This programme must not “solve” a malicious holder of all keys by adding a fashionable mechanism. If it cannot show a real separation that operators will keep, the assumption stays listed, not fixed.

**Programme R5. Witness and portable identity.**
Objective: determine whether any clause in Part 7 is false without a third-party witness or a portable action passport. Success criterion for adoption into the constitution: a concrete guarantee in Part 7 that an implementation cannot meet without that mechanism, demonstrated by a failed attempt. Until that evidence exists, these names stay out.

**Programme R6. Long-lived re-assessment.**
Objective: decide whether any relying party needs qualifications to be re-checked on a clock without mutating old certificates. Success: a procedure that appends new verdicts and withdrawals only. Hostile qualification: show an attempt to edit an old verdict and show it fails. Evidence required: one expired claim re-assessed. Not continuous governance as a new authority.

## Part 9 — Final independent assessment

1. **Internally consistent.** Yes. Operational settlement and independent qualification are different claims, and clauses 9–10 forbid using one as the other. Knowledge is not upgraded to world-truth. Recovery cannot smuggle a new effect.

2. **Responsibility versus implementation.** Yes. Durability, single release, recomputation, and independence are stated as obligations. Engines, topologies, and product names are not.

3. **Minimal.** Yes, against this corpus. Deleting any of clauses 1–10 recreates a failure both reviews already accepted as fatal. Clause 11 is a bound, not a duty. It stays so later editors do not widen the promise.

4. **Duplicated responsibilities.** No constitutional duty is duplicated. Settlement and qualification looked like duplicates and are different questions. Intake of observations is one duty even when an implementation splits push and pull.

5. **Highest architectural risk.** Assumption A3, then A4. The contract governs only effects and certifications that pass through it. A side credential, or a break-glass that collapses duties, makes the record look complete while the world is changed or certified outside it. No clause inside the software removes that.

6. **Freeze today.** Part 7, clauses 1–11, and the deletion results that keep Raft, product topology, Passport, Witness, and automatic undo out of the contract.

7. **Leave unfrozen.** Any state diagram, any storage or consensus choice, any rank ladder finer than self-report versus not, any particular external system’s duplicate rule, and any proposal for Passport, Witness, or continuous governance until R5 or R6 produces the evidence those programmes require.
