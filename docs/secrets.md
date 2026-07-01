# Secrets and credential operations

## Default rule

No secret is required for normal development, CI, docs, or dry-run workflows.
Secrets must never be committed, pasted into issues, stored in evidence, or printed
in logs. Keep local values in ignored `.env` files or an OS password manager.

## Required credentials by mode

| Mode | Secret needed | Where stored | Notes |
| --- | --- | --- | --- |
| local compile / tests | none | nowhere | `cargo test` and `bolt-cosmatic generate --check` are offline |
| engine dry-run workflow | none | nowhere | uses scoped `github.token` with read permissions |
| harness dry-run workflow | none | nowhere | installs a pinned engine revision and runs `loop --dry-run` |
| harness live with `fixer=stub` | `BOLT_COSMATIC_BOT_TOKEN` | GitHub Actions secret on the disposable sandbox only | fine-grained PAT, sandbox repo only |
| harness live with `fixer=claude` | `BOLT_COSMATIC_BOT_TOKEN`, `ANTHROPIC_API_KEY` | GitHub Actions secrets on the disposable sandbox only | prefer `stub` for public demos |

`BOLT_COSMATIC_CHECKS_TOKEN` is supplied by workflows from `github.token`; do not
create it as a repository secret.

## GitHub PAT scope for live sandbox

Use a fine-grained PAT, repository access limited to the disposable sandbox repo,
with only:

- Contents: read/write;
- Issues: read/write;
- Pull requests: read/write.

Do not grant organization-wide access, Actions admin, Packages, Codespaces, or
unrelated repository access.

## Rotation procedure

Rotate whenever a repository is renamed, reset, made public, copied, or a live
run has been demonstrated publicly.

1. Revoke old PATs in GitHub: **Settings → Developer settings → Personal access tokens → Fine-grained tokens**.
2. Delete stale repo secrets:
   ```sh
   gh secret list --repo <owner>/<repo>
   gh secret delete <NAME> --repo <owner>/<repo>
   ```
3. Create a new fine-grained PAT with the minimal scope above.
4. Store it only on the disposable sandbox repo:
   ```sh
   gh secret set BOLT_COSMATIC_BOT_TOKEN --repo <owner>/<sandbox>
   ```
5. Rotate provider keys used by fixers, for example Anthropic:
   - revoke the old key in the provider dashboard;
   - create a new key;
   - store it only when `fixer=claude` is intentionally tested:
     ```sh
     gh secret set ANTHROPIC_API_KEY --repo <owner>/<sandbox>
     ```
6. Confirm there are no stale names:
   ```sh
   gh secret list --repo <owner>/<repo>
   git grep -nE 'AOM_|BOLT_COSMATIC_.*TOKEN|ANTHROPIC_API_KEY'
   ```

## Names that must not be recreated

Legacy `AOM_*` secrets are obsolete. If found, delete them and rotate the backing
credential. Do not create compatibility aliases unless a documented migration
requires them.

## Evidence hygiene

Evidence may contain run URLs, scrubbed command transcripts, and summaries. It
must not contain:

- tokens, key fragments, or bearer headers;
- private URLs;
- raw production logs;
- personal data;
- unredacted prompts or diffs containing user data.

## Preferred long-term direction

Use OIDC/keyless publishing where available. Static secrets should remain the
exception and should be sandbox-scoped, short-lived, and easy to revoke.
