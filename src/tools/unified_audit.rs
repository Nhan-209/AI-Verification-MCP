use crate::engine::{CodeAnalyzer, ConfidenceAnalyzer, ConstraintEngine, PlanDag, TextEvaluator};
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

    // 1. Constraint Verification (Weight: 0.40)
    let constraint_report = if !input.user_requirements.is_empty() {
        let impl_claims: Vec<String> = input
            .executed_steps
            .clone()
            .into_iter()
            .chain(input.draft_response.clone())
            .collect();
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
            weight: 0.40,
        });
        Some(rep)
    } else {
        None
    };

    // 2. Plan DAG Verification (Weight: 0.20)
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
            weight: 0.20,
        });
        Some(metrics)
    } else {
        None
    };

    // 3. Text & Information Theory Evaluation (Weight: 0.15)
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
        if conf.verdict == "LOW_CONFIDENCE" {
            recommendations.push(
                "Low Confidence Warning: Draft text displays high hedging or lack of specificity."
                    .to_string(),
            );
        }
        for c in &conf.self_contradictions {
            critical_violations.push(format!("Self-Contradiction in Response: {}", c));
        }

        let text_quality = if rep.is_verbose || rep.is_too_complex {
            60.0
        } else {
            95.0
        };
        weighted_scores.push(WeightedScore {
            score: text_quality,
            weight: 0.15,
        });
        (Some(rep), Some(conf))
    } else {
        (None, None)
    };

    // 4. Code Metrics Evaluation (Weight: 0.25)
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
            weight: 0.25,
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
            "code": code_report,
        }),
        critical_violations,
        recommendations,
    };

    serde_json::to_value(report).map_err(|e| e.to_string())
}
