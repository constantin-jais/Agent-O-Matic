# Roadmap

This is a contribution map, not a delivery promise. It keeps the Bolt role clear
inside the broader stack:

- **Rumble** owns product UX and user journeys.
- **Bolt** owns orchestration, plans, gates, and execution envelopes.
- **Wrench** owns reusable inspection/extraction tools.
- **Gear** owns durable substrates, registries, packaging, and runtime primitives.

`bolt-cos-matic` is the Bolt engine. `bolt-harness` is the public proof bench.

## Current maturity

- Label: `usable`.
- Scale-ready: no.
- Public version target: `v0.1.0-alpha.1` after the harness dry-run evidence is published.
- Compatibility: CLI and manifest changes may still break before `v1.0.0`, but every public alpha should include migration notes.

See [`docs/versioning.md`](docs/versioning.md) for the version typology.

## Now — alpha readiness

- publish dry-run evidence from `bolt-harness`;
- keep engine workflows read-only except release-specific jobs;
- keep live sandbox execution only in `bolt-harness`;
- document credential rotation and sandbox secret storage;
- stabilize the README quickstart for a contributor who has never seen the stack;
- keep CI, contracts, security checks, and coverage green.

## Next — `v0.1.x` usable line

- tag `v0.1.0-alpha.1` once public docs and harness evidence are coherent;
- pin `bolt-harness` to that tag instead of a raw commit SHA;
- add example plan/refusal/evidence outputs;
- improve diagnostics and error messages around manifest and handoff failures;
- add contract tests around orchestration boundaries;
- add explicit release notes and changelog entries.

## Later — toward `trusted`

- release provenance, checksums, and SBOM for published artifacts;
- keyless/OIDC publishing where registries support it;
- stronger policy isolation for live automation;
- broader Wrench/Gear integrations only when boundaries remain explicit;
- hosted or multi-agent operation only after audit, recovery, and privacy controls are proven.

## Non-goals for the current line

- product UI;
- model hosting;
- persistent memory or artifact registry;
- production deploy automation from the public harness;
- compatibility aliases for legacy `aom` names.
