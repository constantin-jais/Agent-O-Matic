//! Orchestrator — the agentic CI/CD control loop built on top of the
//! `agent_o_matic` compiler. Primitives so far: incident -> issue (idempotent
//! GitHub issue creation) and dispatch (a *bounded* hand-off to a fixer agent —
//! isolated branch, single attempt, never merges). Goals & gates live in the
//! compiler (ADR: goals-safe-declarative-checks).

pub mod dispatch;
pub mod forge;
pub mod incident;
