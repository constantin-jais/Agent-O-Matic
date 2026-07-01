# Secrets and credential operations

## Default rule

No secret is required for normal development, CI, docs, dry-run workflows, or
local LM Studio smoke tests. Secrets must never be committed, pasted into issues,
stored in evidence, or printed in logs. Keep local values in ignored `.env` files
or an OS password manager.

## Fact vs precaution

- Fact observed during the split: a legacy pre-Bolt GitHub Actions secret existed
  in the old harness repository and was deleted from GitHub repository settings.
- Precaution: deleting a repository secret does not revoke the backing token. If
  that token still exists in GitHub's PAT settings, revoke it there too.
- No Anthropic key was observed in repository settings during the final checks.
  Mentions of Claude/Anthropic in code and historical docs describe an optional
  fixer seam, not evidence that a key exists.

## Required credentials by mode

| Mode | Secret needed | Where stored | Notes |
| --- | --- | --- | --- |
| local compile / tests | none | nowhere | `cargo test` and `bolt-cosmatic generate --check` are offline |
| local LM Studio smoke | none | nowhere | uses `http://127.0.0.1:1234/v1`; see `docs/local-llm.md` |
| engine dry-run workflow | none | nowhere | uses scoped `github.token` with read permissions |
| harness dry-run workflow | none | nowhere | installs a pinned engine tag and runs `loop --dry-run` |
| harness live with deterministic stub | `BOLT_COSMATIC_BOT_TOKEN` | GitHub Actions secret on the disposable sandbox only | fine-grained PAT, sandbox repo only |
| optional cloud-provider fixer experiment | provider-specific key | only on the disposable sandbox, only for the experiment | not part of the public default path |

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

Rotate only credentials that actually exist or were used. Rotate whenever a
credential-backed live run has been demonstrated publicly, or when a token might
have been exposed by a repository rename/reset/copy.

1. Revoke old PATs in GitHub: **Settings → Developer settings → Personal access tokens → Fine-grained tokens**.
2. Delete stale repo secrets:
   ```sh
   gh secret list --repo <owner>/<repo>
   gh secret delete <NAME> --repo <owner>/<repo>
   ```
3. If live sandbox mode is needed again, create a new fine-grained PAT with the
   minimal scope above.
4. Store it only on the disposable sandbox repo:
   ```sh
   gh secret set BOLT_COSMATIC_BOT_TOKEN --repo <owner>/<sandbox>
   ```
5. For local LM Studio/Gemma tests, do not create a GitHub secret.
6. For a future cloud-provider fixer experiment, rotate and store that provider
   key only if the experiment actually uses it.
7. Confirm there are no stale names:
   ```sh
   gh secret list --repo <owner>/<repo>
   git grep -nE 'AOM_|BOLT_COSMATIC_.*TOKEN|ANTHROPIC_API_KEY'
   ```

## Names that must not be recreated

Legacy `AOM_*` secrets are obsolete. If found, delete them and rotate the backing
credential if it still exists. Do not create compatibility aliases unless a
documented migration requires them.

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
