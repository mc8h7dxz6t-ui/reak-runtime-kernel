# Recovery lifecycle

1. **Proposed** — `RecoveryHistoryEntry::Proposed` from immutable truth input.  
2. **Intent authorized** — optional `IntentAuthorized` append; does not enable dispatch.  

No transitions rewrite truth or dispatch effects.
