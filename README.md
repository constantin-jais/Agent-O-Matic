# Cos-Matic

**Layer:** Bolt — Orchestration  
**Role:** deterministic intent-to-execution brain  
**Mission:** turn high-level operational intent into safe, inspectable plans across agents, tools, and repositories.

---

## Stack Role

- **Maturity:** `usable`, moving toward `trusted` as gates and evidence harden.
- **Current increment:** P4 orchestration integrated.
- **Learning value:** deterministic planning, refusals, safe writes, policy gates, and auditable agentic work.
- **Next quality step:** keep planning/refusal/evidence paths trusted before expanding runtime execution.

See the ecosystem cockpit in [`constantin-jais/ecosystem/status.md`](https://github.com/constantin-jais/constantin-jais/blob/main/ecosystem/status.md).

## Purpose

`cos-matic` is the central orchestrator of the ecosystem. It receives goals, applies policy gates, selects tools, sequences actions, and records decisions.

It transforms:

> what should be done → how it will be executed safely

## Owns

- Agentic orchestration and delegation.
- Config compilation for coding agents and operational harnesses.
- Safe-write, drift detection, gates, incidents, and execution evidence.
- Coordination of Wrench tools and Gear substrates.

## Does Not Own

- Product UX: belongs to Rumble.
- Raw extraction/parsing: belongs to Wrench.
- Persistent memory, artifact storage, registry, or runtime substrate: belongs to Gear.
- Generic chat UI or model hosting.

## Allowed Dependencies

- Calls **Wrench** tools for extraction, inspection, validation, and evidence.
- Reads/writes context through **Gear** primitives.
- Serves **Rumble** products that need orchestration.

## Product Vision Challenge

`cos-matic` must stay a deterministic orchestrator, not become an all-purpose agent product. Its value is trust: explicit plans, reversible writes, gates, and auditable outcomes.

## Daily Use

Install the CLI locally:

```sh
cargo install --path crates/cli
```

Then use `cosmatic` as the session harness:

```sh
cosmatic generate --check --manifest harness.toml
cosmatic goals --manifest harness.toml
```

This repository dogfoods `harness.toml` to generate `AGENTS.md`, `CLAUDE.md`,
and Cursor rules from one source of truth. See
[`docs/codex-routine.md`](docs/codex-routine.md) for the full Codex session
workflow.
