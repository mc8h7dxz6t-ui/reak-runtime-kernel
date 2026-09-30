# Phase 7 threat model

| ID | Threat | Mitigation |
|----|--------|------------|
| T-TRU-01 | Truth without reconciliation | Requires `reconciliation_id` on input record |
| T-TRU-02 | Truth without admissibility | `all_required_admissible` gate for establishment |
| T-TRU-03 | Upgrade unknown to success | ReconciledUnknown → NonEstablished only |
| T-TRU-04 | Duplicate truth per reconciliation | Index + `DuplicateTruth` |
| T-TRU-05 | Log tampering | Hash-chained TRU stream + `verify` |
