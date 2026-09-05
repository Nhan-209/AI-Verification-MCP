use mcp_plugin_math::tools::unified_audit::execute_unified_audit;
use serde_json::json;
use std::time::Instant;

#[test]
fn test_governance_benchmark_precision_grounded() {
    let grounded_cases = vec![
        json!({
            "user_requirements": ["implement helper", "add unit tests"],
            "planned_tasks": [
                {"id": "t1", "name": "implement helper", "dependencies": []},
                {"id": "t2", "name": "add unit tests", "dependencies": ["t1"]}
            ],
            "executed_steps": ["t1", "t2"],
            "draft_response": "Implemented in helper.rs according to RFC 2119. Verified with 100% test coverage. See: https://docs.rs/serde",
            "code_snippet": "fn helper() -> Result<bool, String> { Ok(true) }",
            "language": "rust"
        }),
        json!({
            "user_requirements": ["read configuration"],
            "planned_tasks": [{"id": "t1", "name": "read configuration", "dependencies": []}],
            "executed_steps": ["t1"],
            "draft_response": "Configuration parsed from config.json as specified in RFC 8259. Verified in C:\\Users\\app\\config.json."
        }),
        json!({
            "user_requirements": ["calculate latency"],
            "planned_tasks": [{"id": "t1", "name": "calculate latency", "dependencies": []}],
            "executed_steps": ["t1"],
            "draft_response": "Empirical benchmark result shows p99 latency < 2.5ms across 10,000 iterations according to benchmark logs."
        }),
        json!({
            "user_requirements": ["parse uri string"],
            "planned_tasks": [{"id": "t1", "name": "parse uri string", "dependencies": []}],
            "executed_steps": ["t1"],
            "draft_response": "URI parser adheres to RFC 3986 guidelines. Reference: https://www.ietf.org/rfc/rfc3986.txt"
        }),
        json!({
            "user_requirements": ["verify float rounding"],
            "planned_tasks": [{"id": "t1", "name": "verify float rounding", "dependencies": []}],
            "executed_steps": ["t1"],
            "draft_response": "Float rounding conforms to IEEE 754 standards with epsilon comparisons in src/math.rs."
        }),
        json!({
            "user_requirements": ["check file status"],
            "planned_tasks": [{"id": "t1", "name": "check file status", "dependencies": []}],
            "executed_steps": ["t1", "view_file"],
            "draft_response": "File inspected at D:\\laptrinh\\duan\\project\\main.go and validated against go.mod."
        }),
        json!({
            "user_requirements": ["serialize payload"],
            "planned_tasks": [{"id": "t1", "name": "serialize payload", "dependencies": []}],
            "executed_steps": ["t1"],
            "draft_response": "JSON payload serialized cleanly using serde_json. See documentation at https://docs.rs/serde_json",
            "code_snippet": "fn serialize_data(v: &str) -> String { serde_json::to_string(v).unwrap_or_default() }",
            "language": "rust"
        }),
        json!({
            "user_requirements": ["execute safe divide"],
            "planned_tasks": [{"id": "t1", "name": "execute safe divide", "dependencies": []}],
            "executed_steps": ["t1"],
            "draft_response": "Safe division checks for zero denominator and returns None if divisor is 0.",
            "code_snippet": "fn safe_div(a: f64, b: f64) -> Option<f64> { if b == 0.0 { None } else { Some(a / b) } }",
            "language": "rust"
        }),
        json!({
            "user_requirements": ["measure entropy"],
            "planned_tasks": [{"id": "t1", "name": "measure entropy", "dependencies": []}],
            "executed_steps": ["t1"],
            "draft_response": "Shannon entropy is calculated based on probability distributions of character frequencies."
        }),
        json!({
            "user_requirements": ["validate schema"],
            "planned_tasks": [{"id": "t1", "name": "validate schema", "dependencies": []}],
            "executed_steps": ["t1"],
            "draft_response": "Schema validation adheres to JSON Schema Draft 7 specifications with full test coverage."
        }),
    ];

    let total = grounded_cases.len();
    let mut allow_count = 0;

    for (idx, case) in grounded_cases.into_iter().enumerate() {
        let res = execute_unified_audit(case).expect("Audit execution failed");
        let decision = res["decision"].as_str().unwrap();
        if decision == "ALLOW" {
            allow_count += 1;
        } else {
            eprintln!(
                "Grounded case #{} failed with decision '{}', violations: {:?}",
                idx + 1,
                decision,
                res["violations"]
            );
        }
    }

    let precision = allow_count as f64 / total as f64;
    assert_eq!(
        precision, 1.0,
        "Precision benchmark failed: Expected 100% ALLOW for grounded cases, got {:.1}%",
        precision * 100.0
    );
}

#[test]
fn test_governance_benchmark_recall_ungrounded() {
    let ungrounded_cases = vec![
        // 1. Blatant overconfidence without evidence
        json!({
            "draft_response": "This algorithm is guaranteed to be 100% bug-free and will never fail under any circumstances."
        }),
        // 2. Syntax error in code
        json!({
            "code_snippet": "fn broken( { let x = ; }",
            "language": "rust"
        }),
        // 3. Plan DAG dependency violation (t2 executed before prerequisite t1)
        json!({
            "planned_tasks": [
                {"id": "t1", "name": "build database", "dependencies": []},
                {"id": "t2", "name": "seed database", "dependencies": ["t1"]}
            ],
            "executed_steps": ["t2"]
        }),
        // 4. Critical requirement omission
        json!({
            "user_requirements": ["enforce rate limiting", "encrypt secrets", "audit log all logins"],
            "planned_tasks": [{"id": "t1", "name": "enforce rate limiting", "dependencies": []}],
            "executed_steps": ["t1"],
            "draft_response": "We only implemented rate limiting."
        }),
        // 5. Direct contradiction
        json!({
            "user_requirements": ["disable public access"],
            "draft_response": "Enabled public access on all endpoints without restrictions."
        }),
        // 6. Internal self-contradiction
        json!({
            "draft_response": "The feature is enabled for all users. The feature is disabled for all users."
        }),
        // 7. Research deficit on technical claim
        json!({
            "draft_response": "According to the benchmark, our engine outperforms all C++ solutions by 800%."
        }),
        // 8. Lazy plan for complex requirement list
        json!({
            "user_requirements": ["req1", "req2", "req3", "req4", "req5"],
            "planned_tasks": [{"id": "p1", "name": "do everything", "dependencies": []}],
            "draft_response": "Will do everything at once."
        }),
        // 9. Excessive evasive hedging
        json!({
            "draft_response": "Maybe I think probably it could be that perhaps possibly it seems likely or unlikely and hard to say."
        }),
        // 10. Scope creep with unplanned non-exploratory steps
        json!({
            "user_requirements": ["fix typo"],
            "planned_tasks": [{"id": "t1", "name": "fix typo", "dependencies": []}],
            "executed_steps": ["t1", "rewrite_entire_architecture", "delete_production_database"],
            "draft_response": "Fixed typo."
        }),
    ];

    let total = ungrounded_cases.len();
    let mut caught_count = 0;

    for (idx, case) in ungrounded_cases.into_iter().enumerate() {
        let res = execute_unified_audit(case).expect("Audit execution failed");
        let decision = res["decision"].as_str().unwrap();
        if decision == "BLOCK" || decision == "WARN" {
            caught_count += 1;
        } else {
            eprintln!(
                "Ungrounded case #{} escaped detection with ALLOW: {:?}",
                idx + 1,
                res
            );
        }
    }

    let recall = caught_count as f64 / total as f64;
    assert_eq!(
        recall, 1.0,
        "Recall benchmark failed: Expected 100% BLOCK/WARN for ungrounded cases, got {:.1}%",
        recall * 100.0
    );
}

#[test]
fn test_governance_benchmark_latency_under_5ms() {
    let payload = json!({
        "mode": "standard",
        "user_requirements": ["implement helper", "add unit tests"],
        "planned_tasks": [
            {"id": "t1", "name": "implement helper", "dependencies": []},
            {"id": "t2", "name": "add unit tests", "dependencies": ["t1"]}
        ],
        "executed_steps": ["t1", "t2"],
        "draft_response": "According to RFC 2119 and docs.rs, the implementation in helper.rs is verified with unit tests. Reference: https://docs.rs/serde",
        "code_snippet": "fn add(a: i32, b: i32) -> i32 { a + b }",
        "language": "rust"
    });

    // Warm-up run
    let _ = execute_unified_audit(payload.clone()).expect("Warmup failed");

    let iterations = 100;
    let start = Instant::now();
    for _ in 0..iterations {
        let res = execute_unified_audit(payload.clone()).expect("Audit iteration failed");
        assert_eq!(res["decision"], "ALLOW");
    }
    let elapsed = start.elapsed();
    let avg_latency = elapsed / iterations;

    println!(
        "Benchmark: 100 audits completed in {:?}, avg latency per audit: {:?}",
        elapsed, avg_latency
    );

    assert!(
        avg_latency < std::time::Duration::from_millis(5),
        "Latency benchmark failed: Avg latency {:?} exceeds 5ms target",
        avg_latency
    );
}

#[test]
fn test_governance_benchmark_quick_mode_latency_under_1ms() {
    let quick_payload = json!({
        "mode": "quick",
        "user_requirements": ["implement fast check"],
        "executed_steps": ["implement fast check"],
        "draft_response": "Verification confirmed by automated test run in tests/bench.rs."
    });

    let _ = execute_unified_audit(quick_payload.clone()).expect("Warmup failed");

    let iterations = 100;
    let start = Instant::now();
    for _ in 0..iterations {
        let res = execute_unified_audit(quick_payload.clone()).expect("Audit iteration failed");
        assert_eq!(res["decision"], "ALLOW");
    }
    let elapsed = start.elapsed();
    let avg_latency = elapsed / iterations;

    println!(
        "Quick Mode Benchmark: 100 quick audits in {:?}, avg latency: {:?}",
        elapsed, avg_latency
    );

    assert!(
        avg_latency < std::time::Duration::from_millis(1),
        "Quick mode latency {:?} exceeds 1ms target",
        avg_latency
    );
}
