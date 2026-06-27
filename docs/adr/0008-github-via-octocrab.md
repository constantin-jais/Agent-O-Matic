# ADR-0008: GitHub via octocrab (accepting tokio)

## Status

Accepted (2026-06-27).

## Context

A3 adds the incident → issue half of the agentic loop: the orchestrator must
create (idempotently) and search GitHub issues. Two options: shell out to the
`gh` CLI, or use a native Rust client (octocrab).

The compiler (ADR-0002) and the orchestrator so far keep dependencies
deliberately minimal (blake3, clap, serde, toml, miette, thiserror). octocrab
pulls a large async tree (tokio, reqwest, hyper, a TLS stack).

## Decision

Use **octocrab** behind a `Forge` trait, accepting `tokio` and `async-trait`.

Rationale (the project owner's call, eyes open on the trade-off):

- The harness is meant to become an **adoptable open-source library**; a
  consumer should not be required to have the `gh` CLI installed and
  authenticated. A native client is the library-grade path.
- Typed API calls — no parsing of CLI output, no shell-injection surface.

Mitigations for the dependency-discipline cost:

- **Network is confined to `forge.rs`.** No other module makes network calls;
  the compiler stays hermetic.
- **`Forge` is a trait** (`GithubForge` + `FakeForge`), so all logic is tested
  offline; the live client is exercised by a single end-to-end run.
- **Async is confined to the boundary.** The CLI spins a `tokio` runtime and
  `block_on`s the `incident`/`dispatch` subcommands only; the compiler and
  `gate run` stay synchronous.
- **No token in code or logs.** `GithubForge::from_env` reads
  `GITHUB_TOKEN`/`GH_TOKEN` only (locally: `export GITHUB_TOKEN=$(gh auth
token)`; in CI: provided). The orchestrator never holds a token literal.

## Consequences

- Heavier build (octocrab's tree) — accepted as the price of the library path.
- A clear seam (`Forge`) means we could swap to a `gh`-subprocess
  implementation later without touching call sites, if dependency weight ever
  becomes a problem.
