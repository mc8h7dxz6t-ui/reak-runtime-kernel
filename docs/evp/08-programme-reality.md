# Programme reality board (PRB-1.0)

Question: if starting today, with the external record already gathered, would this still be built as a standalone programme?

Sources are the EVP reports and the public documents they cite (Microsoft Agent Governance Toolkit, Temporal, AWS Step Functions and the Durable Execution SDK, Stripe idempotency practice, payment postmortems, the SEC Knight Capital order, DORA, EU AI Act Article 12). This pass did not obtain new pages for Palo Alto, Cerbos, Anthropic, Cloudflare, Google, OpenAI, HashiCorp, or CNCF. Those vendors are marked unverified rather than assigned a roadmap. No RFPs, analyst subscriptions, or buyer interviews were found or conducted.

## Part 1 — Market replacement risk

No vendor in the verified set owns authority, commitment, dispatch, observation, reconciliation, and a next-safe-step rule as one product.

| Vendor | Owns | Explicitly does not own | Roadmap in hand | Five-year direction, as inference |
| --- | --- | --- | --- | --- |
| Microsoft | Policy before each tool call; fail closed; sensitive-tool approval; audit sink. | Sequences of allowed calls; intent declared before action; proof the external system committed. | Sequence policy and intent declaration, stated as not yet available. | Absorb agent-tool governance into the platform that already sits in front of MCP. |
| AWS | Workflow history; a page that says Standard workflows are exactly-once unless Retry is set; a later page that says end-to-end exactly-once needs at-most-once plus no retry. | A promise that a lost acknowledgement was not a commit. | Not established beyond those two documents. | Keep this inside Step Functions and the durable-execution SDK. |
| Temporal | Durable history, at-least-once activities, optional single attempt, sagas that can fail and pause for a person. | Refusal to retry as the default. Idempotency is the customer’s job. | Not established here. | Stay the workflow system of record for people who already run it. |
| Okta, CyberArk, Delinea | Session and privilege. | Whether the later effect happened once. | Not established here. | Stay identity and access. |
| Stripe, as the pattern buyers copy | Deduplicate a charge if the same key arrives within 24 hours. | Callers who never send the key; races before the key is stored; providers with no key. | Not a governance roadmap. | Remain the fix for payments. |
| Palo Alto, Cerbos, Anthropic, Cloudflare, Google, OpenAI, HashiCorp, CNCF, Camunda, LangGraph | Not re-verified in this pass. | Nothing in the verified set shows any of them holding the full chain. | Unknown. Do not invent one. | No basis for a five-year claim. |

Kill test: not met. The stack is split across a gateway, a workflow engine, and a provider key. That is why a sentence about “do not send twice” can still be true. It is not why a new company must own all six names.

## Part 2 — Wedge reality

No budget line, RFP, procurement phrase, analyst note, interview, or pilot was found for consequence assurance, execution assurance, runtime reconciliation, or independent execution evidence.

What buyers already pay for, on the evidence in hand, is IAM, workflow, observability, and compliance-file collection.

Kill test: met in substance. Nobody in this record buys “this layer.” They buy the neighbours. The programme cannot keep a standalone commercial plan that assumes a budget that has not appeared.

## Part 3 — Customer willingness

No deployment study. The substitutes that are actually shipping are not new platforms. AGT is a library and a gateway in front of tools the customer already calls. Temporal and Step Functions are the workflow the customer already chose. Article 12 logs stay in the operator’s systems.

INFERENCE. A buyer who has just adopted an agent gateway and a workflow engine will not volunteer a third runtime, a third approval layer, and an evidence platform. The path consistent with the evidence is a plugin or a policy pack for Temporal, Step Functions, or the Microsoft gateway.

Kill test: not “customers consistently reject,” because they were not asked. The observed behaviour of the vendors is to ship inside an existing platform. That is enough to drop a new-platform assumption.

## Part 4 — Incumbent speed

Microsoft already shipped the authorization wedge and said the next slices (sequences, intent) are on their roadmap. AWS and Temporal already document the retry rule that is the actual technical remainder, inside products with distribution. OpenAI, Anthropic, Google, and Cloudflare were not re-verified; they do not need to have shipped the whole stack for the kill test. One incumbent only has to absorb “if it might have committed, do not send again” into a gateway or a workflow timeout.

Kill test: met for a standalone-platform strategy. It is not met for the idea. The board’s own instruction says this points to integration, licensing, or an acquisition-shaped asset, not to pretending the hyperscaler cannot add a retry flag.

## Part 5 — Technical necessity

Asked externally: what breaks in the market if this stage is not a separate product?

| Stage | If it disappears as a separate build | Material? |
| --- | --- | --- |
| Truth, as a claim about the world | Buyers still have logs and workflow history. The sources never ask for a truth engine. | No |
| Recovery, as its own product | Temporal and Step Functions already resume or compensate, and they document the holes. | No, as a product. The hole remains a setting. |
| SafeNext, as its own engine | The next call is whatever policy and the workflow allow. AGT already does one-call allow or deny. | No |
| Commitment, meaning a durable record before a non-idempotent send | This is the behaviour AWS and Temporal tell the user to configure, and payment incidents show what happens when it is absent. | Yes, as behaviour. No, as a new service. |
| Dispatch, as a separate responsibility | The workflow worker is the thing that sends. A second dispatcher is another place to be wrong. | No |
| Observation and reconciliation, as separate products | Provider keys and workflow history are the tools the postmortems use. A third reconciler is not what those write-ups adopt. | No |
| Replay and an evidence lab | Regulators ask the operator to keep logs. They do not ask for a second laboratory. | No, commercially |

Kill test: most named stages fail it as separate products. One behaviour does not: do not issue a second non-idempotent effect while the first outcome is unknown, and keep the record.

## Part 6 — Complexity audit

The customer-visible value in the external record is that one behaviour, plus the policy check the gateway already sells. Qualification packages, a private evidence platform, and a new vocabulary are cost. They are not present in the buying language.

Kill test: met. Ship the small rule inside an existing workflow or gateway. Do not ship the rest as a platform.

## Part 7 — Economics

No engineering-cost model and no customer ROI figure were available. The published alternative has a low adoption cost: forward an idempotency key, put a unique constraint on it, and set the activity to a single attempt when the provider cannot deduplicate. A new runtime, a new qualification regime, and a new evidence system have to beat that cost. Nothing in this record shows they do.

Kill test: met for the current commercial model. Change the model to a feature in someone else’s platform, or stop spending as if a platform sale were available.

## Part 8 — Assumptions

| Assumption | Support | Contradiction | Confidence | Unknown |
| --- | --- | --- | --- | --- |
| Independent evidence matters to buyers | Article 12 and DORA require records | The duty sits on the operator; GRC tools store the operator’s files | High that the duty exists. Low that a new lab is wanted | Interviews |
| Unknown outcomes matter | Temporal, AWS SDK, payment double-charges | The prescribed fix is idempotency or no retry, not a new product | High | Whether a non-idempotent provider with no key is a large market |
| Customers care, as a budget | None found | Spend is on IAM, workflow, observability, compliance files | Medium, because absence of evidence is not a census | RFPs |
| Runtime reconciliation matters as a product | Disagreement shows up in incidents | It is handled, badly, inside the workflow and the provider | Medium | — |
| Dispatch is a separate responsibility | A send can race a record | The send is the workflow activity | High that a second service is unnecessary | — |

Foundational commercial assumption, “buyers will adopt a standalone consequence platform,” is contradicted by where the working fixes already live, and nothing compensates for that. Revise the programme. Do not revise the narrow technical warning. That warning is supported.

## Part 9 — Adoption reality

A first-ten sequence for a new platform is not credible on this record. There is no named economic buyer and no trigger.

A sequence that is merely possible, and still unproven: one team that already runs Temporal or Step Functions against a provider with no idempotency key, loses money on a duplicate, and accepts a single-attempt policy plus a durable attempt record. Customer 2 is the same pattern in the same industry, not a new category. Customers 5 and 10 do not appear unless that incident is common and the workflow vendor refuses the setting. Both conditions are unknown. If the workflow vendor ships the setting, customers 2 through 10 never leave that vendor.

That is a go-to-market problem for a platform. It is a normal feature request for an incumbent.

## Part 10 — Verdict

**INTEGRATE INSTEAD OF PLATFORM**

The external record still supports one behaviour: when a non-idempotent external call might already have been accepted, do not send it again, and keep a recomputable record of that refusal. It does not support building or selling the surrounding platform. Authority is already a gateway feature. History and retry policy are already workflow features. Evidence duties already sit on the operator. The rational place for the remaining behaviour is inside those products, as a plugin, a policy, or a licensed rule, not as another runtime.

What would most change this conclusion, if it appeared tomorrow: a public procurement, from a named regulated firm, that requires a recomputable record of “no second send while the outcome was unknown,” and states that Step Functions, Temporal, and the cloud’s agent gateway will not satisfy it. That would reopen a narrow product. A Microsoft or AWS release note that ships the same rule would move the verdict from integrate to stop.
