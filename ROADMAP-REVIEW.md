# Roadmap review after accepted programme evidence

Board role: Architecture and Qualification Review Board.
This review accepts the stated programme evidence. It does not re-verify those records and does not amend the constitution, CCS-1, or the EvidenceLab constitution.

The roadmap under review is the qualification order M1–M9: admit claimed passes; Commitment; Dispatch; Observation; Recovery re-run; hostile assembly; CCS-1 report; independent qualification; bounded qualification record.

## 1. What would have changed

FACT. The accepted record qualifies IQ-007, IQ-008, IQ-009 with limitations, and R3 (005→009) with limitations. It freezes the R4 Commitment definition. It states that Commitment implementation has not started, that CCS execution is deferred until a runtime exists, and that no article is impossible.

INFERENCE. If that record had been available when M1–M9 was written, M1 would not have been the first gate. M1 existed only because those passes were testimony. They are now accepted programme records with claim ceilings.

RECOMMENDATION. The implementation roadmap starts at Commitment, using the frozen R4 package. It does not rebuild Truth, Recovery, SafeNext, or the 005→009 assembly.

## 2. Stages that are now lower risk

FACT. Architecture, the constitution, CCS as a contract, the EvidenceLab constitution, and traceability are accepted as frozen. Owners and an implementation path are accepted as allocated. Research is accepted as off the critical path. IQ-007, IQ-008, IQ-009, and R3 are accepted as qualified within their ceilings. R4 is a frozen definition, not code.

INFERENCE. The risk that has been removed is specification and scope uncertainty for the prefix and for the Commitment contract. It is not execution risk for code that has not been written.

RECOMMENDATION. Treat these as lower risk, and do not spend roadmap time reopening them: constitutional possibility; whether CCS must be rewritten; whether Truth, Recovery, or SafeNext must be redesigned; whether 005→009 must be reassembled before Commitment starts; what Commitment is. R4 removes definition risk for Commitment. It does not remove implementation risk.

## 3. Highest remaining risk

FACT. Commitment implementation has not started. CCS has not been executed. The accepted qualifications stop at 009 and are limited. Independent EvidenceLab qualification is not in the accepted qualified set. Dispatch is not in the accepted qualified set.

INFERENCE. The highest technical risk is the first unbuilt obligation that can cause an external effect: implementing Commitment, then Dispatch on it, without a second release from an unknown outcome. The highest qualification risk is citing a limited 005→009 or IQ-008 package as if it covered that new releaser, and deferring CCS until a runtime exists and then skipping the scenarios that belong to Commitment and Dispatch.

RECOMMENDATION. Rank the open risk in this order: Commitment implementation; Dispatch bound to it; the join of that pair to the already qualified chain; CCS execution on that runtime; independent qualification if that claim is made. Do not rank another architecture review.

## 4. Roadmap changes because of this evidence

RECOMMENDATION. Four changes. No new runtime component.

- Remove M1 as an open stage. The accepted records close it. Claim ceilings remain constraints on citation, not a work package.
- Reorder the start. Commitment implementation is the first implementation stage. R4 is its prerequisite. Nothing in the qualified prefix blocks that start.
- Remove Recovery and 005→009 as build stages. IQ-008 and R3 are accepted. Do not repeat them.
- Narrow the later assembly. Replace “qualify 005→009 through Commitment and Dispatch” with one integration qualification: the new Commitment and Dispatch implementation joined to the qualified chain, inside the existing claim ceilings. Recovery scenarios that concern a releaser (restart, site loss, restore, backup replay) are an exit of that integration, not a prior build. They are not a re-opening of IQ-008.

Unchanged, and reinforced: Observation of the new decisions before any “satisfied” claim; full CCS-1 execution after that runtime exists, not before; independent qualification only if claimed, and not on the path that blocks writing Commitment; one bounded qualification record at the end.

## 5. Incorrectly scoped packages

FACT. The programme states that claim ceilings bound every qualification package, and that Commitment implementation has not started.

INFERENCE. IQ-007, IQ-008, IQ-009, and R3 are not shown to be wrongly scoped. They would be wrongly used if a later stage cited them as qualification of Commitment, of Dispatch, or of CCS execution.

RECOMMENDATION. Do not resize those packages. Constrain the next qualification so that it does not inherit their ceilings as a pass of Commitment or Dispatch. IQ-009’s limitations stay on SafeNext. They are not an open IQ and they are not a Commitment defect.

## 6. Easier or harder to implement

FACT. No article was found impossible. Traceability now assigns an implementation path. The Commitment definition is frozen. The prefix behaviour is qualified within ceilings.

INFERENCE. Articles 1 through 4 are not harder. What became easier is knowing that the prefix need not be redesigned and that Commitment has a frozen definition to implement. What did not become easier is writing and qualifying the releaser. Clause 11 is unchanged: the new work must not be described as exactly-once, as proof against a holder of every key, or as a halt that recalls an effect already sent.

RECOMMENDATION. Do not add scope to any article. Implement R4 as frozen.

## 7. CCS mandatory scenarios

FACT. CCS-1 is unchanged. Its execution is accepted as deferred until a runtime exists.

INFERENCE. No mandatory scenario is new, dropped, or retargeted. The scenarios that match Commitment and Dispatch were already the exits of those stages. Deferral confirms they run against the implementation, not as a reason to reorder the constitution or to re-run the qualified prefix as if it were the conformance of the whole programme.

RECOMMENDATION. No CCS-driven roadmap change. Keep the relevant scenarios as the exit of Commitment, Dispatch, and the integration. Keep one full mandatory report after that runtime exists.

## 8. Freeze or modify

RECOMMENDATION. Modify the implementation roadmap. Do not freeze M1–M9 unchanged.

The accumulated evidence reinforces the tail: Dispatch after Commitment, no second release from unknown, CCS after the runtime, independent qualification only for that claim, research off the path, no constitutional amendment. It removes the head: M1, a Recovery build, and a greenfield 005→009 assembly. Leaving those in place would ignore accepted qualifications and would treat limited packages as if they still needed to be earned. Removing the Commitment stage, or calling R3 a pass of Commitment, would ignore the fact that Commitment code has not been started.

Freeze, as the roadmap to implement now:

1. Commitment implementation from the frozen R4 package.
2. Dispatch bound to that commitment.
3. Observation of those decisions, including disagreement without a further release.
4. One integration qualification of that implementation against the qualified 005→009 chain, including releaser restart and restore, without reopening IQ-007, IQ-008, IQ-009, or R3.
5. CCS-1 mandatory execution on that runtime.
6. Independent qualification only if that claim is made.
7. One new bounded qualification record for this join. It does not edit the earlier packages.

Do not freeze a claim that this join is already qualified. Do not freeze research.
