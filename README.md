# Agent-O-Matic

> A deterministic, agent-agnostic **configuration compiler**: one declarative
> source → configuration for many AI coding agents, with **safe-write** and
> **drift detection**.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/Rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/constantin-jais/Agent-O-Matic/actions/workflows/ci.yml/badge.svg)](https://github.com/constantin-jais/Agent-O-Matic/actions/workflows/ci.yml)

**Status: `v0` (Phase 1) — a rigorous design exploration.** Built clean-room as a
learning/teaching artifact: every non-obvious decision is recorded in
[`docs/adr/`](docs/adr/), and the tests are the executable specification. The
goal is depth of understanding, not an adoption race — see
[ADR-0001](docs/adr/0001-positioning-and-why-build.md).

## What it does

You write one source-of-truth: a `harness.toml` manifest that declares reusable
instruction **domains**, **profiles** (named subsets), and **targets** (per-agent
outputs). Domain prose lives in plain Markdown files. `aom generate` compiles
that source into each agent's native config — starting with the universal
[`AGENTS.md`](https://agents.md/), with `CLAUDE.md`, Cursor, and others to come.

```toml
# harness.toml
[package]
name = "my-project"

[[domains]]
name = "code-style"
priority = 8
content_file = "domains/code-style.md"

[[profiles]]
name = "default"
domains = ["code-style"]

[[targets]]
name = "agents-md"
adapter = "universal"
output_file = "AGENTS.md"
profile = "default"
```

## Concepts

- **Domain** — a reusable, priority-ordered block of instructions; its prose
  lives in a plain Markdown file.
- **Profile** — a named subset of domains (e.g. `default`, `backend`).
- **Target** — a per-agent output: which adapter renders which profile to which
  file.
- **Safe-write** — a generated file you hand-edit is never silently clobbered;
  an out-of-band lockfile (`.harness/lock.toml`) records what the tool last
  wrote, and regeneration refuses to overwrite human edits unless `--force`.
- **Drift detection** — regeneration is reproducible; `aom generate --check`
  fails (exit 1) when committed outputs diverge from the source.

## Quick start

```sh
# Build the `aom` binary (lands in target/release/aom)
cargo build --release

# In a project that has a harness.toml (see the example above):
aom generate            # compile the source into each target's native config
aom generate --check    # CI gate: exits non-zero if any output drifted from source
aom generate --force    # overwrite even outputs you hand-edited since the last write
```

`--manifest <path>` (`-m`) points at a manifest other than the default
`./harness.toml`. No build step yet? Run it through Cargo: `cargo run -- generate --check`.

## How it works

One source compiles, deterministically, into many native configs. The lockfile
guards human edits; `--check` makes regeneration verifiable in CI.

```
  harness.toml  +  domains/*.md          (one source of truth)
        │
        │  aom generate
        ▼
  parse → resolve → ir → merge(priority) → render → safe-write → audit
                                              ▲
                              .harness/lock.toml  (guards hand-edited outputs)
        │
        ▼
  AGENTS.md   ·   CLAUDE.md*   ·   .cursor/rules*      (* planned adapters)
```

## What makes it different

This is **not** trying to beat the mature, excellent
[`ai-rulez`](https://github.com/Goldziher/ai-rulez) on its own ground (one
source → ~19 agents, batteries-included). It deliberately goes deep on the two
subsystems that tool leaves implicit:

- **Safe-write.** A generated file you hand-edit is never silently clobbered.
  An out-of-band lockfile (`.harness/lock.toml`) records what the tool last
  wrote; regeneration refuses to overwrite human edits unless you pass `--force`.
- **Drift detection.** Regeneration is reproducible. `aom generate --check`
  fails (exit 1) when committed outputs diverge from the source — a CI gate.

See [ADR-0001](docs/adr/0001-positioning-and-why-build.md) for the honest
positioning: the paradigm is a commodity, so the value here is reconstructing
the safe and deterministic write path from first principles, legibly.

## Build & test

```sh
cargo build
cargo test
cargo clippy --all-targets --all-features   # CI denies warnings (RUSTFLAGS="-D warnings")
```

## Contributing

The reasoning matters as much as the code. Start with the ADRs — they record
every non-obvious decision:

- [ADR-0001](docs/adr/0001-positioning-and-why-build.md) — why build this despite `ai-rulez`
- [ADR-0004](docs/adr/0004-safe-write-sentinel-lockfile.md) — the safe-write design
- [ADR-0005](docs/adr/0005-error-handling-miette.md) — diagnostics-first errors

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the bar: tests as spec, zero-warning
lints, and an ADR for any architectural change.

## License

MIT — see [LICENSE](LICENSE).
