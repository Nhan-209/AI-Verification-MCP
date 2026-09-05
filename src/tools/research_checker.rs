use crate::engine::ResearchGate;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct ResearchCheckerInput {
    pub text: String,
}

pub fn execute_research_checker(args: Value) -> Result<Value, String> {
    let input: ResearchCheckerInput = serde_json::from_value(args)
        .map_err(|e| format!("Invalid arguments for math_audit_research: {}", e))?;
    let report = ResearchGate::audit(&input.text);
    Ok(json!(report))
}
