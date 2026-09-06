use crate::engine::{
    CodeAnalyzer, ConfidenceAnalyzer, ConstraintEngine, EvidenceClassifier, ForesightEngine, PlanDag,
    ResearchGate, TextEvaluator,
    receipts::{EvidenceReceipt, ExecutionReceipt, ReceiptsVerificationSummary},
    resource_limits::*,
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
    /// Explicit audit phase: "plan" (validate plan structure only, no execution coverage required)
    /// or "execution" (validate plan + execution coverage). When omitted, auto-detected:
    /// if executed_steps is empty → "plan", otherwise → "execution".
    #[serde(default)]
    pub audit_phase: Option<String>,
    /// Cryptographic / tool execution receipts validating executed steps
    #[serde(default)]
    pub execution_receipts: Option<Vec<ExecutionReceipt>>,
    /// Artifact evidence receipts verifying research and files
    #[serde(default)]
    pub evidence_receipts: Option<Vec<EvidenceReceipt>>,
    /// Minimum required policy mode ("standard" or "deep"). Prevents caller from forcing 'quick' mode.
    #[serde(default)]
    pub min_policy_mode: Option<String>,
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
    pub diagnostic_score: f64,
    #[serde(default)]
    pub composite_score: f64,
    #[serde(default)]
    pub audit_phase: String,
    #[serde(default)]
    pub is_delivery_authorized: bool,
    pub severity_summary: SeveritySummary,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipts_summary: Option<ReceiptsVerificationSummary>,
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

// ─── Resource limits ──────────────────────────────────────────────────────────
const MAX_TASKS: usize = 200;
const MAX_REQUIREMENTS: usize = 200;
const MAX_EXECUTED_STEPS: usize = 500;
const MAX_CODE_BYTES: usize = 512 * 1024; // 512 KB
const MAX_TEXT_BYTES: usize = 64 * 1024; // 64 KB
const MAX_TASK_ID_LEN: usize = 64;
const MAX_TASK_NAME_LEN: usize = 256;

pub fn execute_unified_audit(args: Value) -> Result<Value, String> {
    let input: UnifiedAuditInput =
        serde_json::from_value(args).map_err(|e| format!("Invalid arguments for math_audit_cognition: {}", e))?;

    // ── Mode validation (reject unknown modes immediately) ────────────────────
    let mode_str = input.mode.as_deref().unwrap_or("standard").to_lowercase();
    if !["quick", "standard", "deep"].contains(&mode_str.as_str()) {
        return Err(format!(
            "Invalid mode '{}'. Accepted values: quick, standard, deep.",
            mode_str
        ));
    }
    let is_quick = mode_str == "quick";
    let is_deep = mode_str == "deep";

    // ── Policy mode requirement check ─────────────────────────────────────────
    if let Some(ref min_mode) = input.min_policy_mode {
        let min_lower = min_mode.to_lowercase();
        if (min_lower == "standard" || min_lower == "deep") && is_quick {
            return Err(format!(
                "Policy constraint violated: caller requested mode='quick', but min_policy_mode requires '{}'",
                min_mode
            ));
        }
        if min_lower == "deep" && !is_deep {
            return Err(format!(
                "Policy constraint violated: caller requested mode='{}', but min_policy_mode requires 'deep'",
                mode_str
            ));
        }
    }

    // ── Resource limit guards (DoS / resource exhaustion prevention) ──────────
    if input.planned_tasks.len() > MAX_TASKS {
        return Err(format!(
            "Resource limit exceeded: planned_tasks count {} > MAX_TASKS {}",
            input.planned_tasks.len(),
            MAX_TASKS
        ));
    }
    if input.user_requirements.len() > MAX_REQUIREMENTS {
        return Err(format!(
            "Resource limit exceeded: user_requirements count {} > MAX_REQUIREMENTS {}",
            input.user_requirements.len(),
            MAX_REQUIREMENTS
        ));
    }
    if input.executed_steps.len() > MAX_EXECUTED_STEPS {
        return Err(format!(
            "Resource limit exceeded: executed_steps count {} > MAX_EXECUTED_STEPS {}",
            input.executed_steps.len(),
            MAX_EXECUTED_STEPS
        ));
    }
    if let Some(ref receipts) = input.execution_receipts {
        if receipts.len() > MAX_EXECUTED_STEPS {
            return Err(format!(
                "Resource limit exceeded: execution_receipts count {} > MAX_EXECUTED_STEPS {}",
                receipts.len(),
                MAX_EXECUTED_STEPS
            ));
        }
    }
    if let Some(ref code) = input.code_snippet {
        if code.len() > MAX_CODE_BYTES {
            return Err(format!(
                "Resource limit exceeded: code_snippet size {} bytes > MAX_CODE_BYTES {}",
                code.len(),
                MAX_CODE_BYTES
            ));
        }
    }
    if let Some(ref text) = input.draft_response {
        if text.len() > MAX_TEXT_BYTES {
            return Err(format!(
                "Resource limit exceeded: draft_response size {} bytes > MAX_TEXT_BYTES {}",
                text.len(),
                MAX_TEXT_BYTES
            ));
        }
    }

    // ── Audit phase detection ─────────────────────────────────────────────────
    // "plan" = validate plan structure only; coverage_ratio not penalized.
    // "execution" = validate plan + execution coverage.
    // Auto-detect: no executed_steps → plan phase.
    let resolved_phase = match input.audit_phase.as_deref() {
        Some("plan") => "plan",
        Some("execution") => "execution",
        Some(other) => {
            return Err(format!(
                "Invalid audit_phase '{}'. Accepted values: plan, execution.",
                other
            ));
        }
        None => {
            if input.executed_steps.is_empty() && !input.planned_tasks.is_empty() {
                "plan"
            } else {
                "execution"
            }
        }
    };
    let is_plan_phase = resolved_phase == "plan";

    let mut violations: Vec<AuditViolation> = Vec::new();
    let mut critical_violations = Vec::new();
    let mut recommendations = Vec::new();
    let mut weighted_scores: Vec<WeightedScore> = Vec::new();

    // ── Phase Spoofing Invariants (Anti-Evasion Gate) ──────────────────────────
    if is_plan_phase {
        // Invariant 1: Plan phase must not contain an execution trace
        if !input.executed_steps.is_empty() {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "PHASE_SPOOFING",
                format!(
                    "Phase Invariant Violation: 'executed_steps' ({}) supplied under audit_phase='plan'. Use audit_phase='execution' to evaluate execution trace.",
                    input.executed_steps.len()
                ),
                ViolationSeverity::Critical,
                "Set audit_phase to 'execution' when auditing executed tasks, or remove executed_steps for plan-only audit.",
            );
        }

        // Invariant 2: Plan phase must not be used to deliver implementation code
        if input.code_snippet.is_some() {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "PHASE_SPOOFING",
                "Phase Invariant Violation: 'code_snippet' supplied under audit_phase='plan'. Implementation code requires execution phase verification to measure coverage and regression risk.".to_string(),
                ViolationSeverity::Critical,
                "Switch audit_phase to 'execution' to audit implementation code, or remove code_snippet for plan-only audit.",
            );
        }

        // Invariant 3: Draft response must not claim delivered completion under plan phase
        if let Some(ref text) = input.draft_response {
            let lower_text = text.to_lowercase();
            const COMPLETION_MARKERS: &[&str] = &[
                "i have implemented",
                "here is the implementation",
                "successfully executed",
                "i have completed",
                "all tasks are complete",
                "task is completed",
                "implementation is complete",
                "đã hoàn thành",
                "đã thực hiện xong",
                "đã cài đặt xong",
                "đây là code hoàn chỉnh",
                "the solution is ready",
            ];
            if COMPLETION_MARKERS.iter().any(|&m| lower_text.contains(m)) {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "PHASE_SPOOFING",
                    "Phase Invariant Violation: draft_response claims completed delivery under audit_phase='plan'. A plan audit cannot authorize final delivery.".to_string(),
                    ViolationSeverity::Critical,
                    "Switch audit_phase to 'execution' and provide executed_steps before delivering final response.",
                );
            }
        }
    }

    // Deep Mode Invariants: In deep governance mode, explicit requirements and plan DAG are mandatory
    if is_deep {
        if input.user_requirements.is_empty() {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "REQUIREMENTS_EVIDENCE_MISSING",
                "Deep Audit Invariant: Formal user requirements must be supplied in deep audit mode.".to_string(),
                ViolationSeverity::Critical,
                "Provide user_requirements array defining the target system contracts.",
            );
        }

        if input.planned_tasks.is_empty() {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "PLAN_EVIDENCE_MISSING",
                "Deep Audit Invariant: Formal plan DAG tasks must be supplied in deep audit mode.".to_string(),
                ViolationSeverity::Critical,
                "Provide planned_tasks array specifying the DAG execution graph.",
            );
        }
    }

    // 1. Constraint Verification (Standard: 0.30, Quick: 0.50)
    let constraint_report = if !input.user_requirements.is_empty() {
        let mut impl_claims: Vec<String> = Vec::new();
        let known_ids: std::collections::HashSet<&str> = input.planned_tasks.iter().map(|t| t.id.as_str()).collect();

        for task in &input.planned_tasks {
            if is_plan_phase || input.executed_steps.contains(&task.id) {
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
        // ── Pre-validate task fields before inserting into DAG ────────────────
        let mut seen_ids: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for t in &input.planned_tasks {
            let id_trimmed = t.id.trim();
            let name_trimmed = t.name.trim();

            if id_trimmed.is_empty() {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "INVALID_TASK_ID",
                    "Schema Violation: A task has an empty or whitespace-only ID.".to_string(),
                    ViolationSeverity::Critical,
                    "Assign a non-empty, unique identifier to every task in the plan.",
                );
                continue;
            }
            if id_trimmed.len() > MAX_TASK_ID_LEN {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "INVALID_TASK_ID",
                    format!(
                        "Schema Violation: Task ID '{}' exceeds max length {} chars.",
                        id_trimmed, MAX_TASK_ID_LEN
                    ),
                    ViolationSeverity::Critical,
                    "Shorten task IDs to 64 characters or fewer.",
                );
                continue;
            }
            if name_trimmed.is_empty() {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "INVALID_TASK_NAME",
                    format!(
                        "Schema Violation: Task '{}' has an empty or whitespace-only name.",
                        id_trimmed
                    ),
                    ViolationSeverity::Critical,
                    "Assign a non-empty description to every task.",
                );
                continue;
            }
            if name_trimmed.len() > MAX_TASK_NAME_LEN {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "INVALID_TASK_NAME",
                    format!(
                        "Schema Violation: Task '{}' name exceeds max length {} chars.",
                        id_trimmed, MAX_TASK_NAME_LEN
                    ),
                    ViolationSeverity::Critical,
                    "Shorten task names to 256 characters or fewer.",
                );
                continue;
            }
            if !seen_ids.insert(id_trimmed) {
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "DUPLICATE_TASK_ID",
                    format!(
                        "Schema Violation: Duplicate task ID '{}' detected in plan DAG.",
                        id_trimmed
                    ),
                    ViolationSeverity::Critical,
                    "Every task must have a unique ID. Remove or rename the duplicate.",
                );
            }
        }

        let mut dag = PlanDag::new();
        // Build a set of IDs that appeared more than once — exclude them from the DAG
        // to avoid silently overwriting the first definition with the second.
        let mut id_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for t in &input.planned_tasks {
            let id_trimmed = t.id.trim();
            if !id_trimmed.is_empty() {
                *id_counts.entry(id_trimmed).or_insert(0) += 1;
            }
        }
        for t in &input.planned_tasks {
            let id_trimmed = t.id.trim();
            let name_trimmed = t.name.trim();
            // Only insert unique, non-empty, valid-length tasks
            if !id_trimmed.is_empty()
                && !name_trimmed.is_empty()
                && id_trimmed.len() <= MAX_TASK_ID_LEN
                && name_trimmed.len() <= MAX_TASK_NAME_LEN
                && id_counts.get(id_trimmed).copied().unwrap_or(0) == 1
            {
                dag.add_task(id_trimmed, name_trimmed, t.dependencies.clone());
            }
        }

        // ── Structural validation: unknown deps + cycles ──────────────────────
        if let Err(err) = dag.validate_graph() {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "DAG_STRUCTURAL_ERROR",
                format!("DAG Structural Violation: {}", err),
                ViolationSeverity::Critical,
                "Fix cycle or unknown dependency reference in planned_tasks before executing.",
            );
        }

        for step in &input.executed_steps {
            if let Err(err) = dag.record_step(step) {
                if err.contains("Dependency violation") || err.contains("Unknown dependency") {
                    add_violation(
                        &mut violations,
                        &mut critical_violations,
                        &mut recommendations,
                        "PLAN_DEPENDENCY_ERROR",
                        err,
                        ViolationSeverity::Critical,
                        "Reorder tasks according to DAG topological dependencies.",
                    );
                } else if PlanDag::is_mutation_action(step) {
                    add_violation(
                        &mut violations,
                        &mut critical_violations,
                        &mut recommendations,
                        "UNAPPROVED_MUTATION_SCOPE_CREEP",
                        format!(
                            "Critical Scope Creep: Unplanned mutating action '{}' executed outside approved plan DAG.",
                            step
                        ),
                        ViolationSeverity::Critical,
                        "All state-mutating actions must be formally specified and approved in the plan DAG prior to execution.",
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
        if metrics.scope_creep_count > 0 && !violations.iter().any(|v| v.code == "SCOPE_CREEP" || v.code == "UNAPPROVED_MUTATION_SCOPE_CREEP") {
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

        // Coverage check only applies in execution phase (not plan-only phase)
        if !is_plan_phase && is_deep && metrics.coverage_ratio < 1.0 && !input.executed_steps.is_empty() {
            add_violation(
                &mut violations,
                &mut critical_violations,
                &mut recommendations,
                "PLAN_COVERAGE_DEFICIT",
                format!(
                    "Deep Audit Invariant: Plan execution incomplete (coverage = {:.1}%). Deep governance requires 100% DAG coverage.",
                    metrics.coverage_ratio * 100.0
                ),
                ViolationSeverity::Critical,
                "Complete all planned tasks in DAG before final delivery.",
            );
        }

        let weight = if is_quick { 0.25 } else { 0.15 };
        // In plan phase with no executed steps, coverage = 1.0 (neutral — not penalized).
        let dag_score = if is_plan_phase && input.executed_steps.is_empty() {
            100.0
        } else {
            metrics.coverage_ratio * 100.0
        };
        weighted_scores.push(WeightedScore {
            score: dag_score,
            weight,
        });
        Some(metrics)
    } else {
        None
    };

    // 2.1 Execution Receipts Verification (Provenance Layer)
    let receipts_summary = if !is_plan_phase && !input.executed_steps.is_empty() {
        if let Some(ref receipts) = input.execution_receipts {
            let total_receipts = receipts.len();
            let mut matched_count = 0;
            let mut failed_count = 0;
            let mut unattested_steps = Vec::new();

            for step in &input.executed_steps {
                let matching: Vec<&ExecutionReceipt> = receipts.iter().filter(|r| r.action_id == *step).collect();
                if matching.is_empty() {
                    unattested_steps.push(step.clone());
                    if is_deep {
                        add_violation(
                            &mut violations,
                            &mut critical_violations,
                            &mut recommendations,
                            "UNATTESTED_EXECUTION_CLAIM",
                            format!(
                                "Deep Mode Invariant: Executed step '{}' has no cryptographic/machine-verifiable execution receipt.",
                                step
                            ),
                            ViolationSeverity::Critical,
                            "Provide an ExecutionReceipt with tool output hash or return code for every executed step.",
                        );
                    } else {
                        add_violation(
                            &mut violations,
                            &mut critical_violations,
                            &mut recommendations,
                            "UNATTESTED_EXECUTION_CLAIM",
                            format!(
                                "Executed step '{}' is missing execution receipt verification.",
                                step
                            ),
                            ViolationSeverity::Warning,
                            "Provide ExecutionReceipts to verify execution integrity.",
                        );
                    }
                } else {
                    for r in matching {
                        if let Some(exit_code) = r.exit_code {
                            if exit_code != 0 {
                                failed_count += 1;
                                add_violation(
                                    &mut violations,
                                    &mut critical_violations,
                                    &mut recommendations,
                                    "FAILED_EXECUTION_RECEIPT",
                                    format!(
                                        "Execution Receipt for '{}' reported failure exit_code {}.",
                                        step, exit_code
                                    ),
                                    ViolationSeverity::Critical,
                                    "Ensure all executed tools and commands complete with exit code 0.",
                                );
                            }
                        }
                    }
                    matched_count += 1;
                }
            }

            Some(ReceiptsVerificationSummary {
                total_receipts,
                matched_steps_count: matched_count,
                unattested_steps,
                failed_receipts_count: failed_count,
                has_full_provenance: failed_count == 0 && matched_count == input.executed_steps.len(),
            })
        } else {
            None
        }
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
                    "Verbosity Warning: Information density is low with high token redundancy. Condense response."
                        .to_string(),
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
                let ungrounded_count = r_rep.unverified_claims.len();
                add_violation(
                    &mut violations,
                    &mut critical_violations,
                    &mut recommendations,
                    "RESEARCH_DEFICIT",
                    format!(
                        "Research Deficit: {} factual technical claim(s) lack verified citations (RFC/docs/test logs): {:?}",
                        ungrounded_count, r_rep.unverified_claims
                    ),
                    ViolationSeverity::Critical,
                    "Ground every factual claim with official documentation links, RFCs, or benchmark citations.",
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
        && (input.draft_response.is_some() || input.code_snippet.is_some() || !input.planned_tasks.is_empty())
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

    let mandatory_contract_met = match mode_str.as_str() {
        "quick" => has_any_input,
        "deep" => !input.user_requirements.is_empty() && !input.planned_tasks.is_empty(),
        _ => !input.user_requirements.is_empty() || !input.planned_tasks.is_empty(),
    };

    if !is_quick && !mandatory_contract_met && has_any_input {
        add_violation(
            &mut violations,
            &mut critical_violations,
            &mut recommendations,
            "CONTRACT_EVIDENCE_MISSING",
            format!(
                "{} Mode Invariant: Verification requires formal user_requirements or planned_tasks. Isolated response alone cannot receive ALLOW.",
                if is_deep { "Deep" } else { "Standard" }
            ),
            if is_deep {
                ViolationSeverity::Critical
            } else {
                ViolationSeverity::Warning
            },
            "Provide user_requirements or planned_tasks to substantiate task compliance.",
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

    let (decision, verdict) = if critical_count > 0 {
        ("BLOCK".to_string(), "FAIL".to_string())
    } else if !has_any_input || weighted_scores.is_empty() || !mandatory_contract_met {
        ("INSUFFICIENT_EVIDENCE".to_string(), "UNVERIFIED".to_string())
    } else if policy_score < 50.0 {
        ("BLOCK".to_string(), "FAIL".to_string())
    } else if warning_count > 0 || policy_score < 75.0 {
        ("WARN".to_string(), "WARN".to_string())
    } else if is_plan_phase {
        ("ALLOW".to_string(), "PLAN_APPROVED".to_string())
    } else if is_quick {
        // QUICK MODE CAN NEVER AUTHORIZE DELIVERY
        ("CHECKPOINT_PASS".to_string(), "QUICK_PASS".to_string())
    } else {
        ("ALLOW".to_string(), "PASS".to_string())
    };

    let is_delivery_authorized = decision == "ALLOW" && !is_plan_phase && !is_quick;

    let mut remediation_plan: Vec<String> = Vec::new();
    for v in &violations {
        if !remediation_plan.contains(&v.remediation) {
            remediation_plan.push(v.remediation.clone());
        }
    }

    let diagnostic_score = policy_score;
    let composite_score = policy_score;

    let report = UnifiedAuditReport {
        decision,
        verdict,
        policy_score,
        diagnostic_score,
        composite_score,
        audit_phase: resolved_phase.to_string(),
        is_delivery_authorized,
        severity_summary,
        receipts_summary: receipts_summary.clone(),
        math_breakdown: json!({
            "mode": mode_str,
            "audit_phase": resolved_phase,
            "is_delivery_authorized": is_delivery_authorized,
            "constraints": constraint_report,
            "dag": dag_report,
            "receipts": receipts_summary,
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
        assert_eq!(val["decision"], "CHECKPOINT_PASS");
        assert_eq!(val["verdict"], "QUICK_PASS");
        assert_eq!(val["is_delivery_authorized"], false);
        assert_eq!(val["math_breakdown"]["mode"], "quick");
        assert!(val["math_breakdown"]["code"].is_null());
        assert!(val["math_breakdown"]["research"].is_null());
    }

    #[test]
    fn test_unified_audit_min_policy_mode_rejection() {
        let args = json!({
            "mode": "quick",
            "min_policy_mode": "standard",
            "user_requirements": ["test requirement"],
        });
        let result = execute_unified_audit(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Policy constraint violated"));
    }

    #[test]
    fn test_unified_audit_unapproved_mutation_is_critical_block() {
        let args = json!({
            "user_requirements": ["implement feature"],
            "planned_tasks": [
                {"id": "t1", "name": "implement feature", "dependencies": []}
            ],
            "executed_steps": ["t1", "delete_production_database"],
            "draft_response": "According to docs.rs, the implementation is complete: https://docs.rs/example"
        });
        let result = execute_unified_audit(args);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["decision"], "BLOCK");
        assert_eq!(val["verdict"], "FAIL");
        let violations = val["violations"].as_array().unwrap();
        assert!(violations.iter().any(|v| v["code"] == "UNAPPROVED_MUTATION_SCOPE_CREEP"));
    }

    #[test]
    fn test_unified_audit_receipts_validation() {
        let args = json!({
            "user_requirements": ["implement feature"],
            "planned_tasks": [
                {"id": "t1", "name": "implement feature", "dependencies": []}
            ],
            "executed_steps": ["t1"],
            "execution_receipts": [
                {
                    "action_id": "t1",
                    "tool_name": "cargo_build",
                    "exit_code": 0
                }
            ],
            "draft_response": "According to docs.rs and RFC 1234, implementation is complete. See: https://docs.rs/example",
            "code_snippet": "fn feature() -> bool { true }",
            "language": "rust"
        });
        let result = execute_unified_audit(args);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["decision"], "ALLOW");
        assert_eq!(val["verdict"], "PASS");
        assert_eq!(val["is_delivery_authorized"], true);
        assert_eq!(val["receipts_summary"]["matched_steps_count"], 1);
        assert_eq!(val["receipts_summary"]["failed_receipts_count"], 0);
    }

    #[test]
    fn test_unified_audit_receipts_failure_exit_code() {
        let args = json!({
            "user_requirements": ["implement feature"],
            "planned_tasks": [
                {"id": "t1", "name": "implement feature", "dependencies": []}
            ],
            "executed_steps": ["t1"],
            "execution_receipts": [
                {
                    "action_id": "t1",
                    "tool_name": "cargo_test",
                    "exit_code": 101
                }
            ],
            "draft_response": "According to docs.rs, implementation is complete. See: https://docs.rs/example",
        });
        let result = execute_unified_audit(args);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["decision"], "BLOCK");
        let violations = val["violations"].as_array().unwrap();
        assert!(violations.iter().any(|v| v["code"] == "FAILED_EXECUTION_RECEIPT"));
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
    fn test_unified_audit_score_gaming_rejected() {
        let args = json!({
            "draft_response": "According to docs.rs and RFC 2119, everything is implemented with high confidence and verified citations. See: https://docs.rs/serde"
        });
        let result = execute_unified_audit(args);
        assert!(result.is_ok());
        let val = result.unwrap();
        // Standard mode invariant: Isolated draft response cannot receive ALLOW
        assert_eq!(val["decision"], "INSUFFICIENT_EVIDENCE");
        assert_eq!(val["verdict"], "UNVERIFIED");
        let violations = val["violations"].as_array().unwrap();
        assert!(violations.iter().any(|v| v["code"] == "CONTRACT_EVIDENCE_MISSING"));
    }

    #[test]
    fn test_unified_audit_invalid_json() {
        let args = json!("not an object");
        let result = execute_unified_audit(args);
        assert!(result.is_err());
    }
}
