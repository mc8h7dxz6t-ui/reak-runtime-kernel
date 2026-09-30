# Industry direction

Sources are vendor posts, standards, and incident write-ups. No internal programme document is used as evidence. No buyer interviews were conducted.

## Where enterprise software is moving

The live movement is agents calling tools, with a policy check in front of the call, and an audit record after it. Microsoft’s Agent Governance Toolkit states the gap in those words: the Model Context Protocol “standardizes the execution surface without defining how that surface should be governed,” and there is “no built-in point where policy is evaluated before a call is executed.” AGT evaluates each tool call against Cedar, Rego, or YAML before invocation, and fails closed on evaluation errors. It is in public preview. Microsoft also says AGT “governs individual tool calls” and “does not yet correlate sequences of individually-allowed calls,” and that “intent declaration” before action is roadmap, not product. ([Securing MCP](https://developer.microsoft.com/blog/securing-mcp-a-control-plane-for-agent-tool-execution/), [MCP gateway tutorial](https://github.com/microsoft/agent-governance-toolkit/blob/main/docs/tutorials/07-mcp-security-gateway.md))

A separate production report describes enterprises going from zero to dozens of internal MCP servers in a year, each with its own authentication, and a gateway put in front to give one place to authorize callers and record who did what. ([arXiv:2608.10760](https://arxiv.org/pdf/2608.10760))

Beside that, durable workflow engines already own long-running execution. Temporal documents activities as at-least-once, tells users to make them idempotent, and offers at-most-once only by setting maximum attempts to 1, which includes the possibility of zero executions. ([Temporal on idempotency](https://temporal.io/blog/idempotency-and-durable-execution)) AWS Step Functions Standard workflows claim exactly-once workflow execution unless a Retry is configured, and say that model is suited to non-idempotent actions such as payments. Express workflows are at-least-once. ([Choosing a workflow type](https://docs.aws.amazon.com/step-functions/latest/dg/choosing-workflow-type.html)) AWS’s Durable Execution SDK then narrows its own claim: at-most-once is per retry attempt, not per workflow, and exactly-once end-to-end requires at-most-once plus a no-retry strategy. ([Idempotency and retries](https://docs.aws.amazon.com/durable-execution/patterns/best-practices/idempotency/))

Regulation is moving toward retained incident evidence and automatic logs, not toward a new execution kernel. DORA requires financial entities to report major ICT incidents on a clock (initial within 4 hours of classification and no later than 24 hours from awareness; intermediate within 72 hours; final within a month) and to retain incident evidence. ([DORA, Regulation (EU) 2022/2554](https://eur-lex.europa.eu/legal-content/EN/TXT/?qid=1778267070396&uri=CELEX%3A32022R2554), [ESMA operational instructions](https://www.esma.europa.eu/sites/default/files/2026-09/DORA_Incident_reporting_-_Operational_instructions.pdf)) The EU AI Act, Article 12, requires high-risk systems to allow automatic recording of events over the system’s lifetime, for traceability and post-market monitoring. It does not require an independent laboratory. ([Article 12](https://ai-act-service-desk.ec.europa.eu/en/ai-act/article-12))

## Problems everyone is solving

- Who may call a tool, and with which arguments, before the call runs.
- How to retry work that failed in the network without losing the workflow.
- How to make a payment API safe to retry, using the provider’s idempotency key.
- How to show an auditor a log, a control, or an incident report on a deadline.

## Problems nobody solves well

These are the gaps the same sources admit, not a claim that no product exists.

- A sequence of allowed calls that is harmful only as a sequence. Microsoft says this is not in AGT yet.
- What to do when an external effect may have happened and the acknowledgement was lost. Temporal’s default is to retry. Stripe’s key lasts 24 hours and does nothing if the caller never sends it. Practitioner write-ups show a check-then-insert race that double-charges even when the application “had idempotency keys.” ([The Payment API That Charged Customers Twice](https://www.lukretium.com/blog/the-payment-api-that-charged-customers-twice), [Stripe idempotency keys](https://singhajit.com/how-stripe-prevents-double-payment/))
- Proof that is not the operator’s own log. Article 12 and DORA both ask the operator to keep and report records. They do not create a second trust root.

## Direction over the next three to five years

INFERENCE from the sources above, not a forecast model. Agent tool gateways and identity delegation will be absorbed into identity and cloud platforms, because that is where Microsoft, and the MCP gateway papers, are already putting the work. Durable execution will stay with workflow products, which will keep teaching idempotency rather than forbidding retry. Auditors will ask for automatic logs and incident timelines. A separate product whose only job is “do not send again while the outcome is unknown” is a narrow control those platforms can add. It is not where the budget narrative is today.
