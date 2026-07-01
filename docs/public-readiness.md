# Public readiness checklist

Use this checklist before announcing a repository rename, reset, release, or live
demonstration.

## Repository identity

- [ ] GitHub repository name and description match the README.
- [ ] README names the stack role and boundaries.
- [ ] No stale `aom`, `AOM_*`, `cos-matic`, or old sandbox references remain outside historical ADR context.
- [ ] Public refs are intentional:
  ```sh
  git ls-remote https://github.com/constantin-jais/bolt-cos-matic.git 'refs/heads/*' 'refs/tags/*' 'refs/pull/*'
  git ls-remote https://github.com/constantin-jais/bolt-harness.git 'refs/heads/*' 'refs/tags/*' 'refs/pull/*'
  ```

## Security settings

- [ ] Branch protection requires CI and review on engine `main`.
- [ ] Harness `main` requires hygiene and review.
- [ ] Secret scanning and push protection are enabled.
- [ ] Dependabot security updates are enabled.
- [ ] No repository secret is configured unless a live sandbox needs it.
- [ ] Any legacy `AOM_*` secret has been deleted and the backing credential revoked.

## Workflows

- [ ] Engine workflows are read-only unless a release workflow explicitly needs more.
- [ ] Live sandbox workflow exists only in `bolt-harness` and is `workflow_dispatch` only.
- [ ] Live mode is fenced by `BOLT_HARNESS_SANDBOX=true`.
- [ ] Actions in repository-owned workflows are pinned to commit SHAs.
- [ ] Reusable templates document whether they are pinned examples or user-facing templates.

## Evidence

- [ ] `bolt-harness/evidence/` contains a scrubbed dry-run or hygiene proof.
- [ ] Evidence has no secrets, usernames beyond public GitHub handles, private URLs, or personal data.
- [ ] Claims in README are backed by CI runs, fixtures, commands, ADRs, or evidence files.

## Release/readiness

- [ ] Versioning typology is documented in `docs/versioning.md`.
- [ ] Current roadmap names what is in/out for the next alpha.
- [ ] Harness pins either a released engine tag or a reviewed engine commit SHA.
- [ ] Known limitations are documented before announcement.
