# A1 — Goals & Gates — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:executing-plans (inline) or subagent-driven-development. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Give the orchestrator a declarative goals/gates primitive: parse a `goals.toml`, render a progress report, and run a blocking "gate-wall" (`fmt` + `clippy` + `tests`) that exits non-zero on any red hard gate.

**Architecture:** Pure schema + evaluation in `crates/orchestrator` (no shelling out), behind a `CheckRunner` trait so the gate engine is fully unit-testable with a fake runner; a real `CargoRunner` shells out to `cargo`. Two CLI subcommands in `crates/cli`: `aom goals report` (static view) and `aom gate run` (live gate-wall). Dogfooded by a root `goals.toml`.

**Tech Stack:** Rust 2024, serde + toml + serde_json (already in `[workspace.dependencies]`), `std::process::Command`. **No new external dependencies.**

## Global Constraints

- Edition `2024`, `rust-version 1.95`, license `MIT` (inherited from `[workspace.package]`).
- Zero clippy warnings under `-D warnings`; `cargo fmt` clean.
- **No new external crates** — only workspace deps (`serde`, `serde_json`, `toml`).
- Determinism + zero-PII (reports carry no usernames/absolute paths).
- The gate engine must be testable WITHOUT running `cargo` (no test may shell out to `cargo test` — that recurses). Use the `CheckRunner` trait + a fake.
- Builds on A0 (`feat/a0-workspace`): `crates/{aom,cli,orchestrator}` workspace.

## File Structure

```
crates/orchestrator/Cargo.toml          # MODIFY: add serde, serde_json, toml
crates/orchestrator/src/lib.rs           # MODIFY: declare goals + gate modules, drop CRATE_NAME stub
crates/orchestrator/src/goals.rs         # CREATE: schema, parse, Op, Status, Metrics, evaluate, status, render_markdown
crates/orchestrator/src/gate.rs          # CREATE: CheckRunner trait, CargoRunner, collect_metrics, GateReport, run
crates/cli/Cargo.toml                    # MODIFY: add `orchestrator` dependency
crates/cli/src/cli.rs                    # MODIFY: add Goals{Report} + Gate{Run} subcommands
crates/cli/src/main.rs                   # MODIFY: dispatch the new subcommands
goals.toml                               # CREATE: dogfood — the harness measures itself
docs/adr/0007-gate-evaluation-model.md   # CREATE: why declarative gates + CheckRunner DI + boolean metrics
```

## Public interfaces (the contract)

```rust
// orchestrator::goals
pub enum Op { Eq, Ne, Lt, Lte, Gt, Gte }          // serde rename: "eq","ne","lt","lte","gt","gte"
pub struct Phase { pub id: String, pub title: String }
pub struct Milestone { pub id: String, pub title: String, pub done: bool }
pub struct Gate { pub name: String, pub metric: String, pub op: Op, pub threshold: f64 }
pub struct Observe { pub name: String, pub metric: String, pub op: Op, pub threshold: f64 }
pub struct Goals {
    pub phase: Phase,
    #[serde(default)] pub milestone: Vec<Milestone>,
    #[serde(default)] pub gate: Vec<Gate>,
    #[serde(default)] pub observe: Vec<Observe>,
}
pub type Metrics = std::collections::BTreeMap<String, f64>;
pub enum Status { Green, Red, Pending }

pub fn parse(src: &str) -> Result<Goals, GoalsError>;     // toml::from_str
pub fn evaluate(op: Op, actual: f64, threshold: f64) -> bool;
pub fn status(metric: &str, op: Op, threshold: f64, metrics: &Metrics) -> Status; // Pending if absent
pub fn render_markdown(goals: &Goals, metrics: &Metrics) -> String;

// orchestrator::gate
pub struct CheckOutcome { pub passed: bool, pub detail: String }
pub trait CheckRunner { fn run_check(&self, check: &str) -> CheckOutcome; }  // check ∈ {"fmt","clippy","tests"}
pub struct CargoRunner { pub repo_root: std::path::PathBuf }                  // real: std::process::Command
pub fn metric_for(check: &str) -> &'static str;            // "fmt"→"fmt_violations", etc.
pub fn collect_metrics(runner: &dyn CheckRunner) -> Metrics;
pub struct GateReport { pub rows: Vec<(String, Status)>, pub all_green: bool, pub markdown: String }
pub fn run(goals: &Goals, runner: &dyn CheckRunner) -> GateReport;
```

`goals.toml` shape:

```toml
[phase]
id = "A1"
title = "Goals & gates"

[[milestone]]
id = "gate-wall"
title = "aom gate run enforces fmt+clippy+tests"
done = true

[[gate]]
name = "fmt"
metric = "fmt_violations"
op = "eq"
threshold = 0

[[gate]]
name = "clippy"
metric = "clippy_violations"
op = "eq"
threshold = 0

[[gate]]
name = "tests"
metric = "tests_failed"
op = "eq"
threshold = 0

[[observe]]
name = "coverage"
metric = "coverage_pct"
op = "gte"
threshold = 80
```

---

### Task 1: goals schema + parse

**Files:** Modify `crates/orchestrator/Cargo.toml` (deps), replace `crates/orchestrator/src/lib.rs`, Create `crates/orchestrator/src/goals.rs`.
**Interfaces produced:** `Op`, `Phase`, `Milestone`, `Gate`, `Observe`, `Goals`, `GoalsError`, `parse`.

- [ ] Add to `crates/orchestrator/Cargo.toml`: `serde = { workspace = true }`, `serde_json.workspace = true`, `toml.workspace = true`.
- [ ] Replace `lib.rs` body with `pub mod goals; pub mod gate;` (drop the A0 `CRATE_NAME` stub).
- [ ] Write failing test in `goals.rs`: `parse` a sample with phase + 1 gate + 1 observe → assert fields (`goals.gate[0].op == Op::Eq`, `threshold == 0.0`).
- [ ] Implement the serde structs (`Op` with `#[serde(rename_all = "lowercase")]`) + `parse` (`toml::from_str`, map error to `GoalsError`).
- [ ] `cargo test -p orchestrator` green; commit `feat(orchestrator): goals.toml schema + parse`.

### Task 2: evaluate + status (pure)

**Files:** Modify `goals.rs`.
**Interfaces produced:** `Metrics`, `Status`, `evaluate`, `status`.

- [ ] Failing tests: `evaluate(Op::Lte, 3.0, 5.0)==true`, `evaluate(Op::Eq, 1.0, 0.0)==false`, etc. for all 6 ops; `status("x", Op::Eq, 0.0, &empty) == Pending`; `status` Green/Red with a populated `Metrics`.
- [ ] Implement `evaluate` (match on `Op`) and `status` (Pending if metric absent, else Green/Red via `evaluate`).
- [ ] `cargo test -p orchestrator` green; commit `feat(orchestrator): gate evaluation (pure)`.

### Task 3: render_markdown

**Files:** Modify `goals.rs`.
**Interfaces produced:** `render_markdown`.

- [ ] Failing test: render a Goals with metrics → output contains the phase title, a `## Hard gates` section, each gate name with its status glyph (`✅`/`🔴`/`⏳`), and a `## Observability` section.
- [ ] Implement `render_markdown` (phase header, milestones list with `[x]`/`[ ]`, hard-gates table, observability table). Status glyph helper.
- [ ] `cargo test -p orchestrator` green; commit `feat(orchestrator): goals markdown report`.

### Task 4: gate engine (CheckRunner + run)

**Files:** Create `crates/orchestrator/src/gate.rs`.
**Interfaces produced:** `CheckOutcome`, `CheckRunner`, `metric_for`, `collect_metrics`, `GateReport`, `run`.

- [ ] Failing tests with a `FakeRunner` (in test module): all checks pass → `run(&goals, &fake).all_green == true`, every hard-gate row Green; one check fails → `all_green == false`, that row Red.
- [ ] Implement `CheckRunner`/`CheckOutcome`, `metric_for` (`fmt`→`fmt_violations`, `clippy`→`clippy_violations`, `tests`→`tests_failed`), `collect_metrics` (run the 3 checks, value `0.0` if passed else `1.0`), `run` (collect → `status` per hard gate → assemble `GateReport` with `render_markdown`-style table + `all_green`).
- [ ] `cargo test -p orchestrator` green; commit `feat(orchestrator): gate engine with CheckRunner DI`.

### Task 5: CargoRunner + CLI + goals.toml + ADR-0007

**Files:** Modify `gate.rs` (CargoRunner), `crates/cli/Cargo.toml` (+orchestrator dep), `crates/cli/src/cli.rs` + `main.rs` (subcommands), Create `goals.toml`, `docs/adr/0007-gate-evaluation-model.md`.
**Interfaces produced:** `CargoRunner`; CLI `aom goals report --config`, `aom gate run --config`.

- [ ] Implement `CargoRunner` (`std::process::Command` in `repo_root`: `fmt`→`cargo fmt --all --check`, `clippy`→`cargo clippy --workspace --all-targets -- -D warnings`, `tests`→`cargo test --workspace`; `passed = status.success()`).
- [ ] Add `orchestrator` dep to `crates/cli/Cargo.toml`.
- [ ] Add `Goals { Report { config } }` and `Gate { Run { config } }` to `cli.rs`; dispatch in `main.rs`: report → `parse` + `render_markdown(&goals, &empty_metrics)` printed; gate run → `parse` + `gate::run(&goals, &CargoRunner{repo_root: "."})`, print `report.markdown`, `std::process::exit(if all_green {0} else {1})`.
- [ ] Create root `goals.toml` (dogfood, shape above).
- [ ] Write ADR-0007 (declarative gates; CheckRunner DI to avoid `cargo test` recursion; boolean `*_violations` metrics for A1; drift/coverage deferred).
- [ ] Verify on the real repo:
  - `cargo run -p aom-cli -- goals report --config goals.toml` prints the phase/gates/observability.
  - `cargo run -p aom-cli -- gate run --config goals.toml` runs fmt+clippy+tests, prints green table, exits 0.
- [ ] `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` green; commit `feat(cli): aom goals report + gate run; dogfood goals.toml; ADR-0007`.

---

## Verification (end-to-end)

1. `cargo test --workspace` green (new orchestrator unit tests + existing 46).
2. `cargo clippy --workspace --all-targets -- -D warnings` clean; `cargo fmt --all --check` clean.
3. `aom gate run --config goals.toml` exits 0 on the green repo; flip a file to a bad format → exits 1 with the `fmt` row red (manual spot-check, then revert).
4. `aom goals report --config goals.toml` renders phase A1, the three hard gates, and the coverage observability row (Pending).

## Self-Review

- **No new deps:** only serde/serde_json/toml (workspace) + std. ✓
- **No recursion:** the gate engine is tested via `FakeRunner`; `CargoRunner` is exercised only by the manual CLI verification, never inside `cargo test`. ✓
- **Type consistency:** `metric_for` strings (`fmt_violations`/`clippy_violations`/`tests_failed`) match the `goals.toml` gate `metric` fields and `collect_metrics` keys. ✓
- **Scope:** drift gate (needs the compiler) and coverage metric (needs tarpaulin) are deferred and documented in ADR-0007 — not gold-plated into A1. ✓

## Execution Handoff

Inline execution on worktree `harness-a1` (branch `feat/a1-goals-gates`, stacked on `feat/a0-workspace`). Finish with a stacked PR (base `feat/a0-workspace`); rebase onto `main` once A0 merges.
