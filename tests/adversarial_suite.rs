use ai_verification_mcp::engine::{
    ConfidenceAnalyzer, ConstraintEngine, EvidenceStatus, ResearchGate,
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
