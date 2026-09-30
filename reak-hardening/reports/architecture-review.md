# Architecture review (structural, not redesign)

Frozen architecture is accepted. This review only asks whether implementation coupling and interface size can shrink **without** changing responsibilities.

## Status

**NOT RUN** on code. No import graph, no crate/module boundaries in this repository.

## Questions to answer when intake completes

1. Does any module both parse untrusted input and hold signing keys?
2. Does any public API expose more than one constitutional duty?
3. Are there circular dependencies across commitment, dispatch, and observation paths?
4. Can verification run without linking the network stack?

## Findings

None. Evidence absent.

## Simplification candidates

Deferred until module manifest exists.
