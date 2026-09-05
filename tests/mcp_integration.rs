use ai_verification_mcp::mcp::{handle_request, JsonRpcRequest};
use serde_json::json;

#[test]
fn test_integration_protocol_lifecycle() {
    // 1. Initialize
    let init_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: Some(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": { "name": "integration-test", "version": "1.0" }
        })),
    };
    let init_res = handle_request(init_req).expect("initialize must respond");
    assert_eq!(init_res.jsonrpc, "2.0");
    assert_eq!(init_res.id, Some(json!(1)));
    assert!(init_res.error.is_none());
    let server_info = init_res.result.unwrap();
    assert_eq!(server_info["serverInfo"]["name"], "ai-verification-mcp");

    // 2. Notification (no response)
    let notify_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: None,
        method: "notifications/initialized".to_string(),
        params: None,
    };
    assert!(handle_request(notify_req).is_none());

    // 3. Ping
    let ping_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "ping".to_string(),
        params: None,
    };
    let ping_res = handle_request(ping_req).expect("ping must respond");
    assert!(ping_res.error.is_none());

    // 4. Tools list
    let list_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(3)),
        method: "tools/list".to_string(),
        params: None,
    };
    let list_res = handle_request(list_req).expect("tools/list must respond");
    let tools = list_res.result.unwrap()["tools"].as_array().unwrap().clone();
    assert_eq!(tools.len(), 9, "Server must expose exactly 9 tools in v0.6.0");
    assert_eq!(tools[0]["name"], "verify_agent");

    // 5. Resources list & Prompts list
    let res_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(4)),
        method: "resources/list".to_string(),
        params: None,
    };
    let res_res = handle_request(res_req).expect("resources/list must respond");
    assert_eq!(res_res.result.unwrap()["resources"], json!([]));

    let pr_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(5)),
        method: "prompts/list".to_string(),
        params: None,
    };
    let pr_res = handle_request(pr_req).expect("prompts/list must respond");
    assert_eq!(pr_res.result.unwrap()["prompts"], json!([]));
}

#[test]
fn test_integration_primary_verify_tools_execution() {
    let tool_calls = vec![
        (
            "verify_agent",
            json!({
                "user_requirements": ["task a"],
                "executed_steps": ["task a"],
                "draft_response": "Completed according to RFC 2119. Reference: https://tools.ietf.org/html/rfc2119"
            }),
        ),
        (
            "verify_dag",
            json!({
                "tasks": [{"id": "1", "name": "step 1", "dependencies": []}],
                "executed_steps": ["1"]
            }),
        ),
        (
            "verify_code",
            json!({
                "code": "fn add(a: i32, b: i32) -> i32 { a + b }",
                "language": "rust"
            }),
        ),
        (
            "verify_diff",
            json!({
                "before_code": "let a = 1;",
                "after_code": "let a = 2;",
                "language": "rust"
            }),
        ),
        (
            "verify_text",
            json!({
                "text": "Information theory provides mathematical definitions for communication channels and entropy."
            }),
        ),
        (
            "verify_confidence",
            json!({
                "text": "The library is available at https://crates.io/crates/serde with over 100M downloads."
            }),
        ),
        (
            "verify_research",
            json!({
                "text": "According to IEEE 754 and docs.rs, float rounding behavior is strictly defined. See: https://docs.rs/example"
            }),
        ),
        (
            "verify_foresight",
            json!({
                "text": "We implement timeout recovery, retry mechanisms, and edge case guards.",
                "planned_tasks_count": 3,
                "requirements_count": 2
            }),
        ),
        (
            "verify_constraints",
            json!({
                "requirements": ["must support async"],
                "implementations": ["fully supports async processing"]
            }),
        ),
    ];

    for (i, (tool_name, args)) in tool_calls.into_iter().enumerate() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(100 + i)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": tool_name,
                "arguments": args
            })),
        };

        let res = handle_request(req).unwrap_or_else(|| panic!("Tool '{}' failed to respond", tool_name));
        assert!(res.error.is_none(), "Tool '{}' returned JSON-RPC error", tool_name);
        let result_val = res.result.expect("Tool result missing");
        let content = result_val["content"].as_array().expect("Result content must be array");
        assert!(!content.is_empty(), "Tool '{}' returned empty content", tool_name);
        assert_eq!(
            result_val.get("isError"),
            None,
            "Tool '{}' reported execution error",
            tool_name
        );
    }
}

#[test]
fn test_integration_legacy_math_aliases() {
    let tool_calls = vec![
        (
            "math_audit_cognition",
            json!({
                "user_requirements": ["task a"],
                "executed_steps": ["task a"],
                "draft_response": "Completed according to RFC 2119. Reference: https://tools.ietf.org/html/rfc2119"
            }),
        ),
        (
            "math_track_dag",
            json!({
                "tasks": [{"id": "1", "name": "step 1", "dependencies": []}],
                "executed_steps": ["1"]
            }),
        ),
        (
            "math_eval_code",
            json!({
                "code": "fn add(a: i32, b: i32) -> i32 { a + b }",
                "language": "rust"
            }),
        ),
        (
            "math_eval_diff",
            json!({
                "before_code": "let a = 1;",
                "after_code": "let a = 2;",
                "language": "rust"
            }),
        ),
        (
            "math_eval_text",
            json!({
                "text": "Information theory provides mathematical definitions for communication channels and entropy."
            }),
        ),
        (
            "math_confidence",
            json!({
                "text": "The library is available at https://crates.io/crates/serde with over 100M downloads."
            }),
        ),
        (
            "math_audit_research",
            json!({
                "text": "According to IEEE 754 and docs.rs, float rounding behavior is strictly defined. See: https://docs.rs/example"
            }),
        ),
        (
            "math_eval_foresight",
            json!({
                "text": "We implement timeout recovery, retry mechanisms, and edge case guards.",
                "planned_tasks_count": 3,
                "requirements_count": 2
            }),
        ),
        (
            "math_verify_constraints",
            json!({
                "requirements": ["must support async"],
                "implementations": ["fully supports async processing"]
            }),
        ),
    ];

    for (i, (tool_name, args)) in tool_calls.into_iter().enumerate() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(200 + i)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": tool_name,
                "arguments": args
            })),
        };

        let res = handle_request(req).unwrap_or_else(|| panic!("Legacy tool '{}' failed to respond", tool_name));
        assert!(
            res.error.is_none(),
            "Legacy tool '{}' returned JSON-RPC error",
            tool_name
        );
        let result_val = res.result.expect("Tool result missing");
        let content = result_val["content"].as_array().expect("Result content must be array");
        assert!(
            !content.is_empty(),
            "Legacy tool '{}' returned empty content",
            tool_name
        );
        assert_eq!(
            result_val.get("isError"),
            None,
            "Legacy tool '{}' reported execution error",
            tool_name
        );
    }
}

#[test]
fn test_integration_unknown_tool_and_method() {
    let bad_tool_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(999)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "non_existent_tool",
            "arguments": {}
        })),
    };
    let res = handle_request(bad_tool_req).expect("Must return response");
    let result = res.result.unwrap();
    assert_eq!(result["isError"], true);

    let bad_method_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1000)),
        method: "unknown/method".to_string(),
        params: None,
    };
    let res2 = handle_request(bad_method_req).expect("Must return response");
    assert_eq!(res2.error.unwrap().code, -32601);
}
