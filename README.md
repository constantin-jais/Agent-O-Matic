# Cos-Matic

**Layer:** Bolt — Orchestration  
**Role:** deterministic intent-to-execution brain  
**Mission:** turn high-level operational intent into safe, inspectable plans across agents, tools, and repositories.

---

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
