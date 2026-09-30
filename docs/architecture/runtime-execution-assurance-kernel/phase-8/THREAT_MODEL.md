# Phase 8 threat model

| Threat | Mitigation |
|--------|------------|
| Recovery without truth | Requires full `TruthRecord` |
| Dispatch via recovery | `authorizes_execution` hard-coded false |
| Truth tampering | Truth is input snapshot only; no write API |
| Replay of proposal | One recovery per `truth_id` + input digest index |
| Log tampering | Hash-chained RCV stream |
