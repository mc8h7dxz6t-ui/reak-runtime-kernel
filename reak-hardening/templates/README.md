# Templates

## HARDENING_PIN.json

Copy to the **Qualification Kernel** repository root when that repo exists (optional but recommended).

Single-line JSON. Fields:

| Field | Meaning |
| --- | --- |
| `reak_remote` | Logical remote name (default `reak-runtime`) |
| `reak_revision` | Full commit SHA or annotated tag resolved to SHA |

CI example (conceptual):

```bash
git ls-remote "${REAK_REPO_URL}" "${reak_revision}" || exit 1
git clone --depth 1 "${REAK_REPO_URL}" reak-src
cd reak-src && git checkout "${reak_revision}"
cargo test --locked
```

Hardening campaigns set the same `reak_revision` in `intake/manifest.json` → `repository.commit_sha` for the specimen under review.

Do not commit a pin without verifying the SHA exists on the canonical remote.
