# Commercial opportunity

No interviews were run. No prices were taken from a contract. Where a buyer is named, it is an inference from who already pays for the neighbouring product. That is marked.

## Who writes the cheque

INFERENCE. Three budgets already exist, and they do not sit in one chair.

- Security and platform engineering pay for identity and for anything in front of production tools. The public artifacts are Entra, Okta-class products, privileged access, and now agent gateways. Microsoft is publishing AGT as open source, which lowers the price of “policy before the call” toward zero for teams that can run it.
- Platform engineering pays for Temporal, Step Functions, Camunda, or an internal queue. That budget already includes retry and history.
- Compliance and risk pay for Vanta, Drata, AuditBoard, and for the staff time to answer DORA and, where it applies, Article 12. That budget buys collected evidence, not a new execution path.

A fourth cheque, for a runtime whose job is the unknown-outcome case, is not evidenced.

## How painful is it

FACT. Double charges are painful enough to produce public postmortems and refunds. Knight Capital’s loss was large enough for an SEC order and a $12 million penalty on top of the trading loss. DORA’s clocks are painful because they are legal deadlines.

INFERENCE. Pain is concentrated in payments, market access, and regulated incident reporting. It is not shown to be a general “every enterprise automation” pain. Teams in the payment write-ups fixed it with a unique key and by forwarding that key. They did not describe buying a new platform.

## Current alternatives

Use the provider’s idempotency key and a unique constraint. Set the workflow activity to one attempt when the effect is not safe to repeat. Put Cedar or Rego in a gateway for agent tools. Keep logs for Article 12 and incident files for DORA. Buy a GRC tool if the pain is the questionnaire.

## Buying trigger

FACT. None found for this category.

INFERENCE. A trigger that would fit the evidence is a regulator or a card scheme asking the firm to show, for a named class of instructions, that an uncertain outcome did not produce a second send, and that the record can be recomputed without asking the on-call engineer. That trigger was not found in the documents read. DORA asks for incident reports. Article 12 asks for the system’s own logs. The SEC asked Knight for deployment review and a halt. Those are adjacent triggers, not this product’s name on a purchase order.

## Pilot and return

No external pilot size or ROI figure was found. A honest commercial test is small: one non-idempotent external API, measure duplicate effects before and after, and compare that cost with Stripe-style keys plus a one-attempt workflow. If the keys are available, the published practitioner fix is cheaper than a new runtime. If the external system cannot deduplicate and will not say whether it applied the call, the workflow vendors already tell the customer to stop retrying. The remaining product is the record of that refusal and the qualification that the refusal held. That is a feature of the workflow or the gateway until a buyer says it is not.

## Condition under which a product would be rational

All of the following, and this pass did not confirm them: the external system has no idempotency key; duplicates are expensive; the workflow default of retry is politically hard to turn off; and an auditor will not accept the operator’s log. Missing any one of those, the cheque stays with identity, workflow, or GRC.
