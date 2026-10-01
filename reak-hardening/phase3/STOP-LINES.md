# Phase 3 stop lines

Hard boundaries for Phase 3 review hardening in the `reak-hardening` programme workspace. These apply for the duration of Phase 3 unless a **new operator charter** explicitly changes them.

## Programme boundaries

| Stop line | Rule |
| --- | --- |
| **Phase 10** | Phase 3 does **not** open, imply, or authorize **Phase 10** or any unnumbered future phase. |
| **Phase 2B** | Phase 2B verdict **CONNECTED** is not revoked by Phase 3 opening; connection/intake evidence remains valid. |
| **Doctrine prompts** | No Phase 3 execution prompt is assumed from the doctrine repo; hardening repo charter is authoritative for lane B. |

## Repository and source

| Stop line | Rule |
| --- | --- |
| **REAK source in hardening repo** | Do **not** copy or vendor the REAK workspace into this repository. |
| **REAK writes** | Do **not** modify canonical REAK kernel source in Phase 3 unless a **separate** operator instruct scopes a patch lane (out of scope for opening charter). |
| **Read-only review** | Examination of REAK uses read-only checkout or CI at the pinned SHA only. |

## Claims and deliverables

| Stop line | Rule |
| --- | --- |
| **Production** | No production deployment or production qualification claim. |
| **Certification** | No certification or regulatory approval claim. |
| **Customers** | No customer deployment claim. |
| **Adapters / providers / cloud** | No adapter, provider, or cloud integration deliverable or readiness claim from Phase 3 review alone. |
| **Qualification / EvidenceLab** | No claim that Qualification Kernel or EvidenceLab programmes have run or passed. |

## Execution

| Stop line | Rule |
| --- | --- |
| **Charter vs execution** | Operator charter **OPENED** ≠ adversarial execution **started**. Execution requires explicit kickoff and evidence. |
| **Architecture** | No redesign of frozen REAK architecture (per programme non-goals). |

## Escalation

Crossing a stop line requires a **written operator instruct** and an updated gate or charter artefact before work proceeds.
