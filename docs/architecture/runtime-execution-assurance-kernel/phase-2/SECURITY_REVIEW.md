# Security review — Phase 2 foundation

**Reviewer role:** REAK Chief Architect (implementation pass).  
**Not a qualification sign-off.**

## Cross-cutting

| Control | Implementation |
|---------|----------------|
| Fail closed | Authority deny, UES overrun, exposure ceiling, registry miss, invalid IDs |
| Immutable records | `StoredRecord` append-only; supersede adds new record |
| Append-only | No update API on `DurableRecordStore` |
| Input validation | Tenant/artifact/epoch ID charset + length; payload max 1 MiB |
| Replay resistance | Hash chain verification API; replay module read-only |
| Concurrency | RwLock; durable record concurrent append test |
| Panic safety | No `unwrap` on user input in library paths; tests may assert |
| Serialization | serde_json with reject on malformed (callers); bounded payload |

## Per module

| Module | Trust boundary | Notes |
|--------|----------------|-------|
| reak-durable-record | Stores opaque bytes | No semantic interpretation |
| reak-policy-context | Epoch registry | Hash mismatch rejects resolve |
| reak-ues | Budget enforcement | Violation appended before error return |
| reak-registry | Index only | No correctness claims |
| reak-replay | Read-only verifier | No dispatch hooks |
| reak-authority | Grant/revoke | Revoked grants fail verify |
| reak-exposure | Reservation vs ceiling | Requires active authority scope |

## Cryptographic boundary

SHA-256 used for **integrity chaining**, not encryption. TLS and at-rest encryption are host/adapter responsibilities (ecosystem principle).

## Residual risks

- In-memory stores are not durable across process crash (documented; Phase 3+ persistence adapter).
- No multi-tenant crypto isolation in Phase 2 (logical tenant id only).

## Blocking issues

None identified for Phase 2 gate proposal.
