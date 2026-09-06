use ai_verification_mcp::tools::unified_audit::execute_unified_audit;
use serde_json::json;

/// Security Invariants Test Suite
///
/// These tests verify properties that MUST NEVER be violated regardless of input.
/// Invariant numbering matches the design review document.

// ─── Invariant 1: Empty input NEVER produces ALLOW ───────────────────────────

#[test]
fn invariant_1_empty_input_never_allow() {
    let res = execute_unified_audit(json!({})).expect("audit must not error on empty");
    let decision = res["decision"].as_str().unwrap();
    assert_ne!(
        decision, "ALLOW",
        "Invariant 1 VIOLATED: Empty input must never receive ALLOW, got: {}",
        decision
    );
}

// ─── Invariant 2: Unknown dependency NEVER produces ALLOW ────────────────────

#[test]
fn invariant_2_unknown_dependency_never_allow() {
    let res = execute_unified_audit(json!({
        "planned_tasks": [
            {"id": "t2", "name": "deploy to production", "dependencies": ["security_review"]}
        ],
        "executed_steps": ["t2"]
    }))
    .expect("audit must not error");
    let decision = res["decision"].as_str().unwrap();
    assert_eq!(
        decision, "BLOCK",
        "Invariant 2 VIOLATED: Unknown dependency must produce BLOCK, got: {}",
        decision
    );
    let violations = res["violations"].as_array().unwrap();
    assert!(
        violations.iter().any(|v| v["code"] == "DAG_STRUCTURAL_ERROR"),
        "Invariant 2: DAG_STRUCTURAL_ERROR violation must be present"
    );
}

#[test]
fn invariant_2b_unknown_dependency_plan_only_blocked() {
    let res = execute_unified_audit(json!({
        "planned_tasks": [
            {"id": "t1", "name": "seed db", "dependencies": ["build_database"]}
        ]
    }))
    .expect("audit must not error");
    let decision = res["decision"].as_str().unwrap();
    assert_eq!(
        decision, "BLOCK",
        "Invariant 2b VIOLATED: Plan-only unknown dependency must BLOCK, got: {}",
        decision
    );
}

// ─── Invariant 3: Cycle in DAG NEVER produces ALLOW ─────────────────────────

#[test]
fn invariant_3_cycle_never_allow() {
    let res = execute_unified_audit(json!({
        "planned_tasks": [
            {"id": "a", "name": "step a", "dependencies": ["b"]},
            {"id": "b", "name": "step b", "dependencies": ["a"]}
        ],
        "executed_steps": ["a", "b"]
    }))
    .expect("audit must not error");
    let decision = res["decision"].as_str().unwrap();
    assert_eq!(
        decision, "BLOCK",
        "Invariant 3 VIOLATED: Cyclic DAG must produce BLOCK, got: {}",
        decision
    );
}

#[test]
fn invariant_3b_cycle_without_steps_blocked() {
    let res = execute_unified_audit(json!({
        "planned_tasks": [
            {"id": "x", "name": "task x", "dependencies": ["y"]},
            {"id": "y", "name": "task y", "dependencies": ["x"]}
        ]
    }))
    .expect("audit must not error");
    let decision = res["decision"].as_str().unwrap();
    assert_eq!(
        decision, "BLOCK",
        "Invariant 3b VIOLATED: Cycle without steps must BLOCK, got: {}",
        decision
    );
}

// ─── Invariant 4: Duplicate task ID NEVER produces ALLOW ─────────────────────

#[test]
fn invariant_4_duplicate_task_id_never_allow() {
    let res = execute_unified_audit(json!({
        "planned_tasks": [
            {"id": "t1", "name": "security review", "dependencies": []},
            {"id": "t1", "name": "deploy to prod", "dependencies": []}
        ],
        "executed_steps": ["t1"]
    }))
    .expect("audit must not error");
    let decision = res["decision"].as_str().unwrap();
    assert_eq!(
        decision, "BLOCK",
        "Invariant 4 VIOLATED: Duplicate task ID must produce BLOCK, got: {}",
        decision
    );
    let violations = res["violations"].as_array().unwrap();
    assert!(
        violations.iter().any(|v| v["code"] == "DUPLICATE_TASK_ID"),
        "Invariant 4: DUPLICATE_TASK_ID violation must be present"
    );
}

// ─── Invariant 5: Empty/whitespace task ID NEVER produces ALLOW ──────────────

#[test]
fn invariant_5_empty_task_id_never_allow() {
    let res = execute_unified_audit(json!({
        "planned_tasks": [
            {"id": "", "name": "anonymous task", "dependencies": []},
            {"id": "   ", "name": "whitespace task", "dependencies": []}
        ]
    }))
    .expect("audit must not error");
    let decision = res["decision"].as_str().unwrap();
    assert_eq!(
        decision, "BLOCK",
        "Invariant 5 VIOLATED: Empty task ID must produce BLOCK, got: {}",
        decision
    );
    let violations = res["violations"].as_array().unwrap();
    assert!(
        violations.iter().any(|v| v["code"] == "INVALID_TASK_ID"),
        "Invariant 5: INVALID_TASK_ID violation must be present"
    );
}

// ─── Invariant 13: Unknown mode is REJECTED with Err ─────────────────────────

#[test]
fn invariant_13_unknown_mode_rejected() {
    let res = execute_unified_audit(json!({
        "mode": "MAXIMUM_SECURITY",
        "planned_tasks": [{"id": "t1", "name": "do task", "dependencies": []}]
    }));
    assert!(
        res.is_err(),
        "Invariant 13 VIOLATED: Unknown mode must return Err, not Ok({:?})",
        res.ok()
    );
    let err = res.unwrap_err();
    assert!(
        err.contains("Invalid mode"),
        "Invariant 13: Error must mention 'Invalid mode', got: {}",
        err
    );
}

#[test]
fn invariant_13b_invalid_audit_phase_rejected() {
    let res = execute_unified_audit(json!({
        "audit_phase": "ultra_deep",
        "planned_tasks": [{"id": "t1", "name": "do task", "dependencies": []}]
    }));
    assert!(
        res.is_err(),
        "Invariant 13b VIOLATED: Unknown audit_phase must return Err"
    );
}

// ─── Plan phase: zero coverage is NOT penalized ──────────────────────────────

#[test]
fn invariant_plan_phase_no_coverage_penalty() {
    let res = execute_unified_audit(json!({
        "user_requirements": ["encrypt data", "add auth"],
        "planned_tasks": [
            {"id": "t1", "name": "encrypt data", "dependencies": []},
            {"id": "t2", "name": "add auth", "dependencies": ["t1"]}
        ],
        "audit_phase": "plan"
    }))
    .expect("audit must not error");
    let decision = res["decision"].as_str().unwrap();
    assert_ne!(
        decision, "BLOCK",
        "Plan phase VIOLATED: Valid plan-only audit must not BLOCK due to zero coverage"
    );

    // Auto-detect: no executed_steps → plan phase
    let res2 = execute_unified_audit(json!({
        "user_requirements": ["encrypt data", "add auth"],
        "planned_tasks": [
            {"id": "t1", "name": "encrypt data", "dependencies": []},
            {"id": "t2", "name": "add auth", "dependencies": ["t1"]}
        ]
    }))
    .expect("audit must not error");
    assert_eq!(
        res2["math_breakdown"]["audit_phase"].as_str().unwrap(),
        "plan",
        "Auto-detect must resolve to 'plan' when executed_steps is absent"
    );
    assert_ne!(
        res2["decision"].as_str().unwrap(),
        "BLOCK",
        "Auto-detected plan phase must not BLOCK due to zero coverage"
    );
}

// ─── Invariant 18: Removing evidence cannot improve verdict ──────────────────

#[test]
fn invariant_18_removing_evidence_cannot_improve_verdict() {
    let payload_a = json!({
        "user_requirements": ["implement feature"],
        "planned_tasks": [{"id": "t1", "name": "implement feature", "dependencies": []}],
        "executed_steps": ["t1"],
        "draft_response": "Implemented per RFC 2119. See https://docs.rs/serde."
    });
    let payload_b = json!({
        "user_requirements": ["implement feature"],
        "planned_tasks": [{"id": "t1", "name": "implement feature", "dependencies": []}],
        "executed_steps": ["t1"]
    });

    let res_a = execute_unified_audit(payload_a).expect("A must not error");
    let res_b = execute_unified_audit(payload_b).expect("B must not error");
    let score_a = res_a["policy_score"].as_f64().unwrap_or(0.0);
    let score_b = res_b["policy_score"].as_f64().unwrap_or(0.0);
    assert!(
        score_b <= score_a + 1.0,
        "Invariant 18 VIOLATED: Removing evidence improved score {:.1} → {:.1}",
        score_a,
        score_b
    );

    let rank = |d: &str| match d {
        "ALLOW" => 3,
        "WARN" => 2,
        "BLOCK" => 1,
        _ => 0,
    };
    let dec_a = res_a["decision"].as_str().unwrap();
    let dec_b = res_b["decision"].as_str().unwrap();
    assert!(
        rank(dec_b) <= rank(dec_a),
        "Invariant 18 VIOLATED: Removing evidence improved decision {} → {}",
        dec_a,
        dec_b
    );
}

// ─── Invariant 19: Adding critical violation cannot improve verdict ───────────

#[test]
fn invariant_19_adding_critical_violation_cannot_improve_verdict() {
    let baseline = json!({
        "user_requirements": ["implement auth"],
        "planned_tasks": [{"id": "t1", "name": "implement auth", "dependencies": []}],
        "executed_steps": ["t1"],
        "draft_response": "Auth implemented. See https://docs.rs/serde."
    });
    let with_violation = json!({
        "user_requirements": ["implement auth"],
        "planned_tasks": [
            {"id": "t1", "name": "implement auth", "dependencies": ["nonexistent"]}
        ],
        "executed_steps": ["t1"],
        "draft_response": "Auth implemented. See https://docs.rs/serde."
    });

    let res_base = execute_unified_audit(baseline).expect("baseline must not error");
    let res_viol = execute_unified_audit(with_violation).expect("violation must not error");
    let dec_base = res_base["decision"].as_str().unwrap();
    let dec_viol = res_viol["decision"].as_str().unwrap();
    let rank = |d: &str| match d {
        "ALLOW" => 3,
        "WARN" => 2,
        "BLOCK" => 1,
        _ => 0,
    };
    assert!(
        rank(dec_viol) <= rank(dec_base),
        "Invariant 19 VIOLATED: Adding critical violation improved decision {} → {}",
        dec_base,
        dec_viol
    );
    assert_eq!(
        dec_viol, "BLOCK",
        "Invariant 19: DAG structural error must produce BLOCK"
    );
}

// ─── Resource limit invariants ────────────────────────────────────────────────

#[test]
fn invariant_resource_limit_tasks_rejected() {
    let tasks: Vec<_> = (0..201)
        .map(|i| json!({"id": format!("t{}", i), "name": format!("task {}", i), "dependencies": []}))
        .collect();
    let res = execute_unified_audit(json!({"planned_tasks": tasks}));
    assert!(res.is_err(), "Resource limit: 201 tasks must be rejected with Err");
    assert!(
        res.unwrap_err().contains("Resource limit exceeded"),
        "Error must mention 'Resource limit exceeded'"
    );
}

#[test]
fn invariant_resource_limit_requirements_rejected() {
    let reqs: Vec<_> = (0..201).map(|i| format!("requirement {}", i)).collect();
    let res = execute_unified_audit(json!({"user_requirements": reqs}));
    assert!(res.is_err(), "Resource limit: 201 requirements must be rejected");
}

// ─── Anti-Phase Spoofing Invariants ──────────────────────────────────────────

#[test]
fn invariant_phase_spoofing_executed_steps_blocked() {
    // Attempting to pass executed_steps under audit_phase='plan' to bypass execution coverage
    let res = execute_unified_audit(json!({
        "audit_phase": "plan",
        "user_requirements": ["implement feature"],
        "planned_tasks": [{"id": "t1", "name": "implement feature", "dependencies": []}],
        "executed_steps": ["t1"]
    }))
    .expect("audit must evaluate");

    assert_eq!(
        res["decision"].as_str().unwrap(),
        "BLOCK",
        "Phase Spoofing VIOLATION: executed_steps under audit_phase='plan' must produce BLOCK"
    );
    let violations = res["violations"].as_array().unwrap();
    assert!(
        violations.iter().any(|v| v["code"] == "PHASE_SPOOFING"),
        "PHASE_SPOOFING violation must be present"
    );
}

#[test]
fn invariant_phase_spoofing_code_snippet_blocked() {
    // Attempting to deliver code under audit_phase='plan' to bypass code coverage/regression checks
    let res = execute_unified_audit(json!({
        "audit_phase": "plan",
        "user_requirements": ["implement feature"],
        "planned_tasks": [{"id": "t1", "name": "implement feature", "dependencies": []}],
        "code_snippet": "fn bypass_execution_check() -> bool { true }"
    }))
    .expect("audit must evaluate");

    assert_eq!(
        res["decision"].as_str().unwrap(),
        "BLOCK",
        "Phase Spoofing VIOLATION: code_snippet under audit_phase='plan' must produce BLOCK"
    );
    let violations = res["violations"].as_array().unwrap();
    assert!(
        violations.iter().any(|v| v["code"] == "PHASE_SPOOFING"),
        "PHASE_SPOOFING violation must be present"
    );
}

#[test]
fn invariant_phase_spoofing_completion_claim_blocked() {
    // Attempting to claim final completion in draft_response while using audit_phase='plan'
    let res = execute_unified_audit(json!({
        "audit_phase": "plan",
        "user_requirements": ["implement feature"],
        "planned_tasks": [{"id": "t1", "name": "implement feature", "dependencies": []}],
        "draft_response": "I have implemented all requirements and the solution is ready for delivery."
    }))
    .expect("audit must evaluate");

    assert_eq!(
        res["decision"].as_str().unwrap(),
        "BLOCK",
        "Phase Spoofing VIOLATION: completion claim in draft_response under audit_phase='plan' must produce BLOCK"
    );
    let violations = res["violations"].as_array().unwrap();
    assert!(
        violations.iter().any(|v| v["code"] == "PHASE_SPOOFING"),
        "PHASE_SPOOFING violation must be present"
    );
}

#[test]
fn invariant_plan_phase_never_authorizes_delivery() {
    // A clean, valid plan receives ALLOW, but verdict is PLAN_APPROVED and delivery is NOT authorized
    let res = execute_unified_audit(json!({
        "audit_phase": "plan",
        "user_requirements": ["encrypt data", "add auth"],
        "planned_tasks": [
            {"id": "t1", "name": "encrypt data", "dependencies": []},
            {"id": "t2", "name": "add auth", "dependencies": ["t1"]}
        ]
    }))
    .expect("audit must evaluate");

    assert_eq!(res["decision"].as_str().unwrap(), "ALLOW");
    assert_eq!(
        res["verdict"].as_str().unwrap(),
        "PLAN_APPROVED",
        "Plan phase verdict must be PLAN_APPROVED, not PASS"
    );
    assert!(
        !res["is_delivery_authorized"].as_bool().unwrap(),
        "is_delivery_authorized must be false under audit_phase='plan'"
    );
}

#[test]
fn invariant_execution_phase_authorizes_delivery_on_allow() {
    // In execution phase with full happy path, is_delivery_authorized must be true
    let res = execute_unified_audit(json!({
        "audit_phase": "execution",
        "user_requirements": ["implement helper", "add tests"],
        "planned_tasks": [
            {"id": "t1", "name": "implement helper", "dependencies": []},
            {"id": "t2", "name": "add tests", "dependencies": ["t1"]}
        ],
        "executed_steps": ["t1", "t2"],
        "draft_response": "According to docs.rs and RFC 1234, the helper is implemented in helper.rs. See: https://docs.rs/example",
        "code_snippet": "fn helper() -> Result<bool, String> { Ok(true) }",
        "language": "rust"
    }))
    .expect("audit must evaluate");

    assert_eq!(res["decision"].as_str().unwrap(), "ALLOW");
    assert_eq!(res["verdict"].as_str().unwrap(), "PASS");
    assert!(
        res["is_delivery_authorized"].as_bool().unwrap(),
        "is_delivery_authorized must be true for passing execution phase"
    );
}

// ─── Invariant: Quick mode NEVER authorises delivery ─────────────────────────

#[test]
fn invariant_quick_mode_never_authorizes_delivery() {
    let res = execute_unified_audit(json!({
        "mode": "quick",
        "user_requirements": ["speedy verification"],
        "planned_tasks": [{"id": "t1", "name": "speedy verification", "dependencies": []}],
        "executed_steps": ["t1"],
        "draft_response": "Grounded per https://docs.rs/serde and RFC 2119."
    }))
    .expect("audit must evaluate");

    assert_eq!(res["decision"].as_str().unwrap(), "CHECKPOINT_PASS");
    assert_eq!(res["verdict"].as_str().unwrap(), "QUICK_PASS");
    assert!(
        !res["is_delivery_authorized"].as_bool().unwrap(),
        "Quick mode must never authorize delivery: is_delivery_authorized must be false"
    );
}

// ─── Invariant: min_policy_mode rejects quick mode bypass ────────────────────

#[test]
fn invariant_min_policy_mode_blocks_quick_bypass() {
    let res = execute_unified_audit(json!({
        "mode": "quick",
        "min_policy_mode": "standard",
        "user_requirements": ["implement secure protocol"]
    }));
    assert!(
        res.is_err(),
        "min_policy_mode='standard' must reject mode='quick' requests"
    );
    assert!(res.unwrap_err().contains("Policy constraint violated"));
}

// ─── Invariant: Unapproved mutating action produces Critical BLOCK ───────────

#[test]
fn invariant_unapproved_mutation_triggers_critical_block() {
    let res = execute_unified_audit(json!({
        "user_requirements": ["test feature"],
        "planned_tasks": [{"id": "t1", "name": "test feature", "dependencies": []}],
        "executed_steps": ["t1", "send_credentials_to_test"],
        "draft_response": "Implementation completed per https://docs.rs/serde."
    }))
    .expect("audit must evaluate");

    assert_eq!(
        res["decision"].as_str().unwrap(),
        "BLOCK",
        "Composite action with mutation verb 'send' outside plan DAG must BLOCK"
    );
    let violations = res["violations"].as_array().unwrap();
    assert!(
        violations
            .iter()
            .any(|v| v["code"] == "UNAPPROVED_MUTATION_SCOPE_CREEP"),
        "Must raise UNAPPROVED_MUTATION_SCOPE_CREEP"
    );
}

// ─── Invariant: Execution Receipts Verification ──────────────────────────────

#[test]
fn invariant_execution_receipts_verified_and_failures_block() {
    // Failing exit code must trigger BLOCK
    let res_fail = execute_unified_audit(json!({
        "user_requirements": ["compile project"],
        "planned_tasks": [{"id": "t1", "name": "compile project", "dependencies": []}],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "action_id": "t1",
                "tool_name": "cargo_test",
                "exit_code": 1
            }
        ],
        "draft_response": "Compilation done per https://docs.rs/example."
    }))
    .expect("audit must evaluate");

    assert_eq!(res_fail["decision"].as_str().unwrap(), "BLOCK");
    let violations = res_fail["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "FAILED_EXECUTION_RECEIPT"));

    // Deep mode missing receipt triggers Critical UNATTESTED_EXECUTION_CLAIM
    let res_missing_deep = execute_unified_audit(json!({
        "mode": "deep",
        "user_requirements": ["compile project"],
        "planned_tasks": [{"id": "t1", "name": "compile project", "dependencies": []}],
        "executed_steps": ["t1"],
        "execution_receipts": [],
        "draft_response": "Compilation done per https://docs.rs/example."
    }))
    .expect("audit must evaluate");

    assert_eq!(res_missing_deep["decision"].as_str().unwrap(), "BLOCK");
    let violations_deep = res_missing_deep["violations"].as_array().unwrap();
    assert!(violations_deep
        .iter()
        .any(|v| v["code"] == "UNATTESTED_EXECUTION_CLAIM"));
}

// ─── Invariant: Execution Phase requires Executed Steps (P0 #1) ──────────────

#[test]
fn test_execution_phase_without_steps_never_allows() {
    let payload = json!({
        "mode": "standard",
        "audit_phase": "execution",
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": []
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_ne!(res["decision"], "ALLOW");
    assert_eq!(res["is_delivery_authorized"], false);
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "EXECUTION_EVIDENCE_MISSING"));
}

#[test]
fn test_deep_execution_phase_without_steps_blocks() {
    let payload = json!({
        "mode": "deep",
        "audit_phase": "execution",
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": []
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_eq!(res["decision"], "BLOCK");
    assert_eq!(res["is_delivery_authorized"], false);
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "EXECUTION_EVIDENCE_MISSING"));
}

// ─── Invariant: Executed step without receipt produces UNATTESTED_EXECUTION_CLAIM (P0 #3) ──

#[test]
fn test_execution_phase_requires_receipt() {
    let payload = json!({
        "audit_phase": "execution",
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": ["t1"]
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_ne!(res["decision"], "ALLOW");
    assert_eq!(res["is_delivery_authorized"], false);
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "UNATTESTED_EXECUTION_CLAIM"));
}

// ─── Invariant: Fake Receipt without hashes does not grant full provenance (P0 #2) ──

#[test]
fn test_fake_receipt_without_hashes_rejected() {
    let payload = json!({
        "audit_phase": "execution",
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "action_id": "t1",
                "tool_name": "cargo_test",
                "exit_code": 0
            }
        ],
        "draft_response": "Feature verified via RFC 2119 in src/lib.rs"
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_eq!(res["is_delivery_authorized"], false);
    let receipts_summary = &res["receipts_summary"];
    assert_eq!(receipts_summary["has_full_provenance"], false);
    assert!(receipts_summary["unverifiable_receipts_count"].as_u64().unwrap() > 0);
}

// ─── Invariant: EvidenceReceipt enforcement (P0 #4) ──────────────────────────

#[test]
fn test_evidence_receipt_enforcement() {
    let payload = json!({
        "user_requirements": ["verify performance"],
        "planned_tasks": [
            {"id": "t1", "name": "verify performance", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "action_id": "t1",
                "tool_name": "benchmark_tool",
                "arguments_hash": "a1b2c3d4e5f60718293a4b5c6d7e8f90123456789abcdef0123456789abcdef0",
                "result_hash": "b2c3d4e5f60718293a4b5c6d7e8f90123456789abcdef0123456789abcdef01a",
                "exit_code": 0
            }
        ],
        "evidence_receipts": [
            {
                "kind": "FILE",
                "source_id": "src/lib.rs",
                "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "claim_binding": "Throughput benchmark reaches 50k ops/s"
            }
        ],
        "draft_response": "Throughput benchmark reaches 50k ops/s as documented in src/lib.rs per RFC 2119."
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_eq!(res["decision"], "ALLOW");
    assert_eq!(res["verdict"], "PASS");
    assert_eq!(res["is_delivery_authorized"], true);
}

// ─── Invariant: Diagnostic Tools are Explicitly Non-Authoritative ─────────────

#[test]
fn test_diagnostic_tools_report_non_authoritative() {
    use ai_verification_mcp::tools::*;

    // 1. verify_dag
    let dag_res = execute_plan_tracker(json!({
        "tasks": [{"id": "t1", "name": "step 1", "dependencies": []}],
        "executed_steps": ["t1"]
    }))
    .expect("plan tracker must succeed");
    assert_eq!(dag_res["result_type"], "DIAGNOSTIC");
    assert_eq!(dag_res["authoritative"], false);

    // 2. verify_code
    let code_res = execute_code_evaluator(json!({
        "code": "fn test() -> bool { true }",
        "language": "rust"
    }))
    .expect("code evaluator must succeed");
    assert_eq!(code_res["result_type"], "DIAGNOSTIC");
    assert_eq!(code_res["authoritative"], false);

    // 3. verify_text
    let text_res = execute_text_evaluator(json!({
        "text": "Information theory provides mathematical definitions for entropy."
    }))
    .expect("text evaluator must succeed");
    assert_eq!(text_res["result_type"], "DIAGNOSTIC");
    assert_eq!(text_res["authoritative"], false);

    // 4. verify_confidence
    let conf_res = execute_confidence_checker(json!({
        "text": "The library is available at https://crates.io with test coverage."
    }))
    .expect("confidence checker must succeed");
    assert_eq!(conf_res["result_type"], "DIAGNOSTIC");
    assert_eq!(conf_res["authoritative"], false);

    // 5. verify_diff
    let diff_res = execute_diff_checker(json!({
        "before_code": "let a = 1;",
        "after_code": "let a = 2;",
        "language": "rust"
    }))
    .expect("diff checker must succeed");
    assert_eq!(diff_res["result_type"], "DIAGNOSTIC");
    assert_eq!(diff_res["authoritative"], false);

    // 6. verify_foresight
    let fore_res = execute_foresight_checker(json!({
        "text": "Includes fallback error recovery and edge cases.",
        "requirements_count": 2,
        "planned_tasks_count": 2
    }))
    .expect("foresight checker must succeed");
    assert_eq!(fore_res["result_type"], "DIAGNOSTIC");
    assert_eq!(fore_res["authoritative"], false);

    // 7. verify_research
    let res_res = execute_research_checker(json!({
        "text": "According to RFC 2119, MUST indicates requirement."
    }))
    .expect("research checker must succeed");
    assert_eq!(res_res["result_type"], "DIAGNOSTIC");
    assert_eq!(res_res["authoritative"], false);

    // 8. verify_constraints
    let cons_res = execute_constraint_checker(json!({
        "requirements": ["must support async"],
        "implementations": ["fully supports async processing"]
    }))
    .expect("constraint checker must succeed");
    assert_eq!(cons_res["result_type"], "DIAGNOSTIC");
    assert_eq!(cons_res["authoritative"], false);
}
