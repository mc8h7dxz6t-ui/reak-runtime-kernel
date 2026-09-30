# Capability gap analysis

Recurring unsolved problems, each tied to an external source. “Unsolved” means the sources still describe the failure or the limit. It does not mean no partial product exists.

1. **Policy before one call, not before a sequence.** AGT states sequence-level policy and intent declaration are not available. A buyer who needs “this plan, then these effects” does not get it from the current gateway.

2. **Lost acknowledgement after a real external commit.** Temporal states that a worker can succeed and crash before notifying the server, so the activity runs again. Stripe’s idempotency key returns the first result only if the same key is sent again within 24 hours. A checkout postmortem shows two charges 50–350 ms apart because both requests passed a check before either insert, and the key was not forwarded to Stripe. The gap is the window where the effect happened and the caller does not yet have a durable, unique record the provider will honour.

3. **Idempotency that is only an application check.** The same postmortem and FlowVerify’s write-up both describe check-then-act races. The fix the practitioners publish is a unique constraint plus forwarding the key, not a new governance product. ([FlowVerify](https://www.flowverify.co/blog/idempotency-keys-patterns-race-conditions))

4. **Exactly-once language that the vendor later qualifies.** Step Functions marketing-style docs say Standard workflows execute steps exactly once and are appropriate for payments. The Durable Execution SDK says at-most-once per attempt is not exactly-once across the workflow unless retries are turned off. Buyers can easily buy the first sentence.

5. **Compensation that runs after the side effect and can itself be lost.** Temporal’s saga articles tell the reader to register compensation before the activity, and also describe the failure where the activity did the thing, timed out, and no compensation was registered. Compensation is another fallible call.

6. **Evidence that is the operator’s own log.** DORA and Article 12 require the entity to retain and report. They do not require a second organisation to have seen the effect. Knight Capital’s SEC order records that 97 internal emails identified an error before the open, and nobody treated them as an alert. Evidence existed. Action did not. ([SEC press release](https://www.sec.gov/newsroom/press-releases/2013-222), [SEC order 34-70694](https://www.sec.gov/files/litigation/admin/2013/34-70694.pdf))

7. **A halt that stops new work but does not unwind work already sent.** The SEC’s Knight order faults the absence of controls to halt a system that was already sending orders, and describes staff removing the new code from the healthy servers, which made the fault worse. A gate that only refuses the next signature does not, by itself, satisfy “halt the router.” That is a real requirement the narrow “no new send” rule does not meet.

What is not a gap, on this evidence: single-call allow/deny, SSO in front of tools, workflow history, provider idempotency keys, and audit-log collection. Those are staffed by products shipping now.
