# Artifact-First Release

A release ships immutable artifacts, not hopes that production rebuilds the same
thing later.

## Doctrine

- Build once per supported target in CI or a controlled builder.
- Attach evidence before exposure: tests, audits, checksums, SBOM, provenance,
  and release manifest.
- Promote pointers to already-built artifacts; do not rebuild during promotion.
- Deployment rollback means repointing to a previous known-good artifact.
- Distribution compensation means yank/deprecate/replace forward; never pretend
  irreversible registries can roll back.

## Required artifact metadata

Each release artifact records:

- package name and version;
- git commit and dirty-state flag;
- target triple or platform identifier;
- build profile and relevant feature flags;
- SHA-256 checksum;
- SBOM location or embedded SBOM marker;
- signature or attestation reference;
- smoke-test evidence;
- rollback or compensation path.

## Build matrix

Rust owns the build matrix. Cross-compilation helpers are implementation details,
not architectural owners. A target is supported only when its build, install,
smoke test, and sovereign floor are documented.

## Gates

A release must fail if:

- an artifact has no checksum;
- provenance is missing;
- license or advisory gates are red without documented exception;
- the deploy path rebuilds instead of deploying a recorded artifact;
- a supported platform has only a store channel and no store-free install path;
- secrets are required where OIDC/keyless publishing is available.
