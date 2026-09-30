# Review of the proposed implementation-roadmap freeze

Board role: Architecture and Qualification Review Board.
Normative contracts remain the Programme Constitution, CCS-1, and the EvidenceLab constitution. This review does not amend them.

The submitted freeze is not adopted.

## What stands

FACT. The constitution, CCS-1, and the EvidenceLab constitution stay frozen. No article has been shown to be impossible. Architectural debate stays closed. Commitment implementation is still the first unbuilt gate. CCS execution still waits on a runtime. Research stays off the critical path. It is not abolished, and it is not a v1 implementation item.

## What is rejected

FACT. The accepted programme record already marks IQ-009 qualified with limitations, and R3 (005→009) qualified with limitations. The submitted text calls IQ-009 incomplete and calls 005→009 unvalidated. That is a re-opening, not new evidence. It does not become a blocker.

FACT. CCS-1 does not contain a mandatory set of 25 scenarios. The mandatory count is 43, plus two optional scenarios when a harmless-duplicate claim is made, plus bound checks. An exit of “all 25” cannot certify CCS-1.

FACT. The EvidenceLab constitution says the lab does not operate the specimen, does not authorize effects, and is not a source of ground truth for the specimen. CCS execution uses the scenario’s own instruments and second-party copies. It does not require EvidenceLab to manufacture the proof the suite then consumes. Putting EvidenceLab before CCS makes the lab a dependency of runtime conformance. That contradicts the frozen lab constitution and the clause that a party other than the issuer can recompute from handed-over records.

INFERENCE. “Assembly” in the submitted pipeline means a deployment container. That is an implementation choice. It is not a constitutional stage and it does not have to sit between Reconciliation and EvidenceLab. Reconciliation is not a separate constitutional stage from observation.

RECOMMENDATION. Do not add a stage to finish IQ-009. Do not rebuild 005→009. Do not build EvidenceLab as the feeder for CCS. Do not treat a binary pass/fail certificate, a 100% hash-rejection demo, or “zero state leaking” as the exit. CCS already allows insufficient evidence and indeterminate results, and those are not passes.

## Roadmap that remains frozen

1. Commitment implementation from the frozen R4 package.
2. Dispatch bound to that commitment.
3. Observation of those decisions, including disagreement without a further release.
4. One integration qualification of that implementation against the qualified 005→009 chain, including releaser restart and restore, without reopening IQ-007, IQ-008, IQ-009, or R3.
5. CCS-1 mandatory execution on that runtime.
6. Independent qualification only if that claim is made.
7. One new bounded qualification record for this join.

## Answers to the two execution questions

No. Do not draft new IQ-009 input and output interfaces. That would reopen a package the programme has already qualified with limitations.

No. Do not write a new EvidenceLab signature algorithm. Tamper handling is already frozen: a byte mismatch is a failed check, history is not rewritten, and a qualification bound to the damaged corpus is not currently valid.
