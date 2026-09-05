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
}

#[derive(Debug, Clone, Serialize)]
pub struct UnifiedAuditReport {
    pub verdict: String,
    pub composite_score: f64,
    pub math_breakdown: Value,
    pub critical_violations: Vec<String>,
    pub recommendations: Vec<String>,
}

struct WeightedScore {
    score: f64,
    weight: f64,
}

pub fn execute_unified_audit(args: Value) -> Result<Value, String> {
    let input: UnifiedAuditInput = serde_json::from_value(args)
        .map_err(|e| format!("Invalid arguments for math_audit_cognition: {}", e))?;

    let mut critical_violations = Vec::new();
    let mut recommendations = Vec::new();
    let mut weighted_scores: Vec<WeightedScore> = Vec::new();

    // 1. Constraint Verification (Weight: 0.30)
    let constraint_report = if !input.user_requirements.is_empty() {
        let mut impl_claims: Vec<String> = input.executed_steps.clone();
        for task in &input.planned_tasks {
            if input.executed_steps.contains(&task.id) {
                impl_claims.push(task.name.clone());
            }
        }
        if let Some(ref text) = input.draft_response {
            impl_claims.push(text.clone());
        }
        let rep = ConstraintEngine::verify(&input.user_requirements, &impl_claims);

        if !rep.missing_requirements.is_empty() {
            critical_violations.push(format!(
                "Omission Violation: {} user requirement(s) not fulfilled: {:?}",
                rep.missing_requirements.len(),
                rep.missing_requirements
            ));
        }

        if !rep.contradictions.is_empty() {
            for c in &rep.contradictions {
                critical_violations.push(c.clone());
            }
        }

        if !rep.scope_creep_items.is_empty() {
            recommendations.push(format!(
                "Scope Creep Warning: Found unrequested actions: {:?}",
                rep.scope_creep_items
            ));
        }

        weighted_scores.push(WeightedScore {
            score: rep.alignment_score * 100.0,
            weight: 0.30,
        });
        Some(rep)
    } else {
        None
    };

    // 2. Plan DAG Verification (Weight: 0.15)
    let dag_report = if !input.planned_tasks.is_empty() {
        let mut dag = PlanDag::new();
        for t in &input.planned_tasks {
            dag.add_task(&t.id, &t.name, t.dependencies.clone());
        }

        for step in &input.executed_steps {
            if let Err(err) = dag.record_step(step) {
                critical_violations.push(err);
            }
        }

        let metrics = dag.evaluate_metrics();
        if metrics.scope_creep_count > 0 {
            recommendations.push(format!(
                "Graph Waste W > 0: {} step(s) executed outside approved plan DAG",
                metrics.scope_creep_count
            ));
        }

        weighted_scores.push(WeightedScore {
            score: metrics.coverage_ratio * 100.0,
            weight: 0.15,
        });
        Some(metrics)
    } else {
        None
    };

    // 3. Text & Epistemic Calibration Evaluation (Weight: 0.15)
    let (text_report, confidence_report) = if let Some(ref text) = input.draft_response {
        let rep = TextEvaluator::evaluate(text);
        if rep.is_verbose {
            recommendations.push(
                "Verbosity Warning: Information density is low with high token redundancy. Condense response.".to_string(),
            );
        }
        if rep.is_too_complex {
            recommendations.push(
                "Readability Warning: Syntax is overly convoluted. Increase clarity.".to_string(),
            );
        }
        for sug in &rep.suggestions {
            recommendations.push(sug.clone());
        }

        let conf = ConfidenceAnalyzer::analyze(text);
        if conf.verdict == "OVERCONFIDENT" {
            critical_violations.push(
                "Overconfidence Violation: Absolute claims ('guaranteed', '100%') made without proof. Moderate claims or provide citations."
                    .to_string(),
            );
        } else if conf.verdict == "EVASIVE" {
            recommendations.push(
                "Evasive Language Warning: Text displays excessive hedging. Provide grounded answers."
                    .to_string(),
            );
        }
        for c in &conf.self_contradictions {
            critical_violations.push(format!("Self-Contradiction in Response: {}", c));
        }

        weighted_scores.push(WeightedScore {
            score: conf.calibration_score * 100.0,
            weight: 0.15,
        });
        (Some(rep), Some(conf))
    } else {
        (None, None)
    };

    // 4. Research Gate Evaluation (Weight: 0.10)
    let research_report = if let Some(ref text) = input.draft_response {
        let r_rep = ResearchGate::audit(text);
        if r_rep.has_research_deficit {
            critical_violations.push(
                "Research Deficit: Factual technical assertions made without citations. Verify with docs, RFCs, or test logs."
                    .to_string(),
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
    };

    // 5. Foresight & Diligence Evaluation (Weight: 0.10)
    let foresight_report = if input.draft_response.is_some()
        || input.code_snippet.is_some()
        || !input.planned_tasks.is_empty()
    {
        let f_rep = ForesightEngine::evaluate(
            input.draft_response.as_deref(),
            input.code_snippet.as_deref(),
            input.user_requirements.len(),
            input.planned_tasks.len(),
        );
        if f_rep.is_lazy_plan {
            critical_violations.push(
                "Lazy Plan Violation: High requirement count but shallow plan breakdown (<=1 task). Decompose plan into concrete steps."
                    .to_string(),
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

    // 6. Code Metrics Evaluation (Weight: 0.20)
    let code_report = if let Some(ref code) = input.code_snippet {
        let lang = input.language.as_deref().unwrap_or("rust");
        let rep = CodeAnalyzer::analyze(code, lang);

        if rep.has_syntax_errors {
            critical_violations.push(format!(
                "Syntax Error: Found {} parsing/AST error(s) in code",
                rep.syntax_error_count
            ));
        }

        if rep.cyclomatic_complexity > 20 {
            recommendations.push(format!(
                "High Cyclomatic Complexity M={}: Refactor into smaller sub-functions",
                rep.cyclomatic_complexity
            ));
        }

        if rep.maintainability_index < 50.0 {
            recommendations.push(format!(
                "Low Maintainability Index MI={:.1}: Code is hard to maintain and prone to bugs",
                rep.maintainability_index
            ));
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
    };

    let composite_score = if weighted_scores.is_empty() {
        100.0
    } else {
        let total_weight: f64 = weighted_scores.iter().map(|ws| ws.weight).sum();
        let weighted_sum: f64 = weighted_scores.iter().map(|ws| ws.score * ws.weight).sum();
        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            100.0
        }
    };

    let verdict = if critical_violations.is_empty() && composite_score >= 70.0 {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };

    let report = UnifiedAuditReport {
        verdict,
        composite_score,
        math_breakdown: json!({
            "constraints": constraint_report,
            "dag": dag_report,
            "text": text_report,
            "confidence": confidence_report,
            "research": research_report,
            "foresight": foresight_report,
            "code": code_report,
        }),
        critical_violations,
        recommendations,
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
        assert_eq!(val["verdict"], "PASS");
        assert_eq!(val["composite_score"], 100.0);
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
        assert_eq!(val["verdict"], "PASS");
        assert!(val["composite_score"].as_f64().unwrap() > 70.0);
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
        assert_eq!(val["verdict"], "FAIL");
        let violations = val["critical_violations"].as_array().unwrap();
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_unified_audit_invalid_json() {
        let args = json!("not an object");
        let result = execute_unified_audit(args);
        assert!(result.is_err());
    }
}
