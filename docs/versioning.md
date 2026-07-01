# Versioning and release typology

`bolt-cos-matic` is pre-1.0. Version numbers communicate maturity and compatibility,
not marketing readiness.

## Stack maturity labels

The ecosystem uses these maturity labels across Bolt, Wrench, Gear, and Rumble:

| Label | Meaning | Compatibility promise |
| --- | --- | --- |
| `experimental` | useful for learning and fixtures; behavior may move quickly | no compatibility promise |
| `usable` | safe for local workflows and bounded CI dogfooding | documented commands should keep working within a minor line |
| `trusted` | suitable for broader automation once audits, provenance, and recovery paths are proven | compatibility changes require migration notes |

Current status: `usable`, not `trusted`.

## SemVer shape before 1.0

Use SemVer syntax, with explicit pre-release channels:

- `v0.<minor>.0-alpha.<n>` — public checkpoints, docs and CI coherent, APIs may change;
- `v0.<minor>.0-beta.<n>` — feature set for the minor is mostly frozen, migration notes required;
- `v0.<minor>.<patch>` — stable-enough 0.x release for routine local use;
- `v1.0.0` — only after the public CLI, safe-write contract, release provenance, and harness evidence are trusted.

Before `v1.0.0`, breaking CLI or manifest changes are allowed only when:

1. the project is still marked below `trusted`;
2. the changelog or release notes name the migration;
3. `bolt-harness` is updated in the same release window.

## Repository roles

- `bolt-cos-matic` is the versioned engine. Release tags live here.
- `bolt-harness` is a public proof bench. It should pin a released engine tag or,
  temporarily, a reviewed engine commit SHA.
- `bolt-harness` does not publish independent runtime versions. Its notable states
  are evidence checkpoints, not product releases.

## Release gates for an alpha tag

An alpha tag may be cut when:

- CI, security, contracts, and coverage are green;
- `cargo test --workspace --all-features` passes locally or in CI;
- the public README quickstart is current;
- `bolt-harness` pins the same tag or reviewed SHA;
- release notes mention known limitations and whether live workflows are sandbox-only.

## Release gates for `trusted`

Do not mark `trusted` until:

- release provenance is implemented or explicitly waived with expiry;
- SBOM/checksums are produced for published artifacts;
- live sandbox credentials are documented and rotated in a tested procedure;
- public evidence proves dry-run and live-stub paths without secrets;
- branch protections and security settings are verified after every repo rename/reset.
