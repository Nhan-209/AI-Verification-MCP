use crate::engine::{
    CodeAnalyzer, ConfidenceAnalyzer, ConstraintEngine, ForesightEngine, PlanDag, ResearchGate,
    TextEvaluator,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Deserialize)]
pub struct PlanTaskInput {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnifiedAuditInput {
    #[serde(default)]
    pub user_requirements: Vec<String>,
    #[serde(default)]
    pub planned_tasks: Vec<PlanTaskInput>,
    #[serde(default)]
    pub executed_steps: Vec<String>,
    #[serde(default)]
    pub draft_response: Option<String>,
    #[serde(default)]
    pub code_snippet: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ViolationSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditViolation {
    pub code: String,
    pub message: String,
    pub severity: ViolationSeverity,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeveritySummary {
    pub critical: usize,
    pub warning: usize,
    pub info: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAuditReport {
    pub decision: String,
    pub verdict: String,
    pub policy_score: f64,
    #[serde(default)]
    pub composite_score: f64,
    pub severity_summary: SeveritySummary,
    pub math_breakdown: Value,
    pub critical_violations: Vec<String>,
    pub violations: Vec<AuditViolation>,
    pub recommendations: Vec<String>,
    pub remediation_plan: Vec<String>,
}

struct WeightedScore {
    score: f64,
    weight: f64,
}

fn add_violation(
    violations: &mut Vec<AuditViolation>,
    critical_violations: &mut Vec<String>,
    recommendations: &mut Vec<String>,
    code: &str,
    message: String,
    severity: ViolationSeverity,
    remediation: &str,
) {
    match severity {
        ViolationSeverity::Critical => {
            critical_violations.push(message.clone());
        }
        ViolationSeverity::Warning | ViolationSeverity::Info => {
            recommendations.push(message.clone());
        }
    }
    violations.push(AuditViolation {
        code: code.to_string(),
        message,
        severity,
        remediation: remediation.to_string(),
    });
}

pub fn execute_unified_audit(args: Value) -> Result<Value, String> {
    let input: UnifiedAuditInput = serde_json::from_value(args)
        .map_err(|e| format!("Invalid arguments for math_audit_cognition: {}", e))?;

    let mode_str = input.mode.as_deref().unwrap_or("standard").to_lowercase();
    let is_quick = mode_str == "quick";
    let is_deep = mode_str == "deep";

    let mut violations: Vec<AuditViolation> = Vec::new();
    let mut critical_violations = Vec::new();
    let mut recommendations = Vec::new();
    let mut weighted_scores: Vec<WeightedScore> = Vec::new();

    // 1. Constraint Verification (Standard: 0.30, Quick: 0.50)
    let constraint_report = if !input.user_requirements.is_empty() {
        let mut impl_claims: Vec<String> = Vec::new();
        let known_ids: std::collections::HashSet<&str> =
            input.planned_tasks.iter().map(|t| t.id.as_str()).collect();

        for task in &input.planned_tasks {
            if input.executed_steps.contains(&task.id) {
                impl_claims.push(task.name.clone());
            }
        }

        for step in &input.executed_steps {
            if !known_ids.contains(step.as_str()) && !PlanDag::is_exploratory_action(step) {
                impl_claims.push(step.clone());
            }
        }

        if impl_claims.is_empty() {
            if let Some(ref text) = input.draft_response {
                impl_claims.push(text.clone());
            }
        }
        let rep = ConstraintEngine::verify(&input.user_requirements, &impl_claims);

        if !rep.missing_requirements.is_empty() {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "CONSTRAINT_CONFLICT",
                format!(
                    "Omission Violation: {} user requirement(s) not fulfilled: {:?}",
                    rep.missing_requirements.len(),
                    rep.missing_requirements
                ),
                ViolationSeverity::Critical,
                "Implement missing user requirements or update specification scope.",
            );
        }

        if !rep.contradictions.is_empty() {
            for c in &rep.contradictions {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "CONSTRAINT_CONFLICT",
                    c.clone(),
                    ViolationSeverity::Critical,
                    "Resolve contradictory constraints between user requirements and implementation.",
                );
            }
        }

        if !rep.scope_creep_items.is_empty() {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "SCOPE_CREEP",
                format!(
                    "Scope Creep Warning: Found unrequested actions: {:?}",
                    rep.scope_creep_items
                ),
                ViolationSeverity::Warning,
                "Align actions strictly with approved requirements or request scope change.",
            );
        }

        let weight = if is_quick { 0.50 } else { 0.30 };
        weighted_scores.push(WeightedScore {
            score: rep.alignment_score * 100.0,
            weight,
        });
        Some(rep)
    } else {
        None
    };

    // 2. Plan DAG Verification (Standard: 0.15, Quick: 0.25)
    let dag_report = if !input.planned_tasks.is_empty() {
        let mut dag = PlanDag::new();
        for t in &input.planned_tasks {
            dag.add_task(&t.id, &t.name, t.dependencies.clone());
        }

        for step in &input.executed_steps {
            if let Err(err) = dag.record_step(step) {
                if err.contains("Dependency violation") {
                    add_violation(
                        &mut violations,
                        &mut critical_violations,
                        &mut recommendations,
                        "PLAN_DEPENDENCY_ERROR",
                        err,
                        ViolationSeverity::Critical,
                        "Reorder tasks according to DAG topological dependencies.",
                    );
                } else {
                    add_violation(
                        &mut violations,
                        &mut critical_violations,
                        &mut recommendations,
                        "SCOPE_CREEP",
                        err,
                        ViolationSeverity::Warning,
                        "Register unplanned tasks into plan DAG or remove extraneous execution steps.",
                    );
                }
            }
        }

        let metrics = dag.evaluate_metrics();
        if metrics.scope_creep_count > 0 && !violations.iter().any(|v| v.code == "SCOPE_CREEP") {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "SCOPE_CREEP",
                format!(
                    "Graph Waste W > 0: {} step(s) executed outside approved plan DAG",
                    metrics.scope_creep_count
                ),
                ViolationSeverity::Warning,
                "Register unplanned tasks into plan DAG or remove extraneous execution steps.",
            );
        }

        if is_deep && metrics.coverage_ratio < 1.0 && !input.executed_steps.is_empty() {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "PLAN_COVERAGE_DEFICIT",
                format!(
                    "Deep Audit Warning: Plan execution incomplete (coverage = {:.1}%)",
                    metrics.coverage_ratio * 100.0
                ),
                ViolationSeverity::Warning,
                "Complete all planned tasks in DAG before final delivery.",
            );
        }

        let weight = if is_quick { 0.25 } else { 0.15 };
        weighted_scores.push(WeightedScore {
            score: metrics.coverage_ratio * 100.0,
            weight,
        });
        Some(metrics)
    } else {
        None
    };

    // 3. Text & Epistemic Calibration Evaluation
    let (text_report, confidence_report) = if let Some(ref text) = input.draft_response {
        let rep = if !is_quick {
            let tr = TextEvaluator::evaluate(text);
            if tr.is_verbose {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "VERBOSITY_WARNING",
                    "Verbosity Warning: Information density is low with high token redundancy. Condense response.".to_string(),
                    ViolationSeverity::Info,
                    "Condense text, eliminate filler phrases, and state answers concisely.",
                );
            }
            if tr.is_too_complex {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "READABILITY_WARNING",
                    "Readability Warning: Syntax is overly convoluted. Increase clarity.".to_string(),
                    ViolationSeverity::Info,
                    "Simplify phrasing and shorten compound sentences.",
                );
            }
            for sug in &tr.suggestions {
                recommendations.push(sug.clone());
            }
            Some(tr)
        } else {
            None
        };

        let conf = ConfidenceAnalyzer::analyze(text);
        if conf.verdict == "OVERCONFIDENT" {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "CONFIDENCE_UNCALIBRATED",
                "Overconfidence Violation: Absolute claims ('guaranteed', '100%') made without empirical proof. Moderate claims or provide citations.".to_string(),
                ViolationSeverity::Critical,
                "Tone down absolute claims or provide concrete citations and reproducible evidence.",
            );
        } else if conf.verdict == "EVASIVE" {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "CONFIDENCE_UNCALIBRATED",
                "Evasive Language Warning: Text displays excessive hedging. Provide grounded answers.".to_string(),
                ViolationSeverity::Warning,
                "Remove excessive hedging and make direct, evidenced claims.",
            );
        }
        for c in &conf.self_contradictions {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "LOGICAL_CONTRADICTION",
                format!("Self-Contradiction in Response: {}", c),
                ViolationSeverity::Critical,
                "Resolve contradictory statements within the response.",
            );
        }

        let weight = if is_quick { 0.25 } else { 0.15 };
        weighted_scores.push(WeightedScore {
            score: conf.calibration_score * 100.0,
            weight,
        });
        (rep, Some(conf))
    } else {
        (None, None)
    };

    // 4. Research Gate Evaluation (Skipped in quick mode)
    let research_report = if !is_quick {
        if let Some(ref text) = input.draft_response {
            let r_rep = ResearchGate::audit(text);
            if r_rep.has_research_deficit {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "RESEARCH_DEFICIT",
                    "Research Deficit: Factual technical assertions made without citations. Verify with docs, RFCs, or test logs.".to_string(),
                    ViolationSeverity::Critical,
                    "Ground factual claims with official documentation links, RFCs, or benchmark citations.",
                );
            }
            for rec in &r_rep.recommendations {
                recommendations.push(rec.clone());
            }

            weighted_scores.push(WeightedScore {
                score: r_rep.research_score,
                weight: 0.10,
            });
            Some(r_rep)
        } else {
            None
        }
    } else {
        None
    };

    // 5. Foresight & Diligence Evaluation (Skipped in quick mode)
    let foresight_report = if !is_quick
        && (input.draft_response.is_some()
            || input.code_snippet.is_some()
            || !input.planned_tasks.is_empty())
    {
        let f_rep = ForesightEngine::evaluate(
            input.draft_response.as_deref(),
            input.code_snippet.as_deref(),
            input.user_requirements.len(),
            input.planned_tasks.len(),
        );
        if f_rep.is_lazy_plan {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "LAZY_PLAN",
                "Lazy Plan Violation: High requirement count but shallow plan breakdown (<=1 task). Decompose plan into concrete steps.".to_string(),
                ViolationSeverity::Critical,
                "Break down complex requirement list into discrete, measurable subtasks.",
            );
        }
        for rec in &f_rep.recommendations {
            recommendations.push(rec.clone());
        }

        weighted_scores.push(WeightedScore {
            score: f_rep.foresight_score,
            weight: 0.10,
        });
        Some(f_rep)
    } else {
        None
    };

    // 6. Code Metrics Evaluation (Skipped in quick mode)
    let code_report = if !is_quick {
        if let Some(ref code) = input.code_snippet {
            let lang = input.language.as_deref().unwrap_or("rust");
            let rep = CodeAnalyzer::analyze(code, lang);

            if rep.has_syntax_errors {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "SYNTAX_ERROR",
                    format!(
                        "Syntax Error: Found {} parsing/AST error(s) in code",
                        rep.syntax_error_count
                    ),
                    ViolationSeverity::Critical,
                    "Fix code syntax errors and ensure AST parses cleanly.",
                );
            }

            let cyclomatic_threshold = if is_deep { 15 } else { 20 };
            if rep.cyclomatic_complexity > cyclomatic_threshold {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "COMPLEXITY_WARNING",
                    format!(
                        "High Cyclomatic Complexity M={}: Refactor into smaller sub-functions",
                        rep.cyclomatic_complexity
                    ),
                    ViolationSeverity::Warning,
                    "Refactor monolithic functions into smaller, single-responsibility helper functions.",
                );
            }

            let mi_threshold = if is_deep { 65.0 } else { 50.0 };
            if rep.maintainability_index < mi_threshold {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "MAINTAINABILITY_DEFICIT",
                    format!(
                        "Low Maintainability Index MI={:.1}: Code is hard to maintain and prone to bugs",
                        rep.maintainability_index
                    ),
                    ViolationSeverity::Warning,
                    "Simplify control flow, shorten function length, and reduce operand volume.",
                );
            }

            for bw in &rep.boundary_warnings {
                recommendations.push(format!("Boundary Condition Warning: {}", bw));
            }

            weighted_scores.push(WeightedScore {
                score: rep.maintainability_index,
                weight: 0.20,
            });
            Some(rep)
        } else {
            None
        }
    } else {
        None
    };

    let has_any_input = !input.user_requirements.is_empty()
        || !input.planned_tasks.is_empty()
        || !input.executed_steps.is_empty()
        || input
            .draft_response
            .as_ref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
        || input
            .code_snippet
            .as_ref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);

    if !has_any_input {
        add_violation(
            &mut violations,
            &mut critical_violations,
            &mut recommendations,
            "NO_INPUT_PROVIDED",
            "Empty input payload: Cannot verify empty claims or missing execution trace.".to_string(),
            ViolationSeverity::Info,
            "Provide user_requirements, planned_tasks, draft_response, or code_snippet for verification.",
        );
    }

    let policy_score = if !has_any_input || weighted_scores.is_empty() {
        0.0
    } else {
        let total_weight: f64 = weighted_scores.iter().map(|ws| ws.weight).sum();
        let weighted_sum: f64 = weighted_scores.iter().map(|ws| ws.score * ws.weight).sum();
        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        }
    };
    let composite_score = policy_score;

    let critical_count = violations
        .iter()
        .filter(|v| v.severity == ViolationSeverity::Critical)
        .count();
    let warning_count = violations
        .iter()
        .filter(|v| v.severity == ViolationSeverity::Warning)
        .count();
    let info_count = violations
        .iter()
        .filter(|v| v.severity == ViolationSeverity::Info)
        .count();

    let severity_summary = SeveritySummary {
        critical: critical_count,
        warning: warning_count,
        info: info_count,
    };

    let (decision, verdict) = if !has_any_input || weighted_scores.is_empty() {
        (
            "INSUFFICIENT_EVIDENCE".to_string(),
            "UNVERIFIED".to_string(),
        )
    } else if critical_count > 0 || policy_score < 50.0 {
        ("BLOCK".to_string(), "FAIL".to_string())
    } else if warning_count > 0 || policy_score < 75.0 {
        ("WARN".to_string(), "WARN".to_string())
    } else {
        ("ALLOW".to_string(), "PASS".to_string())
    };

    let mut remediation_plan: Vec<String> = Vec::new();
    for v in &violations {
        if !remediation_plan.contains(&v.remediation) {
            remediation_plan.push(v.remediation.clone());
        }
    }

    let report = UnifiedAuditReport {
        decision,
        verdict,
        policy_score,
        composite_score,
        severity_summary,
        math_breakdown: json!({
            "mode": mode_str,
            "constraints": constraint_report,
            "dag": dag_report,
            "text": text_report,
            "confidence": confidence_report,
            "research": research_report,
            "foresight": foresight_report,
            "code": code_report,
        }),
        critical_violations,
        violations,
        recommendations,
        remediation_plan,
    };

    serde_json::to_value(report).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_audit_empty_input() {
        let args = json!({});
        let result = execute_unified_audit(args);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["decision"], "INSUFFICIENT_EVIDENCE");
        assert_eq!(val["verdict"], "UNVERIFIED");
        assert_eq!(val["policy_score"], 0.0);
        assert_eq!(val["composite_score"], 0.0);
        assert_eq!(val["severity_summary"]["critical"], 0);
    }

    #[test]
    fn test_unified_audit_full_happy_path() {
        let args = json!({
            "user_requirements": ["implement helper", "add tests"],
            "planned_tasks": [
                {"id": "t1", "name": "implement helper", "dependencies": []},
                {"id": "t2", "name": "add tests", "dependencies": ["t1"]}
            ],
            "executed_steps": ["t1", "t2"],
            "draft_response": "According to docs.rs and RFC 1234, the helper is implemented in helper.rs with assertions, fallback retry, and unit test coverage. See: https://docs.rs/example",
            "code_snippet": "fn helper() -> Result<bool, String> { Ok(true) }",
            "language": "rust"
        });
        let result = execute_unified_audit(args);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["decision"], "ALLOW");
        assert_eq!(val["verdict"], "PASS");
        assert!(val["composite_score"].as_f64().unwrap() > 70.0);
        assert_eq!(val["policy_score"], val["composite_score"]);
        assert_eq!(val["severity_summary"]["critical"], 0);
        assert!(val["critical_violations"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_unified_audit_critical_violations() {
        let args = json!({
            "user_requirements": ["implement helper", "secure auth", "database migration"],
            "planned_tasks": [
                {"id": "t1", "name": "implement helper", "dependencies": []}
            ],
            "executed_steps": ["t1"],
            "draft_response": "This is guaranteed 100% flawless and will never fail.",
        });
        let result = execute_unified_audit(args);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["decision"], "BLOCK");
        assert_eq!(val["verdict"], "FAIL");
        let violations = val["violations"].as_array().unwrap();
        assert!(!violations.is_empty());
        assert!(val["severity_summary"]["critical"].as_u64().unwrap() > 0);
        assert!(!val["remediation_plan"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_unified_audit_quick_mode() {
        let args = json!({
            "mode": "quick",
            "user_requirements": ["must be fast"],
            "executed_steps": ["must be fast"],
            "draft_response": "Execution is verified by tests in tests/bench.rs."
        });
        let result = execute_unified_audit(args);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["decision"], "ALLOW");
        assert_eq!(val["math_breakdown"]["mode"], "quick");
        assert!(val["math_breakdown"]["code"].is_null());
        assert!(val["math_breakdown"]["research"].is_null());
    }

    #[test]
    fn test_unified_audit_warning_tier() {
        let args = json!({
            "user_requirements": ["handle request"],
            "planned_tasks": [
                {"id": "t1", "name": "handle request", "dependencies": []}
            ],
            "executed_steps": ["t1", "unplanned_extra_step"],
            "draft_response": "I think maybe this might work perhaps. Reference: https://docs.rs/example"
        });
        let result = execute_unified_audit(args);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["decision"], "WARN");
        assert_eq!(val["verdict"], "WARN");
        assert!(val["severity_summary"]["warning"].as_u64().unwrap() > 0);
    }

    #[test]
    fn test_unified_audit_invalid_json() {
        let args = json!("not an object");
        let result = execute_unified_audit(args);
        assert!(result.is_err());
    }
}
