# QK2 pre-promotion identity repair (2026-10-03)

**Terminal:** `QK2-PRE-PROMOTION-IDENTITY-REPAIR`  
**Mode:** correction-only (no catalogue row edits, no QK reruns)

## Cause

**`STALE_CURRENT_STATE_REFERENCE`** — `promotion-record.json` and the T01 promotion README treated the frozen T01 evidence snapshot digest (`e532f7b5…`) as the live authoritative current-state matrix after `fef20be` amended promotion metadata in the on-disk matrix (`e3287c54…`).

## Semantic impact

None on catalogue semantics. All 16 scenario rows are identical between `e532…` (historical snapshot) and `e328…` (authoritative current-state). Rollup remains **6 PASS / 10 NOT RUN / 0 FAIL / 16 TOTAL**.

## Dual identity pins

| Role | SHA-256 |
| --- | --- |
| Authoritative current-state | `e3287c54630f92426e739365f3ad646c9ddcde6fa04e16b64c451ed6f180fc52` |
| Historical T01 evidence snapshot | `e532f7b5d0961dcb81b21391baabc326a9e43d571ffc01cd9825d803f3942f54` |

Machine record: [identity-repair-record.json](./identity-repair-record.json)
