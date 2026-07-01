//! Branch ownership policy for bounded repository autonomy.
//!
//! The agent may own branches inside an explicit namespace (for example
//! `bolt/run/<run-id>/...`) but must never own the repository, protected
//! branches, settings, or secrets.

use std::fmt;

/// A candidate branch owned by the agent for one bounded attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttemptBranch {
    name: String,
}

impl AttemptBranch {
    /// Build a deterministic branch for one run/issue/attempt tuple.
    pub fn new(run_id: &str, issue: u64, attempt: u32) -> Result<Self, BranchPolicyError> {
        if attempt == 0 {
            return Err(BranchPolicyError::InvalidBranch(
                "attempt must be >= 1".to_string(),
            ));
        }
        let run = sanitize_segment(run_id)?;
        Ok(Self {
            name: format!("bolt/run/{run}/issue-{issue}/attempt-{attempt}"),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }
}

/// Policy for branch create/push/delete operations.
#[derive(Debug, Clone)]
pub struct BranchPolicy {
    owned_prefixes: Vec<String>,
    protected_branches: Vec<String>,
}

impl BranchPolicy {
    /// Default policy for Bolt-owned branches.
    pub fn bolt_default() -> Self {
        Self {
            owned_prefixes: vec!["bolt/".to_string()],
            protected_branches: vec![
                "main".to_string(),
                "master".to_string(),
                "develop".to_string(),
                "release".to_string(),
            ],
        }
    }

    pub fn new(owned_prefixes: Vec<String>, protected_branches: Vec<String>) -> Self {
        Self {
            owned_prefixes,
            protected_branches,
        }
    }

    pub fn validate_create(&self, branch: &str) -> Result<(), BranchPolicyError> {
        self.validate_owned_branch(branch)
    }

    pub fn validate_push(&self, branch: &str) -> Result<(), BranchPolicyError> {
        self.validate_owned_branch(branch)
    }

    /// Deletion is intentionally allowed only for agent-owned branches. Repo
    /// deletion, protected branch deletion, and human branch deletion are outside
    /// this policy and must be impossible with the token/settings layer too.
    pub fn validate_delete(&self, branch: &str) -> Result<(), BranchPolicyError> {
        self.validate_owned_branch(branch)
    }

    pub fn is_owned(&self, branch: &str) -> bool {
        self.validate_owned_branch(branch).is_ok()
    }

    fn validate_owned_branch(&self, branch: &str) -> Result<(), BranchPolicyError> {
        validate_branch_syntax(branch)?;
        if self.protected_branches.iter().any(|b| b == branch) {
            return Err(BranchPolicyError::ProtectedBranch(branch.to_string()));
        }
        if !self
            .owned_prefixes
            .iter()
            .any(|prefix| branch.starts_with(prefix))
        {
            return Err(BranchPolicyError::OutsideOwnedNamespace {
                branch: branch.to_string(),
                prefixes: self.owned_prefixes.clone(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchPolicyError {
    InvalidBranch(String),
    ProtectedBranch(String),
    OutsideOwnedNamespace {
        branch: String,
        prefixes: Vec<String>,
    },
}

impl fmt::Display for BranchPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchPolicyError::InvalidBranch(reason) => write!(f, "invalid branch: {reason}"),
            BranchPolicyError::ProtectedBranch(branch) => {
                write!(f, "protected branch is not agent-owned: {branch}")
            }
            BranchPolicyError::OutsideOwnedNamespace { branch, prefixes } => write!(
                f,
                "branch `{branch}` is outside the agent-owned namespace(s): {}",
                prefixes.join(", ")
            ),
        }
    }
}

impl std::error::Error for BranchPolicyError {}

fn sanitize_segment(input: &str) -> Result<String, BranchPolicyError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(BranchPolicyError::InvalidBranch(
            "run id must not be empty".to_string(),
        ));
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return Err(BranchPolicyError::InvalidBranch(format!(
            "unsafe run id segment `{trimmed}`"
        )));
    }
    Ok(trimmed.to_string())
}

fn validate_branch_syntax(branch: &str) -> Result<(), BranchPolicyError> {
    if branch.is_empty() {
        return Err(BranchPolicyError::InvalidBranch(
            "branch must not be empty".to_string(),
        ));
    }
    if branch.len() > 180 {
        return Err(BranchPolicyError::InvalidBranch(
            "branch is too long".to_string(),
        ));
    }
    if branch.starts_with('/') || branch.ends_with('/') {
        return Err(BranchPolicyError::InvalidBranch(
            "branch must not start or end with slash".to_string(),
        ));
    }
    if branch.contains("//") || branch.contains("..") || branch.contains("@{") {
        return Err(BranchPolicyError::InvalidBranch(
            "branch contains forbidden git ref sequence".to_string(),
        ));
    }
    if branch.ends_with(".lock") || branch.ends_with('.') {
        return Err(BranchPolicyError::InvalidBranch(
            "branch has forbidden git ref suffix".to_string(),
        ));
    }
    if branch.starts_with("refs/") {
        return Err(BranchPolicyError::InvalidBranch(
            "full refs are not accepted; pass a branch name".to_string(),
        ));
    }
    if !branch
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '-' | '_' | '.'))
    {
        return Err(BranchPolicyError::InvalidBranch(
            "branch contains unsafe characters".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_branch_is_deterministic_and_namespaced() {
        let branch = AttemptBranch::new("run-20260701", 42, 2).unwrap();
        assert_eq!(branch.as_str(), "bolt/run/run-20260701/issue-42/attempt-2");
        BranchPolicy::bolt_default()
            .validate_create(branch.as_str())
            .unwrap();
    }

    #[test]
    fn attempt_zero_is_rejected() {
        let err = AttemptBranch::new("run", 1, 0).unwrap_err();
        assert!(err.to_string().contains("attempt"));
    }

    #[test]
    fn unsafe_run_id_is_rejected() {
        let err = AttemptBranch::new("../main", 1, 1).unwrap_err();
        assert!(err.to_string().contains("unsafe run id"));
    }

    #[test]
    fn allows_create_push_delete_inside_owned_namespace() {
        let p = BranchPolicy::bolt_default();
        for op in [
            BranchPolicy::validate_create,
            BranchPolicy::validate_push,
            BranchPolicy::validate_delete,
        ] {
            op(&p, "bolt/run/r/issue-1/attempt-1").unwrap();
        }
    }

    #[test]
    fn refuses_protected_branches() {
        let p = BranchPolicy::bolt_default();
        assert!(matches!(
            p.validate_delete("main"),
            Err(BranchPolicyError::ProtectedBranch(_))
        ));
    }

    #[test]
    fn refuses_human_branch_outside_namespace() {
        let p = BranchPolicy::bolt_default();
        assert!(matches!(
            p.validate_push("feature/human-work"),
            Err(BranchPolicyError::OutsideOwnedNamespace { .. })
        ));
    }

    #[test]
    fn refuses_unsafe_ref_syntax() {
        let p = BranchPolicy::bolt_default();
        for branch in [
            "bolt//double",
            "bolt/../main",
            "refs/heads/bolt/x",
            "bolt/x.lock",
            "bolt/x@{1}",
            "bolt/x y",
        ] {
            assert!(
                matches!(
                    p.validate_create(branch),
                    Err(BranchPolicyError::InvalidBranch(_))
                ),
                "{branch} should be invalid"
            );
        }
    }
}
