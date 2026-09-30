# Inventories

Generated only after Phase 2B build and test verification succeeds.

Current status: **empty** — connection BLOCKED (`intake/connection-report.md`).

When unblocked, add:

- `modules.md` — from `manifest.json`
- `dependencies.md` — from lockfile / `cargo tree` / equivalent
- `public-api.md` — from `cargo doc --no-deps` or published API list
- `trust-boundaries.md` — from manifest trust_boundary fields plus review notes in Phase 3
