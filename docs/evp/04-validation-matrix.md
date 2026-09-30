# External validation matrix

The left column is a claim about what a consequence-governance runtime would have to be, stated in external language. The mark is how the sources treat that claim. Internal documents are not evidence.

| Claim | Mark | External basis |
| --- | --- | --- |
| Something must decide, before a tool call, whether that call may run. | Strong support | Microsoft AGT exists specifically because MCP does not do this. MCP gateway deployments report the same fragmentation. |
| That decision should be a deterministic function of a named policy. | Strong support | AGT: Cedar, Rego, or YAML, evaluated before the call, fail closed. |
| Humans cannot approve every call at agent volume; policy plus rare approval is the pattern. | Moderate support | AGT routes approval callbacks only for sensitive tools. No public study in this pass measured volume. |
| End-to-end exactly-once against an external system is available from the orchestrator. | Contradicted | Temporal: at-least-once activities; at-most-once includes zero. AWS SDK: exactly-once end-to-end only with no retry. Stripe: key helps only if sent, and only for 24 hours. |
| If the outcome is unknown, automatic retry is unsafe unless the external system deduplicates. | Strong support | Temporal’s own warning that a completed activity can run again. Payment postmortems of double charges. AWS SDK telling users to use at-most-once plus no-retry for card charges. |
| The industry’s chosen fix is a separate runtime that freezes the action. | Weak support | The published fixes are idempotency keys, unique constraints, and “set max attempts to 1” inside the workflow product the customer already runs. |
| Saga-style automatic undo is a safe primitive. | Contradicted as a complete answer | Temporal documents compensation lost when the activity times out after the side effect, and compensation that itself fails and must be handed to an operator. |
| Append-only or at least retained operational records are required in regulated sectors. | Strong support | DORA incident retention and reporting clocks. EU AI Act Article 12 automatic logs. |
| Those records must be produced by a party who does not operate the system. | Weak support | Article 12 and DORA assign the duty to the provider or the financial entity. GRC tools store the entity’s artifacts. |
| Buyers will pay a new vendor for independent qualification of effects. | No evidence | No pricing, no RFP, no interview in this pass. Adjacent spend is identity, workflow, and compliance automation. |
| A pre-send gate would have stopped Knight Capital’s 45 minutes of orders. | Weak support | The SEC order is about deployment verification, unused code, ignored internal mail, capital limits, and the ability to halt a running router. A gate helps only if every child order is a new decision against a limit. The order does not describe that design. |
| Multi-agent orchestration is the budget centre. | Moderate support | MCP and agent-governance posts are where new writing is. Spending evidence in this pass is product existence, not contract value. |
| Formal methods or runtime verification are how production teams close this. | No evidence in the buyer record | This pass did not find a production buyer requiring a proof assistant. Vendors point at policy evaluation and idempotency. Academic interest is not a cheque. |
