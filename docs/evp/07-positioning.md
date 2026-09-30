# Positioning and messaging

Written only after the other tracks. The name is whatever an analyst would file this under, given the sources, not an internal title.

## What an analyst would call it

They would not coin a new category from the evidence in this pass. They would file the pieces they can already see:

- **Runtime authorization** or **agent tool governance**, for a policy decision before a tool call. That is Microsoft’s phrase: “a runtime governance layer for MCP tool execution.”
- **Durable execution**, for remembering steps and retrying them. That is Temporal’s and AWS’s category.
- **Operational resilience evidence**, for retained incident records and automatic logs. That is DORA and Article 12 language.

“Consequence assurance,” “outcome assurance,” and “execution assurance” do not appear in these sources as a market name. Using them would ask the buyer to learn a category the adjacent vendors are not using. That is a cost, and this pass found no buyer who is searching for it.

## How to describe the remainder, in market language

If a product still has a job after AGT, Temporal, and a provider idempotency key, the sentence that matches the evidence is narrow:

A control that, once a non-idempotent external call might have been accepted, does not issue another call for that same instruction, and keeps a record of that refusal which someone outside the on-call rotation can recompute.

That sentence is a constraint on a workflow or a gateway. It is not, on this evidence, a platform.

## What not to say

- Do not say exactly-once. Step Functions says it and the Durable Execution SDK qualifies it. Temporal refuses it for activities.
- Do not say the product replaces identity, policy, workflow, or the auditor’s evidence locker. Those jobs have owners.
- Do not say independent evidence is a regulatory requirement. The requirements found are duties on the operator.
- Do not say a pre-send halt would have stopped Knight Capital. The order describes a running router, unused code, and a missing kill control.

## Messaging that survives falsification

“Policy decides the call. The workflow remembers the attempt. If the provider cannot make a duplicate harmless, do not send twice. Keep the record.”

Anything broader was not supported, or was contradicted, in tracks A–I.
