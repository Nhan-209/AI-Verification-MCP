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
    let input: CodeEvaluatorInput =
        serde_json::from_value(args).map_err(|e| format!("Invalid arguments for math_eval_code: {}", e))?;

    let metrics = CodeAnalyzer::analyze(&input.code, &input.language);
    Ok(json!(metrics))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_code_evaluator_valid() {
        let args = json!({
            "code": "fn test() -> i32 { 42 }",
            "language": "rust"
        });
        let res = execute_code_evaluator(args);
        assert!(res.is_ok());
        let val = res.unwrap();
        assert!(val.get("cyclomatic_complexity").is_some());
    }

    #[test]
    fn test_execute_code_evaluator_invalid() {
        let args = json!({ "wrong": 123 });
        let res = execute_code_evaluator(args);
        assert!(res.is_err());
    }
}
