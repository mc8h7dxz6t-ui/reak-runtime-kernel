# REAK Runtime Kernel v1.0.0 — golden clone programme

**Status:** **FROZEN** (post Phase 8–9 acceptance)  
**Tag:** `v1.0.0` on the canonical commit that completes the reasoning spine.

## What v1.0.0 is

The first **complete** Runtime Execution Assurance Kernel:

- Foundation substrate + seven engines (Commitment through Progression)
- No adapters, no cloud SDKs, no MCP/Temporal/HTTP integrations in-tree
- Append-only durable streams per phase; dispatch-only execution

This tag is the **golden master**. All product and research lines start here.

## Three repositories (clone now, diverge later)

| Repository | Purpose | May add features? | May change kernel APIs? |
|------------|---------|-------------------|-------------------------|
| **reak-runtime-kernel** | Linux-kernel analogue: security, correctness, qualification-backed bug fixes only | **No** (fixes only) | Only with governance + semver |
| **reak-platform** | Commercial integrations (AWS, Azure, GCP, K8s, MCP, Temporal, Step Functions, Stripe, HTTP, Kafka, SQL, adapters) | **Yes** | Via dependency on pinned kernel crate/tag |
| **reak-research** | Experiments (adaptive progression, probabilistic policies, AI-assisted reasoning, alt recovery/observation) | **Yes** | Proposals only until promoted |

### Clone procedure (when remote exists)

```bash
# From golden tag on canonical REAK repo
git clone <canonical-reak-url> reak-runtime-kernel
cd reak-runtime-kernel && git checkout v1.0.0

git clone <canonical-reak-url> reak-platform
cd reak-platform && git checkout v1.0.0
# rename remote origin → platform; add kernel as path dependency or git submodule at v1.0.0 pin

git clone <canonical-reak-url> reak-research
cd reak-research && git checkout v1.0.0
```

**Rule:** Nothing reaches **reak-runtime-kernel** until proven in qualification and accepted through governance. Platform and research **must not** fork kernel semantics silently — pin `v1.0.0` (or later signed tags) explicitly.

## What stops now

- No new `reak-*` runtime **phase** modules in the kernel repo
- No adapters, providers, or cloud integrations in the kernel repo
- No integration-host implementation until platform programme opens (separate repo)

## Workspace version

Cargo workspace package version is **1.0.0** aligned with git tag `v1.0.0`.

## Evidence

- [ARCHITECTURAL_FREEZE_REVIEW.md](./ARCHITECTURAL_FREEZE_REVIEW.md)
- [REAK_KERNEL_V1_FREEZE.json](./REAK_KERNEL_V1_FREEZE.json)
- Phase gates 1–9: **ACCEPTED / FROZEN**
