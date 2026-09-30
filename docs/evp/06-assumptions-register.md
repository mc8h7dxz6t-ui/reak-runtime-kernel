# Critical assumptions register

Each assumption is one that a consequence-governance programme needs. The mark is only as strong as the external sources. Primary interviews: none.

| Assumption | Mark | Why |
| --- | --- | --- |
| Enterprise software is moving toward agents that execute tools, not only toward more dashboards. | Supported | Microsoft, MCP gateway deployments, and agent framework ecosystems are publishing this as current work, not as a research theme. |
| Runtime authorization before a tool call is missing from the agent protocols. | Supported | Microsoft’s sentence: MCP standardizes execution and does not define governance. AGT is the patch. |
| Nobody has shipped that authorization. | Contradicted | AGT public preview; gateway papers describing production fronting dozens of MCP servers. Identity products already authorize sessions. |
| Sequence-level and “declare intent, then act” governance is widely solved. | Not supported | AGT lists both as not yet available. |
| Independent evidence, from a party who does not run the system, is what customers or regulators ask for. | Not supported | DORA and Article 12 assign retention and reporting to the entity or the provider. GRC products store that entity’s evidence. |
| Retained, automatic operational records matter. | Supported | Article 12 and DORA. Knight’s order shows internal mail that existed and was not used, which is a different failure. |
| Idempotency already solves duplicate external effects. | Partially supported | It solves them when the key is actually sent, stored uniquely, and honoured by the provider inside its retention window. It does not solve the check-then-act race, the forgotten forward, or the provider with no key. |
| Therefore a separate freeze-on-unknown runtime is unnecessary. | Partially supported | AWS and Temporal already tell the user to combine at-most-once with no retry for non-idempotent calls. That is the behaviour, inside the product they already run. A separate product is not required by those docs. |
| Recovery and compensation semantics are unaddressed. | Contradicted as “unaddressed”; supported as “still sharp” | Temporal ships sagas and also documents compensation that never registers, and compensation that fails. The problem is known and only partly contained. |
| Exactly-once is what buyers think they are buying. | Partially supported | Step Functions’ workflow-type page uses exactly-once for Standard workflows and names payments. The SDK page walks that back. The dangerous gap is the distance between the two pages. |
| A halt of new authorizations is what incidents require. | Partially contradicted | Knight’s order requires controls to halt a system that is already emitting orders, plus capital thresholds. Refusing the next signature does not retract orders in flight and did not, on the facts in the order, replace a router kill switch. |
| Human approval of every consequential act is the enterprise pattern. | Not supported | AGT uses policy by default and approval for sensitive tools. Workflow products use human tasks as an exception path. |
| AI agent safety research is the buying centre. | Not supported | The buying artefacts found are gateways, identity, workflow, and compliance tools. Papers and OWASP-style mappings are real. They are not invoices. |
| Formal methods are required to make this true. | Not supported | No production requirement turned up in this pass. Vendors specify policy evaluation and retry rules. |
| The next three to five years create a new budget line for “consequence assurance.” | Not supported | The direction of travel is absorption into the gateway, the workflow engine, and the compliance file. |

Falsification tests that remain open, because this pass could not run them: a buyer interview in payments or market infrastructure; a procurement search for “uncertain external effect” or equivalent; a count of MCP gateways that already store the tool-call decision durably before the socket write. Any one of those could move “no evidence” to supported or to contradicted.
