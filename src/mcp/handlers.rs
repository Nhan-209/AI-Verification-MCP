use crate::mcp::protocol::{CallToolResult, JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use crate::tools::{
    execute_code_evaluator, execute_confidence_checker, execute_constraint_checker, execute_diff_checker,
    execute_foresight_checker, execute_plan_tracker, execute_research_checker, execute_text_evaluator,
    execute_unified_audit, get_available_tools,
};
use serde_json::{json, Value};

pub fn handle_request(req: JsonRpcRequest) -> Option<JsonRpcResponse> {
    let method = req.method.as_str();
    let id = req.id.clone();

    // Validate JSON-RPC version
    if req.jsonrpc != "2.0" {
        return Some(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code: -32600,
                message: format!("Invalid Request: jsonrpc version must be '2.0', got '{}'", req.jsonrpc),
                data: None,
            }),
        });
    }

    // Notifications do not expect a response
    if method == "notifications/initialized" || method.starts_with("notifications/") {
        return None;
    }

    let response = match method {
        "initialize" => {
            let params = req.params.unwrap_or(Value::Null);
            let requested_version = params
                .get("protocolVersion")
                .and_then(|v| v.as_str())
                .unwrap_or("2026-07-28");
            let negotiated_version = match requested_version {
                "2024-11-05" => "2024-11-05",
                _ => "2026-07-28",
            };
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!({
                    "protocolVersion": negotiated_version,
                    "supportedProtocolVersions": ["2026-07-28", "2024-11-05"],
                    "capabilities": {
                        "tools": { "listChanged": false },
                        "resources": { "subscribe": false, "listChanged": false },
                        "prompts": { "listChanged": false }
                    },
                    "serverInfo": {
                        "name": "ai-verification-mcp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                })),
                error: None,
            }
        }
        "server/discover" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "2026-07-28",
                "supportedProtocolVersions": ["2026-07-28", "2024-11-05"],
                "serverInfo": {
                    "name": "ai-verification-mcp",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "capabilities": {
                    "tools": { "listChanged": false },
                    "resources": { "subscribe": false, "listChanged": false },
                    "prompts": { "listChanged": false }
                },
                "tools": get_available_tools(),
                "resources": [],
                "prompts": []
            })),
            error: None,
        },

        "ping" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({})),
            error: None,
        },
        "resources/list" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "resources": []
            })),
            error: None,
        },
        "prompts/list" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "prompts": []
            })),
            error: None,
        },
        "tools/list" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "tools": get_available_tools()
            })),
            error: None,
        },
        "tools/call" => {
            let params = req.params.unwrap_or(Value::Null);
            let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

            let result = match tool_name {
                // Primary unified governance gate & aliases
                "verify_agent" | "ai_audit_cognition" | "math_audit_cognition" => execute_unified_audit(arguments),
                // Granular diagnostic tools & legacy aliases
                "verify_dag" | "math_track_dag" => execute_plan_tracker(arguments),
                "verify_code" | "math_eval_code" => execute_code_evaluator(arguments),
                "verify_diff" | "math_eval_diff" => execute_diff_checker(arguments),
                "verify_text" | "math_eval_text" => execute_text_evaluator(arguments),
                "verify_confidence" | "math_confidence" => execute_confidence_checker(arguments),
                "verify_research" | "math_audit_research" => execute_research_checker(arguments),
                "verify_foresight" | "math_eval_foresight" => execute_foresight_checker(arguments),
                "verify_constraints" | "math_verify_constraints" => execute_constraint_checker(arguments),
                _ => Err(format!("Unknown tool: '{}'", tool_name)),
            };

            let tool_result = match result {
                Ok(val) => CallToolResult::ok(val.to_string()),
                Err(err) => CallToolResult::err(err),
            };

            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::to_value(tool_result).unwrap_or(Value::Null)),
                error: None,
            }
        }
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", method),
                data: None,
            }),
        },
    };

    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_initialize() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "initialize".to_string(),
            params: None,
        };
        let res = handle_request(req).expect("Should return response");
        assert_eq!(res.jsonrpc, "2.0");
        assert_eq!(res.id, Some(json!(1)));
        assert!(res.error.is_none());
        let result = res.result.unwrap();
        assert_eq!(result["serverInfo"]["name"], "ai-verification-mcp");
    }

    #[test]
    fn test_handle_resources_and_prompts_list() {
        let req_res = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(2)),
            method: "resources/list".to_string(),
            params: None,
        };
        let res = handle_request(req_res).expect("Should return response");
        assert_eq!(res.result.unwrap()["resources"], json!([]));

        let req_prompt = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(3)),
            method: "prompts/list".to_string(),
            params: None,
        };
        let res = handle_request(req_prompt).expect("Should return response");
        assert_eq!(res.result.unwrap()["prompts"], json!([]));
    }

    #[test]
    fn test_handle_notifications_return_none() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: "notifications/initialized".to_string(),
            params: None,
        };
        assert!(handle_request(req).is_none());
    }

    #[test]
    fn test_handle_invalid_jsonrpc_version() {
        let req = JsonRpcRequest {
            jsonrpc: "1.0".to_string(),
            id: Some(json!(4)),
            method: "ping".to_string(),
            params: None,
        };
        let res = handle_request(req).expect("Should return error response");
        assert_eq!(res.error.unwrap().code, -32600);
    }

    #[test]
    fn test_handle_unknown_method() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(5)),
            method: "non_existent_method".to_string(),
            params: None,
        };
        let res = handle_request(req).expect("Should return error response");
        assert_eq!(res.error.unwrap().code, -32601);
    }
}
