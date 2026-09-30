# Consequential action runtime

Architecture for governing consequential software actions on external systems this runtime does not control.

The frozen design is [ARCHITECTURE.md](ARCHITECTURE.md). It is the programme: philosophy, duties, state machine, execution and evidence, recovery, qualification, and the hostile review that justified the freeze.

This repository is a design freeze, not a runtime yet. Implementation starts from the duties and invariants in that document.

[EVIDENCELAB.md](EVIDENCELAB.md) is a separate constitutional specification for an independent evidence and qualification lab. It does not extend the runtime design.

[CONSTITUTION.md](CONSTITUTION.md) is the review-board contract that survives both designs. Implementations are candidates. The contract is the obligation.

[CCS.md](CCS.md) is the conformance suite for that contract. It defines how clauses are falsified. It does not add obligations.

[ROADMAP-QUALIFICATION.md](ROADMAP-QUALIFICATION.md) is the qualification board's decision on whether implementation planning replaces further architectural debate.

[TRACEABILITY.md](TRACEABILITY.md) maps each frozen constitutional article to CCS scenarios, evidence, and ownership. It does not add articles or components.

[IQ-009.md](IQ-009.md) is the qualification verdict for IQ-009.

[ROADMAP-REVIEW.md](ROADMAP-REVIEW.md) records whether later accepted evidence changes the implementation roadmap. It does not change the constitution.

[ROADMAP-FREEZE-REVIEW.md](ROADMAP-FREEZE-REVIEW.md) rejects a later freeze proposal that reopened qualified packages and put EvidenceLab in front of CCS.

[External validation](docs/evp/README.md) tests the same problem against vendor docs, incidents, and regulation. It does not use internal architecture as evidence. The programme-reality verdict is to integrate the remaining behaviour into existing platforms rather than build another one.

[REAK hardening](reak-hardening/README.md) is the adversarial quality programme for the Runtime Execution Assurance Kernel. It does not redesign architecture.
