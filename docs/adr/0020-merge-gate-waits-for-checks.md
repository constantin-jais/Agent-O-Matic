# ADR-0020 — The merge gate waits for checks to settle

## Status

Accepted (2026-06-28).

## Context

The CI-bot runbook (ADR: operate-loop-as-scoped-ci-bot) surfaced that the merge
gate read a freshly-published PR exactly once. Right after publish the CI is still
pending -> Unknown -> the loop stops at automerge every time. Fail-closed, but the
loop could never complete in a single run.

## Decision

`ForgeGate` polls GitHub check-runs through the `Forge` seam (octocrab in the
real implementation; every `interval`, default 10s) until the checks settle,
bounded by a `timeout` (default 3 min), then returns the verdict:

- all pass (or skipped) -> Green; any fail/cancel -> Red; no open PR -> Unknown
  immediately; no checks yet / still pending -> keep waiting until the deadline,
  then Unknown (fail-closed — the wait is bounded, never indefinite).
- The classification (which forge buckets mean pass/fail/wait) is extracted into
  a pure `classify` and unit-tested; only the forge call and sleep loop are the
  live boundary.

## Consequences

- The loop can complete in one run: publish -> the gate waits out the bot's CI ->
  merge on green. The CI-bot path (ADR-0019) is now end-to-end, not wedged at
  automerge.
- The wait is bounded by the timeout, so a hung or never-arriving check fails
  closed rather than blocking forever. `skipping` checks no longer wedge the gate
  (a latent bug — they had been treated as pending).
