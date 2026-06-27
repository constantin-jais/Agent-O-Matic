//! `goals.toml` — declarative phase, milestones, hard gates and observability.
//!
//! Thresholds are integers (sufficient for A1: violation counts and a coverage
//! percentage); runtime metric *values* are `f64` so a metric like coverage can
//! be fractional. Comparisons happen in `f64` (see [`evaluate`]).

use serde::Deserialize;

/// Comparison operator for a gate or observability metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Op {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
}

/// The phase the project is currently in.
#[derive(Debug, Clone, Deserialize)]
pub struct Phase {
    pub id: String,
    pub title: String,
}

/// A sub-milestone within a phase.
#[derive(Debug, Clone, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub done: bool,
}

/// A hard, blocking gate: `metric op threshold` must hold for the phase to pass.
#[derive(Debug, Clone, Deserialize)]
pub struct Gate {
    pub name: String,
    pub metric: String,
    pub op: Op,
    pub threshold: i64,
}

/// A non-blocking observability target (reported, never blocks).
#[derive(Debug, Clone, Deserialize)]
pub struct Observe {
    pub name: String,
    pub metric: String,
    pub op: Op,
    pub threshold: i64,
}

/// The whole `goals.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct Goals {
    pub phase: Phase,
    #[serde(default)]
    pub milestone: Vec<Milestone>,
    #[serde(default)]
    pub gate: Vec<Gate>,
    #[serde(default)]
    pub observe: Vec<Observe>,
}

/// Failure to parse a `goals.toml`.
#[derive(Debug)]
pub struct GoalsError(String);

impl std::fmt::Display for GoalsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid goals.toml: {}", self.0)
    }
}

impl std::error::Error for GoalsError {}

/// Parse a `goals.toml` source string into a [`Goals`].
pub fn parse(src: &str) -> Result<Goals, GoalsError> {
    toml::from_str(src).map_err(|e| GoalsError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_phase_gate_and_observe() {
        let src = r#"
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

[[observe]]
name = "coverage"
metric = "coverage_pct"
op = "gte"
threshold = 80
"#;
        let goals = parse(src).expect("valid goals.toml");
        assert_eq!(goals.phase.id, "A1");
        assert_eq!(goals.phase.title, "Goals & gates");
        assert_eq!(goals.milestone.len(), 1);
        assert!(goals.milestone[0].done);
        assert_eq!(goals.gate.len(), 1);
        assert_eq!(goals.gate[0].name, "fmt");
        assert_eq!(goals.gate[0].metric, "fmt_violations");
        assert_eq!(goals.gate[0].op, Op::Eq);
        assert_eq!(goals.gate[0].threshold, 0);
        assert_eq!(goals.observe[0].op, Op::Gte);
        assert_eq!(goals.observe[0].threshold, 80);
    }

    #[test]
    fn rejects_invalid_toml() {
        assert!(parse("this is = not valid = toml").is_err());
    }
}
