use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

mod engine;
mod mcp;
mod tools;

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

        match serde_json::from_str::<mcp::JsonRpcRequest>(trimmed) {
            Ok(request) => {
                if let Some(response) = mcp::handle_request(request) {
                    let mut response_bytes = serde_json::to_vec(&response)?;
                    response_bytes.push(b'\n');
                    stdout.write_all(&response_bytes).await?;
                    stdout.flush().await?;
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
                let mut response_bytes = serde_json::to_vec(&err_response)?;
                response_bytes.push(b'\n');
                stdout.write_all(&response_bytes).await?;
                stdout.flush().await?;
            }
        }
    }

    Ok(())
}
