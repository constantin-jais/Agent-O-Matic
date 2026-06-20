//! Planning-only Rumble → Bolt handoff validation.
//!
//! This module is deliberately small and deterministic. It validates the first
//! `ImplementationHandoff` contract before any product UI starts depending on
//! Bolt execution. MVP scope: validate/refuse/produce a dry-run planning report;
//! never execute implementation work.

use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::error::{Error, Result};

const SUPPORTED_FORMAT: &str = "canvas.bolt_handoff.v0.1";
const SUPPORTED_KIND: &str = "planning_request";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindingSeverity {
    Error,
    Warning,
}

impl FindingSeverity {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffFinding {
    pub severity: FindingSeverity,
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffReport {
    pub format: Option<String>,
    pub kind: Option<String>,
    pub handoff_id: Option<String>,
    pub package_id: Option<String>,
    pub package_hash: Option<String>,
    pub planning_goal: Option<String>,
    pub requested_outputs: Vec<String>,
    pub findings: Vec<HandoffFinding>,
}

impl HandoffReport {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DryRunPlan {
    pub report: HandoffReport,
    pub gates: Vec<String>,
    pub tasks: Vec<String>,
}

pub fn validate_file(path: &Path) -> Result<HandoffReport> {
    let source = fs::read_to_string(path).map_err(|source| Error::Io {
        path: path.display().to_string(),
        source,
    })?;
    validate_str(&source)
}

pub fn validate_str(source: &str) -> Result<HandoffReport> {
    let payload: Value = serde_json::from_str(source).map_err(|err| Error::InvalidHandoff {
        message: format!("invalid JSON: {err}"),
    })?;
    Ok(validate_payload(&payload))
}

pub fn dry_run_plan_file(path: &Path) -> Result<DryRunPlan> {
    let report = validate_file(path)?;
    let mut gates = vec![
        "package_approved".to_string(),
        "planning_only_enforced".to_string(),
        "traceability_present".to_string(),
        "blocking_questions_waived_or_absent".to_string(),
        "high_risks_waived_or_absent".to_string(),
        "shared_capability_review_requested".to_string(),
    ];
    let mut tasks = Vec::new();

    if report.is_valid() {
        for output in &report.requested_outputs {
            match output.as_str() {
                "implementation_plan" => tasks.push("draft implementation plan".to_string()),
                "task_breakdown" => tasks.push("derive task breakdown".to_string()),
                "risk_review" => tasks.push("review risks and waivers".to_string()),
                "test_plan" => tasks.push("derive acceptance and contract test plan".to_string()),
                "shared_capability_extraction_review" => {
                    tasks.push("review shared capability extraction".to_string())
                }
                other => tasks.push(format!("produce requested output `{other}`")),
            }
        }
    } else {
        gates.push("refuse_until_validation_clean".to_string());
    }

    Ok(DryRunPlan {
        report,
        gates,
        tasks,
    })
}

fn validate_payload(payload: &Value) -> HandoffReport {
    let mut findings = Vec::new();

    let format = string_at(payload, &["format"]);
    if format.as_deref() != Some(SUPPORTED_FORMAT) {
        findings.push(error(
            "unsupported_format",
            format!(
                "expected format `{SUPPORTED_FORMAT}`, got `{}`",
                format.as_deref().unwrap_or("<missing>")
            ),
        ));
    }

    let kind = string_at(payload, &["kind"]);
    if kind.as_deref() != Some(SUPPORTED_KIND) {
        findings.push(error(
            "unsupported_kind",
            format!(
                "expected kind `{SUPPORTED_KIND}`, got `{}`",
                kind.as_deref().unwrap_or("<missing>")
            ),
        ));
    }

    let handoff_id = string_at(payload, &["source", "handoff_id"]);
    require_string(
        &mut findings,
        payload,
        &["source", "product"],
        "missing_source_product",
    );
    require_string(
        &mut findings,
        payload,
        &["source", "workspace_id"],
        "missing_workspace_id",
    );
    require_string(
        &mut findings,
        payload,
        &["source", "handoff_id"],
        "missing_handoff_id",
    );
    require_string(
        &mut findings,
        payload,
        &["source", "created_by"],
        "missing_created_by",
    );
    require_string(
        &mut findings,
        payload,
        &["source", "created_at"],
        "missing_created_at",
    );

    let package_id = string_at(payload, &["package", "package_id"]);
    let package_hash = string_at(payload, &["package", "package_hash"]);
    require_string(
        &mut findings,
        payload,
        &["package", "package_id"],
        "missing_package_id",
    );
    require_string(
        &mut findings,
        payload,
        &["package", "version"],
        "missing_package_version",
    );
    validate_package_hash(&mut findings, package_hash.as_deref());
    validate_non_empty_array(
        &mut findings,
        payload,
        &["package", "items"],
        "empty_package_items",
    );

    let planning_goal = string_at(payload, &["planning_scope", "goal"]);
    require_string(
        &mut findings,
        payload,
        &["planning_scope", "mode"],
        "missing_planning_mode",
    );
    require_string(
        &mut findings,
        payload,
        &["planning_scope", "goal"],
        "missing_planning_goal",
    );

    validate_non_empty_array(
        &mut findings,
        payload,
        &["traceability_links"],
        "missing_traceability_links",
    );
    validate_blockers(payload, &mut findings);
    validate_risks(payload, &mut findings);
    validate_capability_candidates(payload, &mut findings);
    validate_execution_policy(payload, &mut findings);

    let requested_outputs = array_strings_at(payload, &["requested_outputs"]);
    if requested_outputs.is_empty() {
        findings.push(warning(
            "missing_requested_outputs",
            "no requested outputs declared; dry-run plan will be minimal".to_string(),
        ));
    }

    HandoffReport {
        format,
        kind,
        handoff_id,
        package_id,
        package_hash,
        planning_goal,
        requested_outputs,
        findings,
    }
}

fn validate_execution_policy(payload: &Value, findings: &mut Vec<HandoffFinding>) {
    let planning_only = bool_at(payload, &["execution_policy", "planning_only"]);
    let allow_execution = bool_at(payload, &["execution_policy", "allow_execution"]);
    let requires_human_approval = bool_at(
        payload,
        &["execution_policy", "requires_human_approval_for_execution"],
    );

    if planning_only != Some(true) {
        findings.push(error(
            "planning_only_required",
            "execution_policy.planning_only must be true".to_string(),
        ));
    }
    if allow_execution != Some(false) {
        findings.push(error(
            "execution_forbidden",
            "execution_policy.allow_execution must be false".to_string(),
        ));
    }
    if requires_human_approval != Some(true) {
        findings.push(error(
            "human_approval_required",
            "execution_policy.requires_human_approval_for_execution must be true".to_string(),
        ));
    }
}

fn validate_package_hash(findings: &mut Vec<HandoffFinding>, hash: Option<&str>) {
    match hash {
        Some(value)
            if value.starts_with("sha256:")
                && value.len() == "sha256:".len() + 64
                && value["sha256:".len()..]
                    .chars()
                    .all(|c| c.is_ascii_hexdigit()) => {}
        Some(value) => findings.push(error(
            "invalid_package_hash",
            format!("package_hash must be sha256:<64 hex chars>, got `{value}`"),
        )),
        None => findings.push(error(
            "missing_package_hash",
            "package.package_hash is required".to_string(),
        )),
    }
}

fn validate_blockers(payload: &Value, findings: &mut Vec<HandoffFinding>) {
    let has_accepted_waiver = has_accepted_waiver(payload);
    for question in array_at(payload, &["open_questions"]) {
        let impact = string_field(question, "impact");
        let status = string_field(question, "status");
        if impact.as_deref() == Some("blocking")
            && status.as_deref() == Some("open")
            && !has_accepted_waiver
        {
            findings.push(error(
                "blocking_question_without_waiver",
                "blocking open question requires an accepted active waiver".to_string(),
            ));
        }
    }
}

fn validate_risks(payload: &Value, findings: &mut Vec<HandoffFinding>) {
    let has_accepted_waiver = has_accepted_waiver(payload);
    for risk in array_at(payload, &["risks"]) {
        let severity = string_field(risk, "severity");
        let status = string_field(risk, "status");
        let high = matches!(
            severity.as_deref(),
            Some("high") | Some("critical") | Some("blocking")
        );
        let open = !matches!(
            status.as_deref(),
            Some("mitigated") | Some("accepted") | Some("waived")
        );
        if high && open && !has_accepted_waiver {
            findings.push(error(
                "high_risk_without_waiver",
                "high/critical open risk requires mitigation or an accepted active waiver"
                    .to_string(),
            ));
        }
    }
}

fn validate_capability_candidates(payload: &Value, findings: &mut Vec<HandoffFinding>) {
    for candidate in array_at(payload, &["capability_candidates"]) {
        let owner = string_field(candidate, "proposed_owner_layer");
        if owner.as_deref().unwrap_or_default().trim().is_empty() {
            findings.push(warning(
                "capability_owner_missing",
                "capability candidate has no proposed_owner_layer".to_string(),
            ));
        }
    }
}

fn has_accepted_waiver(payload: &Value) -> bool {
    array_at(payload, &["active_waivers"]).iter().any(|waiver| {
        matches!(
            string_field(waiver, "status").as_deref(),
            Some("accepted") | Some("active")
        )
    })
}

fn require_string(
    findings: &mut Vec<HandoffFinding>,
    payload: &Value,
    path: &[&str],
    code: &'static str,
) {
    if string_at(payload, path).is_none() {
        findings.push(error(
            code,
            format!("missing required string `{}`", path.join(".")),
        ));
    }
}

fn validate_non_empty_array(
    findings: &mut Vec<HandoffFinding>,
    payload: &Value,
    path: &[&str],
    code: &'static str,
) {
    match value_at(payload, path).and_then(Value::as_array) {
        Some(values) if !values.is_empty() => {}
        _ => findings.push(error(
            code,
            format!("`{}` must be a non-empty array", path.join(".")),
        )),
    }
}

fn value_at<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for segment in path {
        current = current.get(*segment)?;
    }
    Some(current)
}

fn string_at(value: &Value, path: &[&str]) -> Option<String> {
    value_at(value, path)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn bool_at(value: &Value, path: &[&str]) -> Option<bool> {
    value_at(value, path).and_then(Value::as_bool)
}

fn array_at<'a>(value: &'a Value, path: &[&str]) -> Vec<&'a Value> {
    value_at(value, path)
        .and_then(Value::as_array)
        .map(|values| values.iter().collect())
        .unwrap_or_default()
}

fn array_strings_at(value: &Value, path: &[&str]) -> Vec<String> {
    value_at(value, path)
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn string_field(value: &Value, field: &str) -> Option<String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn error(code: &'static str, message: String) -> HandoffFinding {
    HandoffFinding {
        severity: FindingSeverity::Error,
        code,
        message,
    }
}

fn warning(code: &'static str, message: String) -> HandoffFinding {
    HandoffFinding {
        severity: FindingSeverity::Warning,
        code,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_payload() -> String {
        r#"{
          "format":"canvas.bolt_handoff.v0.1",
          "kind":"planning_request",
          "source":{"product":"rumble-canvas","workspace_id":"w","handoff_id":"h","created_by":"a","created_at":"2026-06-30T00:00:00Z"},
          "package":{"package_id":"p","version":"0.1.0","package_hash":"sha256:0000000000000000000000000000000000000000000000000000000000000000","items":[{"section_id":"s","revision_id":"r"}]},
          "planning_scope":{"mode":"full_package","target_objects":[],"excluded_objects":[],"goal":"Plan only"},
          "spec_context":{},
          "traceability_links":[{"source_type":"journey","source_id":"j","target_type":"action","target_id":"a","relation_type":"implements"}],
          "active_waivers":[],
          "open_questions":[],
          "risks":[],
          "capability_candidates":[],
          "requested_outputs":["implementation_plan"],
          "execution_policy":{"planning_only":true,"allow_execution":false,"requires_human_approval_for_execution":true}
        }"#
        .to_string()
    }

    #[test]
    fn accepts_valid_planning_only_payload() {
        let report = validate_str(&valid_payload()).unwrap();
        assert!(report.is_valid(), "{:#?}", report.findings);
    }

    #[test]
    fn rejects_execution_enabled_payload() {
        let payload =
            valid_payload().replace("\"allow_execution\":false", "\"allow_execution\":true");
        let report = validate_str(&payload).unwrap();
        assert!(!report.is_valid());
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.code == "execution_forbidden")
        );
    }

    #[test]
    fn rejects_missing_traceability() {
        let payload = valid_payload().replace(
            "\"traceability_links\":[{\"source_type\":\"journey\",\"source_id\":\"j\",\"target_type\":\"action\",\"target_id\":\"a\",\"relation_type\":\"implements\"}]",
            "\"traceability_links\":[]",
        );
        let report = validate_str(&payload).unwrap();
        assert!(!report.is_valid());
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.code == "missing_traceability_links")
        );
    }
}
