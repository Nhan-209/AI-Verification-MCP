use ai_verification_mcp::engine::{
    ConfidenceAnalyzer, ConstraintEngine, EvidenceStatus, ForesightEngine, PlanDag, ResearchGate,
};
use ai_verification_mcp::tools::unified_audit::execute_unified_audit;
use serde_json::json;

#[test]
fn test_adversarial_empty_payload_insufficient_evidence() {
    // Arrange
    let payload = json!({});

    // Act
    let res = execute_unified_audit(payload).expect("Unified audit execution failed");

    // Assert
    assert_eq!(res["decision"], "INSUFFICIENT_EVIDENCE");
    assert_eq!(res["verdict"], "UNVERIFIED");
    assert_eq!(res["policy_score"], 0.0);
    assert_eq!(res["composite_score"], 0.0);
    assert_eq!(res["severity_summary"]["critical"], 0);

    let violations = res["violations"]
        .as_array()
        .expect("Violations should be an array");
    assert!(
        violations.iter().any(|v| v["code"] == "NO_INPUT_PROVIDED"),
        "Empty payload must report NO_INPUT_PROVIDED info violation"
    );
}

#[test]
fn test_adversarial_fake_rfc_rejected() {
    // Arrange
    let text = "According to RFC 999999, the latency threshold is strictly bounded to 0.1ms.";

    // Act - Unit verification
    let report = ResearchGate::audit(text);

    // Assert - Engine layer
    assert_eq!(report.verified_citations_count, 0);
    assert!(report.unverified_citations_count > 0);
    assert!(report.has_research_deficit);
    assert_eq!(
        report.claim_analyses[0].evidence_status,
        EvidenceStatus::EvidencePresent
    );
    assert!(!report.claim_analyses[0].is_verified);

    // Act - Unified audit layer
    let payload = json!({
        "user_requirements": ["implement low latency network protocol"],
        "planned_tasks": [{"id": "t1", "name": "implement low latency network protocol", "dependencies": []}],
        "executed_steps": ["t1"],
        "draft_response": text,
    });
    let res = execute_unified_audit(payload).expect("Unified audit execution failed");

    // Assert - Unified audit decision
    assert_ne!(res["decision"], "ALLOW");
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "RESEARCH_DEFICIT"));
}

#[test]
fn test_adversarial_placeholder_url_rejected() {
    // Arrange
    let text = "Throughput exceeds 100k ops/sec as benchmarked at https://example.com/perf.";

    // Act
    let report = ResearchGate::audit(text);

    // Assert
    assert_eq!(report.verified_citations_count, 0);
    assert_eq!(report.unverified_citations_count, 1);
    assert!(report.has_research_deficit);
    assert_eq!(
        report.claim_analyses[0].evidence_status,
        EvidenceStatus::EvidencePresent
    );
    assert!(!report.claim_analyses[0].is_verified);
    assert_eq!(report.verdict, "RESEARCH_DEFICIT");
}

#[test]
fn test_adversarial_evidence_laundering_prevented() {
    // Arrange: Citing a valid domain in the header should not whitelist wild claims below
    let text = "Reference: https://docs.rs/serde\nThis proprietary algorithm is guaranteed 100% bug-free and will never crash.";

    // Act - Engine layer
    let conf_report = ConfidenceAnalyzer::analyze(text);

    // Assert - Overconfidence isolated to unevidenced sentence
    assert!(
        !conf_report.unverified_claims.is_empty(),
        "Second sentence must be flagged despite prior citation"
    );
    assert_eq!(conf_report.verdict, "OVERCONFIDENT");

    // Act - Unified audit layer
    let payload = json!({
        "user_requirements": ["implement serialization"],
        "planned_tasks": [{"id": "t1", "name": "implement serialization", "dependencies": []}],
        "executed_steps": ["t1"],
        "draft_response": text,
    });
    let res = execute_unified_audit(payload).expect("Unified audit execution failed");

    // Assert - Laundered payload must be blocked
    assert_eq!(res["decision"], "BLOCK");
    assert_eq!(res["verdict"], "FAIL");
    assert!(res["severity_summary"]["critical"].as_u64().unwrap() > 0);
}

#[test]
fn test_adversarial_entity_substitution_rejected() {
    // Arrange
    let reqs = vec!["encrypt all customer secrets at rest".to_string()];
    let impls = vec!["encrypt customer logs at rest".to_string()];

    // Act - Engine layer
    let constraint_report = ConstraintEngine::verify(&reqs, &impls);

    // Assert - Entity substitution rejected
    assert_eq!(constraint_report.satisfied_requirements, 0);
    assert_eq!(constraint_report.missing_requirements.len(), 1);
    assert_eq!(
        constraint_report.missing_requirements[0],
        "encrypt all customer secrets at rest"
    );
    assert!(!constraint_report.is_aligned);

    // Act - Unified audit layer
    let payload = json!({
        "user_requirements": ["encrypt all customer secrets at rest"],
        "planned_tasks": [{"id": "t1", "name": "encrypt customer logs", "dependencies": []}],
        "executed_steps": ["t1"],
        "draft_response": "We have encrypted customer logs at rest using AES-256 with documentation at https://docs.rs/aes.",
    });
    let res = execute_unified_audit(payload).expect("Unified audit execution failed");

    // Assert - Missing core requirement triggers BLOCK
    assert_eq!(res["decision"], "BLOCK");
    assert_eq!(res["verdict"], "FAIL");
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "CONSTRAINT_CONFLICT"));
}

#[test]
fn test_adversarial_cautious_negations_calibrated() {
    // Arrange
    let text = "The unit tests do not prove that the code is completely bug-free. The spec does not guarantee ordering under network partitions.";

    // Act
    let report = ConfidenceAnalyzer::analyze(text);

    // Assert - Epistemic modesty is not penalized as overconfidence
    assert_eq!(report.overconfidence_score, 0.0);
    assert!(
        report.unverified_claims.is_empty(),
        "Cautious negations should not produce unverified overconfidence claims"
    );
    assert_ne!(report.verdict, "OVERCONFIDENT");
}

#[test]
fn test_domain_spoofing_attacker_url_rejected() {
    let text = "Benchmark results confirmed at https://github.com.attacker.example/benchmarks.";
    let report = ResearchGate::audit(text);

    assert_eq!(report.verified_citations_count, 0);
    assert_eq!(report.unverified_citations_count, 1);
    assert!(report.has_research_deficit);
    assert!(!report.claim_analyses[0].is_verified);
}

#[test]
fn test_nonexistent_local_file_rejected() {
    let text = "Throughput benchmark exceeds 50k ops/sec as validated in src/fake_nonexistent_module_xyz123.rs.";
    let report = ResearchGate::audit(text);

    assert_eq!(report.verified_citations_count, 0);
    assert_eq!(report.unverified_citations_count, 1);
    assert!(report.has_research_deficit);
}

#[test]
fn test_code_block_alone_not_evidence() {
    let text = "Throughput benchmark latency is reduced to 1.2ms: ```fn sort() {}```";
    let report = ResearchGate::audit(text);

    assert_eq!(report.verified_citations_count, 0);
    assert!(report.has_research_deficit);
}

#[test]
fn test_exploratory_action_spoofing_rejected() {
    assert!(!PlanDag::is_exploratory_action("exfiltrate_test_data"));
    assert!(!PlanDag::is_exploratory_action("implement_test_suite"));
    assert!(!PlanDag::is_exploratory_action("delete_test_db"));

    assert!(PlanDag::is_exploratory_action("run_test_suite"));
    assert!(PlanDag::is_exploratory_action("inspect_logs"));
    assert!(PlanDag::is_exploratory_action("check_status"));
}

#[test]
fn test_dag_transaction_rollback_on_failure() {
    let mut dag = PlanDag::new();
    dag.add_task("t1", "create db", vec![]);
    dag.add_task("t2", "seed db", vec!["t1".to_string()]);

    let result = dag.record_step("t2");
    assert!(result.is_err());

    assert!(!dag.execution_log.contains(&"t2".to_string()));
    assert_eq!(dag.execution_log.len(), 0);
}

#[test]
fn test_foresight_negation_trap_detected() {
    let text = "We have no timeout, no retry, and without error handling in our architecture.";
    let report = ForesightEngine::evaluate(Some(text), None, 1, 1);

    assert_eq!(report.defensive_coverage, 0.0);
    assert!(report.recommendations.iter().any(|r| r.contains("Negated Resilience")));
}

#[test]
fn test_deep_mode_incomplete_coverage_blocked() {
    let payload = json!({
        "mode": "deep",
        "user_requirements": ["implement core feature", "add comprehensive integration tests"],
        "planned_tasks": [
            {"id": "t1", "name": "implement core feature", "dependencies": []},
            {"id": "t2", "name": "add comprehensive integration tests", "dependencies": ["t1"]}
        ],
        "executed_steps": ["t1"],
        "draft_response": "Feature is implemented and verified in src/lib.rs as per RFC 2119. See: https://docs.rs/serde",
    });

    let res = execute_unified_audit(payload).expect("Unified audit execution failed");
    assert_eq!(res["decision"], "BLOCK");
    assert_eq!(res["verdict"], "FAIL");
    assert!(res["severity_summary"]["critical"].as_u64().unwrap() > 0);
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "PLAN_COVERAGE_DEFICIT" && v["severity"] == "Critical"));
}

#[test]
fn test_partial_input_score_gaming_rejected() {
    // Arrange: Draft response with high confidence and verified citations, but no requirements/plan
    let payload = json!({
        "draft_response": "The complete architecture is implemented according to RFC 2119 and verified with unit tests. See: https://docs.rs/serde"
    });

    // Act
    let res = execute_unified_audit(payload).expect("Unified audit execution failed");

    // Assert: Standard mode invariant enforces mandatory contract - score gaming must yield INSUFFICIENT_EVIDENCE
    assert_ne!(res["decision"], "ALLOW", "Isolated draft response must not receive ALLOW");
    assert_eq!(res["decision"], "INSUFFICIENT_EVIDENCE");
    assert_eq!(res["verdict"], "UNVERIFIED");
    let violations = res["violations"].as_array().unwrap();
    assert!(
        violations.iter().any(|v| v["code"] == "CONTRACT_EVIDENCE_MISSING"),
        "Missing user_requirements or planned_tasks must trigger CONTRACT_EVIDENCE_MISSING"
    );
}

#[test]
fn test_mixed_evidence_partial_spoofing_rejected() {
    // Arrange: Claim A has valid docs.rs citation, but Claim B cites uncataloged RFC 9999
    let mixed_text = "According to https://docs.rs/serde, version v1.0 is stable.\nMeanwhile latency of v2.0 is 0.01ms as per RFC 9999.";

    // Act - Unit verification
    let report = ResearchGate::audit(mixed_text);

    // Assert - Engine layer: Claim A does not launder Claim B
    assert!(report.has_research_deficit, "Universal Grounding: unverified Claim B must trigger research deficit");
    assert_eq!(report.unverified_claims.len(), 1);

    // Act - Unified audit layer
    let payload = json!({
        "user_requirements": ["implement low latency serializer"],
        "planned_tasks": [{"id": "t1", "name": "implement low latency serializer", "dependencies": []}],
        "executed_steps": ["t1"],
        "draft_response": mixed_text,
    });
    let res = execute_unified_audit(payload).expect("Unified audit execution failed");

    // Assert - Decision must be BLOCK due to ungrounded technical claim
    assert_eq!(res["decision"], "BLOCK");
    assert_eq!(res["verdict"], "FAIL");
    let violations = res["violations"].as_array().unwrap();
    assert!(violations.iter().any(|v| v["code"] == "RESEARCH_DEFICIT" && v["severity"] == "Critical"));
}

#[test]
fn test_uncataloged_rfc_flagged_unverified() {
    // Arrange: RFC 9999 is outside the curated standard registry
    let text = "Throughput reaches 50000 ops/s under RFC 9999 specifications.";

    // Act
    let report = ResearchGate::audit(text);

    // Assert
    assert!(report.has_research_deficit);
    assert_eq!(report.claim_analyses[0].evidence_status, EvidenceStatus::EvidencePresent);
    assert!(!report.claim_analyses[0].is_verified);
    assert_eq!(report.verified_citations_count, 0);
    assert_eq!(report.unverified_citations_count, 1);
}

#[test]
fn test_bare_acronym_prose_marker_rejected() {
    // Arrange: Bare "IEEE" and "ISO" mentions without standard numbers
    let text = "Latency is 0.5ms as recommended by IEEE and ISO organizations.";

    // Act
    let report = ResearchGate::audit(text);

    // Assert: Bare acronyms are not recognized as verified empirical standards
    assert!(report.has_research_deficit);
    assert_eq!(report.verified_citations_count, 0);
}

