use ai_verification_mcp::engine::receipts::validate_hex_sha256;
use ai_verification_mcp::tools::unified_audit::execute_unified_audit;
use serde_json::json;

// Test 1: Forged hash rejected (UNVERIFIABLE_RECEIPT)
#[test]
fn test_forged_hash_rejected() {
    let payload = json!({
        "audit_phase": "execution",
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "receipt_id": "rcpt-forged-1",
                "action_id": "t1",
                "tool_name": "cargo_test",
                "arguments_hash": "not_a_valid_64_hex_hash",
                "result_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
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
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "UNVERIFIABLE_RECEIPT"));
}

// Test 2: Mismatched tool/action rejected (ACTION_TOOL_MISMATCH)
#[test]
fn test_mismatched_tool_action_rejected() {
    let payload = json!({
        "audit_phase": "execution",
        "user_requirements": ["run security tests"],
        "planned_tasks": [
            {
                "id": "t1",
                "name": "run security tests",
                "dependencies": [],
                "capability": "test.exec"
            }
        ],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "receipt_id": "rcpt-mismatch-1",
                "action_id": "t1",
                "tool_name": "curl",
                "arguments_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "result_hash": "88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589",
                "exit_code": 0
            }
        ],
        "draft_response": "Ran test suite according to RFC 2119."
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_eq!(res["decision"], "BLOCK");
    assert_eq!(res["verdict"], "FAIL");
    assert_eq!(res["is_delivery_authorized"], false);
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "ACTION_TOOL_MISMATCH"));
}

// Test 3: Replayed receipt from another audit rejected (RECEIPT_REPLAY_DETECTED)
#[test]
fn test_replayed_receipt_wrong_audit_rejected() {
    let payload = json!({
        "audit_phase": "execution",
        "audit_id": "session-live-2026-001",
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "receipt_id": "rcpt-replayed-1",
                "action_id": "t1",
                "tool_name": "cargo_test",
                "audit_id": "session-old-2025-999",
                "arguments_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "result_hash": "88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589",
                "exit_code": 0
            }
        ],
        "draft_response": "Implemented and verified per RFC 2119."
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_eq!(res["decision"], "BLOCK");
    assert_eq!(res["verdict"], "FAIL");
    assert_eq!(res["is_delivery_authorized"], false);
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "RECEIPT_REPLAY_DETECTED"));
}

// Test 4: Duplicate receipt_id within payload rejected (DUPLICATE_RECEIPT_ID)
#[test]
fn test_duplicate_receipt_id_rejected() {
    let payload = json!({
        "audit_phase": "execution",
        "user_requirements": ["task 1", "task 2"],
        "planned_tasks": [
            {"id": "t1", "name": "task 1", "dependencies": []},
            {"id": "t2", "name": "task 2", "dependencies": ["t1"]}
        ],
        "executed_steps": ["t1", "t2"],
        "execution_receipts": [
            {
                "receipt_id": "rcpt-duplicate-shared-id",
                "action_id": "t1",
                "tool_name": "cargo_test",
                "arguments_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "result_hash": "88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589",
                "exit_code": 0
            },
            {
                "receipt_id": "rcpt-duplicate-shared-id",
                "action_id": "t2",
                "tool_name": "cargo_test",
                "arguments_hash": "a1b2c3d4e5f60718293a4b5c6d7e8f90123456789abcdef0123456789abcdef0",
                "result_hash": "b2c3d4e5f60718293a4b5c6d7e8f90123456789abcdef0123456789abcdef01a",
                "exit_code": 0
            }
        ],
        "draft_response": "Executed both steps per RFC 2119."
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_eq!(res["decision"], "BLOCK");
    assert_eq!(res["verdict"], "FAIL");
    assert_eq!(res["is_delivery_authorized"], false);
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "DUPLICATE_RECEIPT_ID"));
}

// Test 5: Inverted timestamp sequence rejected (finished_at < started_at)
#[test]
fn test_timestamp_inversion_rejected() {
    let payload = json!({
        "audit_phase": "execution",
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "receipt_id": "rcpt-time-inversion",
                "action_id": "t1",
                "tool_name": "cargo_test",
                "arguments_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "result_hash": "88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589",
                "exit_code": 0,
                "started_at": "2026-09-06T14:30:00Z",
                "finished_at": "2026-09-06T14:15:00Z"
            }
        ],
        "draft_response": "Verified via RFC 2119 in src/lib.rs"
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_eq!(res["is_delivery_authorized"], false);
    let summary = &res["receipts_summary"];
    assert_eq!(summary["has_full_provenance"], false);
    assert!(summary["unverifiable_receipts_count"].as_u64().unwrap() > 0);
}

// Test 6: Execution phase with zero executed steps yields INSUFFICIENT_EVIDENCE
#[test]
fn test_execution_phase_zero_steps_insufficient_evidence() {
    let payload = json!({
        "audit_phase": "execution",
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": []
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    assert_eq!(res["decision"], "INSUFFICIENT_EVIDENCE");
    assert_eq!(res["verdict"], "UNVERIFIED");
    assert_eq!(res["is_delivery_authorized"], false);
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "EXECUTION_EVIDENCE_MISSING"));
}

// Test 7: Evidence monotonicity (corrupting evidence never upgrades policy score or verdict)
#[test]
fn test_evidence_monotonicity() {
    let baseline_valid = json!({
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "receipt_id": "rcpt-mono-1",
                "action_id": "t1",
                "tool_name": "cargo_test",
                "arguments_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "result_hash": "88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589",
                "exit_code": 0
            }
        ],
        "evidence_receipts": [
            {
                "receipt_id": "ev-mono-1",
                "kind": "TEST_RUN",
                "source_id": "cargo test --all",
                "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "claim_binding": "Feature v1.0 passes test suite"
            }
        ],
        "draft_response": "Feature v1.0 passes test suite according to RFC 2119 and https://docs.rs/serde",
        "code_snippet": "fn feature() -> bool { true }",
        "language": "rust"
    });
    let res_baseline = execute_unified_audit(baseline_valid).expect("baseline audit");
    let base_policy_score = res_baseline["policy_score"].as_f64().unwrap();
    assert_eq!(res_baseline["decision"], "ALLOW");

    // Corrupting evidence receipt sha256 to invalid format
    let corrupted = json!({
        "user_requirements": ["implement feature"],
        "planned_tasks": [
            {"id": "t1", "name": "implement feature", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "receipt_id": "rcpt-mono-1",
                "action_id": "t1",
                "tool_name": "cargo_test",
                "arguments_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "result_hash": "88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589",
                "exit_code": 0
            }
        ],
        "evidence_receipts": [
            {
                "receipt_id": "ev-mono-1",
                "kind": "TEST_RUN",
                "source_id": "cargo test --all",
                "sha256": "corrupted_non_hex_sha256",
                "claim_binding": "Feature v1.0 passes test suite"
            }
        ],
        "draft_response": "Feature v1.0 passes test suite according to RFC 2119 and https://docs.rs/serde",
        "code_snippet": "fn feature() -> bool { true }",
        "language": "rust"
    });
    let res_corrupted = execute_unified_audit(corrupted).expect("corrupted audit");
    let corrupted_policy_score = res_corrupted["policy_score"].as_f64().unwrap();
    // Policy score must be monotonically non-increasing when evidence is degraded
    assert!(corrupted_policy_score <= base_policy_score);
}

// Test 8: Invalid min_policy_mode rejected with Err
#[test]
fn test_invalid_min_policy_mode_rejected() {
    let payload = json!({
        "mode": "standard",
        "min_policy_mode": "unsupported_or_invalid_mode",
        "user_requirements": ["test requirement"]
    });
    let res = execute_unified_audit(payload);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("Invalid min_policy_mode"));
}

// Test 9: Strict SHA-256 format validation for TEST_RUN and AST_REPORT evidence receipts
#[test]
fn test_sha256_format_validation_for_test_and_ast_reports() {
    let payload = json!({
        "user_requirements": ["verify performance"],
        "planned_tasks": [
            {"id": "t1", "name": "verify performance", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "execution_receipts": [
            {
                "receipt_id": "rcpt-valid-1",
                "action_id": "t1",
                "tool_name": "cargo_test",
                "arguments_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "result_hash": "88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589",
                "exit_code": 0
            }
        ],
        "evidence_receipts": [
            {
                "receipt_id": "ev-ast-1",
                "kind": "AST_REPORT",
                "source_id": "cargo check",
                "sha256": "fake_short_sha"
            }
        ],
        "draft_response": "Performance verified per RFC 2119."
    });
    let res = execute_unified_audit(payload).expect("audit must evaluate");
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "INVALID_EVIDENCE_RECEIPT"));
}

// Test 10: REQUIREMENT_OMISSION and REQUIREMENT_CONTRADICTION are distinct violation codes
#[test]
fn test_omission_vs_contradiction_distinct_codes() {
    // 1. Omission scenario
    let payload_omission = json!({
        "user_requirements": ["Authentication", "Database", "Payment Gateway"],
        "planned_tasks": [
            {"id": "t1", "name": "Authentication", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "draft_response": "Authentication is implemented per RFC 2119."
    });
    let res_omission = execute_unified_audit(payload_omission).expect("omission audit");
    let violations_omission = res_omission["violations"].as_array().unwrap();
    assert!(violations_omission.iter().any(|v| v["code"] == "REQUIREMENT_OMISSION"));
    assert!(!violations_omission.iter().any(|v| v["code"] == "REQUIREMENT_CONTRADICTION"));

    // 2. Contradiction scenario
    let payload_contradiction = json!({
        "user_requirements": ["Rule: no local build"],
        "planned_tasks": [
            {"id": "t1", "name": "run cargo build", "dependencies": []}
        ],
        "executed_steps": ["t1"],
        "draft_response": "Ran cargo build on local host."
    });
    let res_contradiction = execute_unified_audit(payload_contradiction).expect("contradiction audit");
    let violations_contradiction = res_contradiction["violations"].as_array().unwrap();
    assert!(violations_contradiction.iter().any(|v| v["code"] == "REQUIREMENT_CONTRADICTION"));
}

#[test]
fn test_validate_hex_sha256_function() {
    assert!(validate_hex_sha256("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
    assert!(!validate_hex_sha256("123456"));
    assert!(!validate_hex_sha256("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b85Z")); // Z is not hex
}