# Phase 5 performance notes

- Append path: one JSON serialize + durable append + O(1) index updates per observation.  
- Per-ticket list grows with observation count (expected; reconciliation reads sets later).  
- `append_serial` mutex bounds write contention; reads use `RwLock` on indexes.  
- No network or provider polling in this phase.
