# Runtime for consequential actions

Status: frozen for implementation.
Scope: govern consequential actions on external systems this runtime does not control.
Target: production in 2028, millions of actions per day, ordinary engineering discipline.

A consequential action is one whose incorrect execution could cause financial loss, infrastructure damage, safety impact, legal consequences, or an irreversible external effect.

This document is the programme. It is not a research proposal.

---

## 1. Core philosophy

The runtime governs **intent**. It does not own **effect**.

External systems commit on their own terms. They can acknowledge and then lose the write, apply the write and lose the acknowledgement, apply it twice, or never answer. No protocol inside this runtime can promote those outcomes to exactly-once execution.

What the runtime can make exact is narrower and sufficient:

- An action is authorized at most once, under a named policy version, and that decision is durable before any byte is sent.
- Every outbound attempt is a signed projection of a committed authorization, never an invention of the sender.
- The runtime tells the truth about what it knows. Success is a settlement based on evidence. Silence is uncertainty. Uncertainty is a stable state, not an error to be cleared by retry.
- A human scales by approving policy and limits, not by approving each action. Individual judgment is reserved for thresholds, contradictions, and actions the external system cannot make safe to repeat.

Operator simplicity comes from a small state machine and a halt that fails closed. It does not come from hiding uncertainty.

## 2. First principles

1. **Default deny.** No committed authorization, no send.
2. **Commit before effect.** The authorization record is durable on a quorum before the gate will sign.
3. **Idempotency or no automatic retry.** If the external system cannot make a duplicate harmless, the runtime will not send twice.
4. **Uncertainty is terminal until evidence.** Timeouts, lost responses, and ambiguous acknowledgements enter `uncertain`. They do not enter `failed` and they do not re-enter `authorized`.
5. **Facts used to decide are recorded facts.** The authorizer does not perform hidden reads. Every input is an evidence record or a policy constant, named in the authorization.
6. **Effects are not undone by the log.** Compensation is a new consequential action with its own authorization. Some actions have no compensation path.
7. **Order is per partition, never global.** A partition is the unit of authority, halt, replay, and recovery.
8. **Determinism is replay of recorded inputs.** Same log prefix, same policy text, same decision. Clocks enter only as recorded values.
9. **Halt is local and immediate.** The gate stops signing when it cannot prove it is the leader, or when a halt is set. Bytes that have already left are `uncertain`.
10. **Authority is split.** Policy authors, authorizers, the signing gate, effectors, observers, and auditors are separate duties. No single credential can both change policy and cause an effect.
11. **Audit is a product of the same log.** It is not a side channel that can diverge from what was authorized.
12. **Boring consensus.** A replicated log per partition. No new consensus protocol.

## 3. Constitutional responsibilities

| Duty | Owns | Must not |
| --- | --- | --- |
| Partition log | Durability and order of records inside one partition. Linearizable commit. | Interpret policy. Talk to external systems. |
| Policy registry | Immutable policy versions and their signatures. | Authorize a particular action. Dispatch. |
| Authorizer | Deterministic admit / refuse against a committed policy version and recorded inputs. | Send bytes. Read the live external world. |
| Gate | Sign one dispatch token for a committed, unexpired, unhalted authorization. Persist the attempt record before returning the token. | Create an authorization. Retry a non-idempotent action. |
| Effector | Carry a signed token and payload to the external system. Return the transport result to the log. | Alter payload. Sign. Decide settlement. |
| Observer | Append evidence: callbacks, reconciliations, operator attestations. | Change authorization. Mark settled by itself. |
| Settler | Pure function from authorization, attempt, and evidence to `settled`, `uncertain`, or `contradicted`. | Call external systems. |
| Supervisor | Halt, dispatch budget, leadership, partition freeze. | Mint authorizations. Un-halt by deleting history. |
| Auditor | Verify hash chains, anchors, and duty separation. | Hold dispatch keys. |
| Operator | Approve policy versions, limits, and resolutions of `contradicted` or frozen single-shot actions. | Bypass the gate with a direct credential to the external system. |

The constitutional rule that keeps the design sound: **the only component that can cause an external effect is the effector, and it can send only a payload accompanied by a gate signature over a committed authorization.**

## 4. Runtime architecture

One partition is one replicated log plus a single active gate.

```
policy registry (immutable versions)
        |
        v
client intent --> authorizer --> partition log <-- observer
                                   |     ^
                                   v     |
                                 gate    settler
                                   |
                                   v
                                effector --> external system
                                   ^
                                   |
                               supervisor (halt, budget, lease)
```

Processes:

- **Log nodes** (three or five) run ordinary replicated consensus. They store the partition log. They do not have the dispatch signing key.
- **Gate** runs only on the current leader. It holds the partition dispatch key in memory while the leader lease is valid. On lease loss it zeroes the key and refuses. A standby cannot sign.
- **Effector pool** is horizontally scaled and stateless. It has network access and no signing key. A stolen effector can replay a token only within that token’s expiry and only for the payload the gate already bound. Tokens are single-use at the gate: the attempt count is committed before the token is released.
- **Authorizer** is pure code, invoked by the leader, replayable offline.
- **Settler** is pure code, invoked when new evidence arrives.
- **Supervisor** is the leader’s safety interlock, not a second control plane.

Cross-partition work is not a transaction. A parent action in partition P may authorize child actions in other partitions only as new intents, each with its own life cycle. The parent settles only on evidence of child settlement. There is no two-phase commit with external systems.

Administrative acts that change authority (policy publish, limit change, un-halt, manual settlement attestation) are themselves records in a small **control partition**, authorized by a stricter policy. Halt is the exception: it must also be a local, durable flag on the gate so it does not wait for a remote control-plane round trip. The log record of the halt follows immediately. If the log cannot take the record, the gate stays halted.

## 5. State model

An action has one progression. Records are append-only. The visible state is the fold of the log.

States:

1. `recorded` — intent appended, not yet decided.
2. `refused` — authorizer rejected. Terminal.
3. `authorized` — decision committed. No byte sent.
4. `dispatched` — gate has issued a token and the attempt record is committed. Not a claim that the external system received it.
5. `uncertain` — transport ambiguous, acknowledgement not sufficient to settle, or observation overdue past the action’s uncertainty deadline.
6. `settled` — settlement rule satisfied. Terminal for this action id.
7. `contradicted` — evidence conflicts with the authorization or with earlier evidence. Terminal until a new action or an operator attestation, both appended, never overwritten.

There is no `failed` state that permits another send. Transport failure with a clear “nothing was accepted” result, on an idempotent class, appends a retryable attempt **only while the authorization is still valid and the attempt budget remains**. That path stays in `authorized` until a token is issued. Any other failure becomes `uncertain`.

Action classes, fixed at admit time:

- **Idempotent.** External contract documents that the idempotency key suppresses duplicate effects. Automatic retry is allowed inside attempt budget and expiry.
- **Compare-and-set.** External contract documents a unique business reference such that a second apply is rejected and the first apply is observable. One automatic attempt. Ambiguity goes to `uncertain` and further sends are forbidden until observation.
- **Single-shot.** Neither of the above. The runtime admits it only under an explicit policy that accepts residual duplication risk. The gate issues at most one token, ever. Ambiguity freezes that business reference in the partition: no later action may target it until an operator appends an attestation of what the external world actually did.

Compensation is not a state. It is a new action id whose intent names the predecessor.

Invariants:

- A gate signature exists only for an `authorized` record that was committed first.
- Attempt count never decreases.
- Single-shot attempt count is at most one.
- `settled` and `refused` have no later dispatch token.
- Policy version on the authorization is immutable.

## 6. External execution model

Outbound request carries: action id, partition id, idempotency key (stable, derived from action id and business reference), payload hash, payload, authorization expiry, policy version, and gate signature over that tuple.

Rules:

- The effector treats the external system as hostile and unreliable. It does not interpret business success except to return bytes and status to the log.
- Connection failure before a request byte is written may be retried by the effector without a new token, within token expiry. This is transport, not a new attempt.
- Any timeout after the request may have been written is `uncertain`. The effector must not “try again” with the same token if the external system might have consumed it, and must not request a new token.
- HTTP 200, or any application acknowledgement, is evidence. It is settlement only if the action’s settlement rule says that acknowledgement is enough. For money movement, infrastructure mutation, and safety effects, the default settlement rule is **observation of the resulting external state**, not the acknowledgement.
- Clock on the external system is not trusted. Expiry is evaluated by the gate at sign time and by the external system only as a hint.
- Credentials used by the effector are scoped per external system and per partition. They are not the gate key. Rotating them does not rewrite history.
- The runtime does not participate in XA, two-phase commit, or the external system’s internal transaction. If the external system offers an idempotency key, use that. If it offers an outbox or a statement API, use that for observation. Do not wrap it in a distributed transaction.

## 7. Observation model

Two channels, same record type:

- **Push.** Signed callback from the external system, or from a connector the runtime operates. Signature proves origin, not truth.
- **Pull.** Reconcile job, under the supervisor’s budget, reads external state by business reference and appends what it saw, including “not found”.

Observation rules:

- Absence of evidence is `uncertain` once the deadline passes. It is not `refused` and not `settled`.
- “Not found” after a successful acknowledgement is `contradicted` or remains `uncertain`, according to the settlement rule, until the reconcile window closes. It is not automatically retried.
- Observers are rate-limited per external system. A down dependency must not be probed at the dispatch rate.
- Every observation names its source, time received (local monotonic plus wall clock as recorded values), and the raw payload hash.
- Operator attestation is an observation with a different duty and a stronger policy. It is how single-shot freezes end. It is logged, attributed, and not editable.

The settler runs on each new evidence record. Its outputs are log records. Two settlers replaying the same prefix produce the same settlement.

## 8. Evidence model

The partition log is the evidence store. Each record includes the hash of the previous record. Checkpoints record the hash at a sequence number.

Durability:

- Committed means replicated to a quorum of log nodes in the partition, fsync’d, before the gate may sign.
- A checkpoint hash is copied on a fixed interval to a **WORM archive in a separate administrative domain** from the runtime operators. The archive cannot be rewritten by the gate key or the log-admin credential. This is an ordinary locked object store with retention and legal hold, operated by the audit function. It is not a chain of blocks and it has no token.
- Loss of the archive does not stop dispatch. It is an audit incident. Loss of log quorum stops dispatch. That asymmetry is deliberate: availability of effect yields to durability of intent.

What is provable later:

- The policy bytes that authorized an action.
- The inputs the authorizer saw.
- Whether a token was issued, when, and with what expiry.
- What the effector claimed it sent and received.
- What observers appended.
- Whether the hash chain matches the anchor the auditor holds.

What is not provable:

- That the external system’s current state matches reality beyond the observations recorded.
- That an operator with access to the external system, outside this runtime, did not act. That path is closed operationally by removing direct credentials, not by a cryptographic claim.

Retention: full payloads stay for the regulatory window of the domain. After that, payload bodies may be dropped if the hash, decision, settlement, and anchor remain. The chain head is never dropped.

## 9. Recovery model

**Leader loss.** A new leader is elected. It loads the log. It does not re-sign any attempt already recorded. Actions in `authorized` may be signed if still unexpired, unhalted, and inside budget. Actions in `dispatched` or `uncertain` are not resent. Idempotent actions with a clear unsent failure may retry under the normal rule.

**Quorum loss.** Gate halts. No signatures. Restore from surviving replicas or from backup plus archive verification. After restore, the partition stays halted until a reconcile has been appended for every `dispatched` and `uncertain` action, or an operator explicitly accepts residual uncertainty as a logged control action. There is no silent resume.

**Effector loss.** Irrelevant. Effectors are stateless. In-flight tokens expire. Expired tokens that may have been sent leave the action `uncertain`.

**External outage.** Supervisor cuts the dispatch budget to zero for that dependency. New authorizations may still be committed if policy allows queuing, but the gate will not sign. Queued authorizations expire rather than burst when the dependency returns. Recovery of sending is a ramp of the budget, not a backlog dump.

**Bad policy.** Already-committed authorizations keep the policy version they were decided under. The supervisor stops new admits against that version. Rollback is “publish a corrected version and route new intents to it”, not mutation of old decisions.

**Contradictions and single-shot freezes.** The partition continues to serve other business references. Only the frozen reference is blocked. Unfreeze is an operator attestation record.

**Region loss.** Partitions are placed across failure zones so quorum can survive one zone. Surviving a full region is a placement decision per domain: either a remote replica, or an accepted halt until restore. The programme does not promise multi-region active dispatch for the same partition. Two active gates would be a soundness bug. Geographic scale is many partitions, one gate each.

## 10. Progression model

Progression is the fold of the log. Legal moves:

- `recorded` → `refused` | `authorized`
- `authorized` → `dispatched` | `refused` (expiry or halt before any token) 
- `dispatched` → `uncertain` | `settled` | `contradicted`
- `uncertain` → `settled` | `contradicted`
- `contradicted` stays contradicted. A later attestation may append a linked resolution record; it does not delete the contradiction.

No other edges exist. In particular there is no edge from `uncertain` or `dispatched` back to `authorized` for single-shot and compare-and-set classes.

Expiry: an authorization names a deadline. At the deadline, if no token was issued, the leader appends `refused` with reason `expired`. If a token was issued, expiry does not settle or cancel the external effect; it only forbids another token. The action remains `dispatched` or `uncertain` until evidence arrives.

Attempt budget and dispatch budget are recorded counters, not in-memory feelings. Changing a budget is a control record.

Operators see one screen per action: state, class, policy version, inputs, attempts, evidence, and the single next legal action. If the next legal action is “wait” or “attest”, the screen says that. It does not offer a retry button that the state machine would reject.

## 11. Verification strategy

Verification is replay and fault injection, not a proof assistant.

1. **Decision replay.** For every policy version, a corpus of recorded inputs is checked in CI. The authorizer is a pure function. Divergence fails the build.
2. **State-machine enumeration.** Every legal and illegal edge is tested, including attempts to double-sign single-shot actions and to settle without the required evidence.
3. **Gate tests.** A gate without a committed record must not sign. A gate after lease loss must not sign. A gate under halt must not sign. These tests run against the real gate binary with a test log.
4. **Fault injection, in a sandbox with fake external systems.** Drop the response after apply. Apply twice. Ack then roll back. Split the leader after commit but before sign. Kill the leader after sign but before effector return. Jump the clock. Corrupt one replica. Fill the disk. Each scenario has an expected resulting state, usually `uncertain` or a single `settled`, never two effects for idempotent and compare-and-set classes.
5. **Token binding.** Any bit flip in payload, action id, or expiry fails verification at the effector and at a reference checker.
6. **Replay from archive.** Given only the WORM checkpoint and a log prefix, an offline tool reproduces every settlement. This is the audit test, run continuously on a sample and fully on every release.

No verification step is allowed to call a live external production system.

## 12. Qualification strategy

Qualification is gated. A later stage does not start because the calendar moved.

**Stage A — Shadow.** Authorizer and log run on a copy of real intents. Gate does not sign. Compare decisions to the incumbent control, if any, and to manual review of a sample. Exit: replay is deterministic, refuse/admit matches the written policy on the reviewed sample, no unsigned path exists in the binary (tested by the gate tests).

**Stage B — Idempotent, limited blast radius.** One external system with a real idempotency contract. Dispatch budget small. Exit: injected duplicates cause one external effect; halt stops new tokens within the leader’s local flag time (measured, budget under one second for the flag, accepting that in-flight bytes may already have left); kill of a log node does not double-effect; restore drill leaves the partition halted until reconcile.

**Stage C — Compare-and-set.** Same, plus ambiguity drills that leave business references `uncertain` and prove no second token is issued.

**Stage D — Single-shot, if and only if a domain requires it.** Qualification must show the freeze: after an ambiguous attempt, a second intent with the same business reference is refused, and only an operator attestation clears it. If this cannot be shown, that domain does not go live on this runtime.

**Stage E — Load and soak.** Sustain the partition rate with headroom. Failure is not throughput alone. Failure includes budget-ramp behaviour after a dependency outage, anchor lag, and operator time to determine why an action is `uncertain`.

**Stage F — Game days, repeating.** Quorum loss, archive unavailable, bad policy version, credential rotation, leader flap, uncertainty flood. Exit criteria are the invariants in section 5, measured, not a slide.

Production for a domain means Stage B at minimum, the class stages that match its external systems, and a named owner for halt and attestation.

## 13. Security strategy

Threat model: operators err; some credentials leak; an effector host is compromised; an external system lies; a policy author is careless. Not in the threat model: a requirement to hide action contents from the runtime operator. This runtime is an audit and control system. Confidential computing is not a requirement and is not used.

Controls:

- Dispatch signing keys exist only on the active gate, per partition, short-lived, loaded from a conventional HSM or KMS unwrap at leader start. Loss of lease destroys the in-memory key. Log replicas cannot sign.
- Effector credentials cannot write to the log and cannot read the dispatch key.
- Policy versions are signed by a policy key held by the policy duty, distinct from the dispatch key. The authorizer rejects unsigned policy.
- Admin changes are control-partition actions. Break-glass exists as a pre-authorized, time-bounded, loudly anchored control action, not as a shared root password. Break-glass cannot delete log records.
- mTLS and workload identity between components. No long-lived shared bearer tokens between gate and effector: the dispatch token is the authorization, and it is bound to the payload.
- External callbacks are authenticated. Unauthenticated observation is stored as untrusted and cannot settle.
- Direct human credentials to external systems are removed for domains this runtime governs. Otherwise the log is theatre.
- A compromised gate can still emit tokens for already `authorized` actions and can, for single-shot, cause the one allowed effect early or against a stale authorization that has not expired. Mitigation is short authorization expiry, small queues, partition-scoped keys, halt, and detection via anchor and evidence gaps. The design does not claim that a live malicious gate is harmless. It claims the blast radius is the partition’s currently authorized set, and that history cannot be quietly rewritten.

## 14. Distributed systems strategy

- One partition, one consensus group, one active gate. Consensus is Raft or an equivalent well-implemented replicated log. The programme does not invent a protocol.
- Linearizability is required for the log commit that precedes signing. It is not required of the external world and is not promised end-to-end.
- Availability yields to the quorum. A partition that cannot commit cannot dispatch. That is the correct failure.
- Network partitions: a leader that cannot refresh its lease stops signing even if it can still reach the external system. A minority that can still reach the external system must not. Fencing is the lease plus the rule that tokens are only minted against the latest committed attempt counter. There is no second signer to fence against if the key never loads on a non-leader. The dangerous case is a leader that signs, then pauses, then signs again after losing leadership. The lease timeout must exceed the token-issue critical section, and the critical section must be: commit attempt record, sign, only then return the token. A paused leader that missed the lease dies without having released a token it did not commit. A token it did commit is already in the log, so the new leader will not issue another for single-shot.
- Clocks: consensus uses logical time. Wall clocks are inputs recorded in evidence, never a hidden condition inside the authorizer. Authorization expiry is a logical deadline stored in the record, compared by the leader against its committed time, which advances only by log records and a bounded lease clock. Clock jumps are detected by the supervisor and cause halt, not faster expiry or slower expiry at the gate’s whim.
- Messages are at-least-once. The state machine is built for duplicates. Consumers of the log are idempotent on record sequence numbers.
- No cross-partition atomicity. A parent that needs two external effects uses two actions and settles on evidence. Partial completion is an explicit `uncertain` or `contradicted` parent, resolved by a new action, not by rollback of the log.

## 15. Scalability strategy

Millions of high-consequence actions per day is on the order of tens per second sustained and, with peaks, low thousands per second. That is a modest rate for a replicated log if the working set is partitioned. The programme will not spend its complexity budget on global scale-out.

- Partition by domain and external account or business scope, chosen so that two actions that must be ordered fall in the same partition, and unrelated accounts do not.
- Hot partitions are split by a recorded re-shard: freeze new admits, drain or hand off `authorized` work, start a child partition at a new key range. In-flight `uncertain` actions stay with the old partition until they settle. Splitting is rare and manual.
- Effectors scale out. The log and the gate do not. If a gate cannot sign fast enough, partition further. Do not add a second signer.
- Observers are sharded by external system and capped. Observation backlog increases uncertainty deadlines’ queue; it must not increase send rate.
- Payloads are stored by hash. The log carries hashes and small records so consensus stays on the critical path of small entries.
- The intended ceiling is many independent partitions, each deliberately small enough to operate and to halt. Global dashboards sum metrics. They are not a global lock.

If a single business reference truly requires more ordered throughput than one log can commit, the external contract is wrong for per-action consensus, and the remedy is batching inside one authorization (one signed batch, one idempotency key), not a faster consensus invention.

## 16. Production roadmap

The roadmap is the qualification stages, in order, with one external system at a time.

1. Freeze this document. Changes require a written invariant that the current design breaks, not a preference.
2. Build the log, the pure authorizer, the gate, and the state machine against fake external systems. No production credentials.
3. Pass verification in section 11.
4. Qualify Stage A, then B, on one idempotent external system and one domain.
5. Add compare-and-set only for systems whose contract has been written down and tested.
6. Add single-shot only for a domain that has accepted freeze-on-ambiguity in its operating procedure.
7. Add the WORM anchor and the offline replay tool before Stage E. Audit that cannot be replayed is not done.
8. Remove direct operator credentials for each domain as that domain cuts over.
9. Expand partitions by domain. Do not generalize into a platform for actions that are not consequential.
10. Operate. The production work after cutover is policy discipline, game days, and keeping the state machine small.

Staffing shape, not a schedule: a small group that owns the log and gate, a group that owns connectors and observation per external system, and an audit duty that owns the anchor and is not in the on-call path for dispatch. Policy authors sit with the business owners of each domain.

## 17. Research roadmap

Research is out of the production path. Nothing below is required to qualify or to operate.

- Faster replay for very long partitions, only if restore drills miss the recovery objective.
- Better operator summarisation of large `uncertain` sets, if game days show operators mis-classify evidence. Summaries must not gain authority to settle.
- Hardware key custody improvements if, and only if, a measured incident shows in-memory gate keys are the actual loss mode. Ordinary HSM unwrap is the baseline.

If a research item would add a component on the dispatch path, it does not graduate.

## 18. What absolutely does NOT belong

- Models that authorize actions. Policy is code and data that humans review. A model may help an author draft a policy offline. It has no duty in section 3 and no key.
- Blockchains, tokens, zero-knowledge proofs, and trusted execution environments. They do not make an external system’s commit exactly-once, and they add operator concepts the state machine does not need. The independent archive is a locked object store.
- Confidential computing. The operator is inside the trust boundary for seeing actions. Hiding actions from the operator fights audit.
- New consensus protocols, eBPF control paths, and runtime verification products. Raft and tests are the mechanism.
- Formal proofs as a gate to production. Useful as private engineering notes if someone wants them. They are not a dependency and not a stage.
- XA and two-phase commit across external systems.
- Automatic undo, saga engines that compensate without a new authorization, and retry storms.
- A global total order.
- A retry button in the operator UI.
- Exactly-once marketing, internal or external.
- A mesh, service bus, or workflow product as the safety argument. Those may move bytes. They must not be where authorization lives.
- Multi-active dispatch for one partition.
- Direct production credentials held by humans for governed domains.

## 19. Biggest architectural risks

1. **External idempotency is weaker than the contract.** The soundness of automatic retry depends on a promise this runtime cannot enforce. Mitigation: Stage B qualification that actually double-submits, and a willingness to demote a system to single-shot or compare-and-set when it fails that test. Residual risk: a defect that appears only under a rare external failure mode.
2. **Single-shot freeze is operationally unacceptable, so someone bypasses it.** If operators refuse to leave a business reference frozen, they will demand a retry button or a side credential. That destroys the invariant. Mitigation: do not onboard a domain that will not accept the freeze; do not ship the button. Residual risk: organisational override.
3. **A compromised or buggy gate emits the one legal token for the wrong reason.** Short expiry and a small authorized set bound this, but do not eliminate it. Mitigation: gate tests, short deadlines, no hidden inputs, separate keys. Residual risk: logic bugs in the pure authorizer that are faithfully and disastrously deterministic.
4. **Observation is mistaken for truth.** A lying or stale read can settle an action incorrectly. Mitigation: settlement rules that require the right kind of evidence, `contradicted` as a first-class outcome, no settlement from unauthenticated data. Residual risk: a bad settlement rule signed off in policy.
5. **Anchor administrative domain is not actually independent.** If the same people can rewrite the log and the archive, audit after a malicious operator is weaker. Mitigation: separate duty, separate credentials, retention lock. Residual risk: one company, one pressure path.
6. **Halt arrives after the packet.** In-flight attempts still happen. Anyone who treats halt as “no external effect from this moment” will be wrong. The correct statement is “no new token”.

## 20. The three assumptions most likely to be wrong

1. **External systems that claim idempotency keys actually suppress duplicate effects under timeout, failover, and partial apply.** This is the assumption Stage B exists to attack. If it is broadly false, most domains must run as single-shot with freezes, and throughput of ambiguous retries collapses by design. The architecture still stands; the automation people expect does not.
2. **Operators and incident commanders will leave `uncertain` actions unresolved rather than force a resend.** Under financial or safety pressure, the social system routes around a runtime that will not retry. If that happens, the runtime is no longer governing those actions. The assumption is organisational, and it is the one most likely to fail in production even if the software is right.
3. **A separate administrative domain for the WORM anchor will remain separate.** In many companies audit, platform, and operations converge under one break-glass. If that happens, tamper-evidence after an insider with full access is an aspiration. The runtime remains correct for faults and for ordinary mistakes. It becomes weaker against its own administrators.

---

## Hostile review

The review tried to kill the design. Findings and the disposition:

**“Exactly-once across external systems.”** Destroyed, and it was never a goal. End-to-end exactly-once is false. The design claims exactly-once authorization and at-most-once token release for non-idempotent classes. That claim is enforceable inside the log and the gate. It does not extend past the network. The design survives by refusing the stronger claim.

**“Two gates sign.”** The earlier sketch of a standby signer was rejected. Only the leader loads the key, only after the attempt record commits, and lease loss zeroes the key. A duplicated token still depends on a duplicated critical section. The remaining hole is a leader that commits an attempt and then both it and a confused operator copy the token out of band. Tokens are short-lived and bound to payload; the attempt counter still shows one attempt. Accepted residual risk, not a second component.

**“Retry from `uncertain`.”** This edge was deleted. Adding it back would make single-shot unsound. The design survives only if the edge stays deleted.

**“Compensation as undo.”** Deleted as a primitive. A compensating action can itself be consequential and can fail or double. Making it implicit would hide a second effect. Linked new actions are less convenient and are sound.

**“Hidden reads in the authorizer.”** Forbidden. A live balance check at decision time would make replay a lie and would race the external world. Stale recorded evidence can still authorize a bad action. That is visible in the authorization record and is a policy problem, which is the correct layer for it.

**“Halt as consensus.”** Too slow and still too late for in-flight packets. Halt is a local flag. The design admits in-flight effects. Any requirement that halt retracts a packet already sent is physically false and was not adopted.

**“Global consensus for audit order.”** Rejected. Cross-partition order is not an invariant. Audit order is per partition, with anchored hashes. A regulator that demands a single total order of all actions would force a bottleneck and a false coupling. The design refuses that requirement. If a real legal obligation demands it for one domain, that domain is one partition, not a global log of all domains.

**“Malicious gate.”** Not solved. Bounded. A design that claimed otherwise would be unsound. Section 13 states the bound.

**“Parent transaction across two external systems.”** Any redesign that adds 2PC here is rejected on contact. Partial external effects are a business state, represented as `uncertain` or `contradicted`, resolved by a new authorized action.

**“This is just a workflow engine with extra words.”** The difference that matters is constitutional: the sender cannot sign, the signer cannot invent, uncertainty does not retry, and settlement is a pure function of recorded evidence. Remove any one of those and the design becomes a normal orchestrator, which is not safe enough for this problem. Those four are the freeze surface.

### Why it was not redesigned again

Further components (policy DSLs, verification coprocessors, automatic compensation planners, active-active leaders, end-to-end proofs) increase the number of duties that can cause or hide an effect. They do not remove the external-system boundary, which is the actual problem. The design is frozen because every remaining risk is either:

- outside the process boundary and explicitly represented as `uncertain`, `contradicted`, or a freeze, or
- an organisational assumption named in section 20, which software should not paper over.

### Freeze

Freeze the duties in section 3, the states and classes in section 5, the commit-then-sign rule, the ban on retry from `uncertain`, and the ban on a second signer. Implementation may change storage, language, and consensus library. It may not add a state, a signer, or an automatic compensation path without reopening this review.
