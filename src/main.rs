use ai_verification_mcp::engine::resource_limits::MAX_JSON_REQUEST_BYTES;
use ai_verification_mcp::mcp;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let mut stdout = tokio::io::stdout();

    while let Some(line) = reader.next_line().await? {
        let mut trimmed = line.trim();
        if trimmed.starts_with('\u{feff}') {
            trimmed = &trimmed['\u{feff}'.len_utf8()..];
        }
        if trimmed.is_empty() {
            continue;
        }

        // Transport-level frame size guard against DoS payload exhaustion
        if trimmed.len() > MAX_JSON_REQUEST_BYTES {
            let err_response = mcp::JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: None,
                result: None,
                error: Some(mcp::protocol::JsonRpcError {
                    code: -32600,
                    message: format!(
                        "Invalid Request: Request frame size {} bytes exceeds transport limit of {} bytes",
                        trimmed.len(),
                        MAX_JSON_REQUEST_BYTES
                    ),
                    data: None,
                }),
            };
            if let Ok(mut response_bytes) = serde_json::to_vec(&err_response) {
                response_bytes.push(b'\n');
                let _ = stdout.write_all(&response_bytes).await;
                let _ = stdout.flush().await;
            }
            continue;
        }

        match serde_json::from_str::<mcp::JsonRpcRequest>(trimmed) {
            Ok(request) => {
                if let Some(response) = mcp::handle_request(request) {
                    match serde_json::to_vec(&response) {
                        Ok(mut response_bytes) => {
                            response_bytes.push(b'\n');
                            if let Err(e) = stdout.write_all(&response_bytes).await {
                                eprintln!("Failed to write response to stdout: {}", e);
                            } else if let Err(e) = stdout.flush().await {
                                eprintln!("Failed to flush stdout: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Failed to serialize response: {}", e),
                    }
                }
            }
            Err(err) => {
                let err_response = mcp::JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(mcp::protocol::JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", err),
                        data: None,
                    }),
                };
                match serde_json::to_vec(&err_response) {
                    Ok(mut response_bytes) => {
                        response_bytes.push(b'\n');
                        if let Err(e) = stdout.write_all(&response_bytes).await {
                            eprintln!("Failed to write error response: {}", e);
                        } else if let Err(e) = stdout.flush().await {
                            eprintln!("Failed to flush error response: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to serialize error response: {}", e),
                }
            }
        }
    }

    Ok(())
}
