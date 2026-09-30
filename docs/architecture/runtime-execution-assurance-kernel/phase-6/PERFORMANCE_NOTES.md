# Phase 6 performance notes

Compare is O(n) in observations for a ticket (linear scan). Emit serialises one record per mismatch. Indexes by ticket for read path.
