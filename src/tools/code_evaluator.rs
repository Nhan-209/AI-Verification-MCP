use crate::engine::CodeAnalyzer;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct CodeEvaluatorInput {
    pub code: String,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "rust".to_string()
}

pub fn execute_code_evaluator(args: Value) -> Result<Value, String> {
    let input: CodeEvaluatorInput = serde_json::from_value(args)
        .map_err(|e| format!("Invalid arguments for math_eval_code: {}", e))?;

    let metrics = CodeAnalyzer::analyze(&input.code, &input.language);
    Ok(json!(metrics))
}
