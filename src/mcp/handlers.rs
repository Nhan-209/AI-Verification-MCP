use crate::mcp::protocol::{CallToolResult, JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use crate::tools::{
    execute_code_evaluator, execute_constraint_checker, execute_plan_tracker,
    execute_text_evaluator, execute_unified_audit, get_available_tools,
};
use serde_json::{json, Value};

pub fn handle_request(req: JsonRpcRequest) -> Option<JsonRpcResponse> {
    let method = req.method.as_str();
    let id = req.id.clone();

    // Notifications do not expect a response
    if method == "notifications/initialized" || method.starts_with("notifications/") {
        return None;
    }

    let response = match method {
        "initialize" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "mcp-plugin-math",
                    "version": "0.1.0"
                }
            })),
            error: None,
        },
        "ping" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({})),
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
                "math_audit_cognition" => execute_unified_audit(arguments),
                "math_track_dag" => execute_plan_tracker(arguments),
                "math_eval_code" => execute_code_evaluator(arguments),
                "math_eval_text" => execute_text_evaluator(arguments),
                "math_verify_constraints" => execute_constraint_checker(arguments),
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
