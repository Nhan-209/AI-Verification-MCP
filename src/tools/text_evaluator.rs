use crate::engine::resource_limits::{validate_text_bound, MAX_TEXT_BYTES};
use crate::engine::TextEvaluator;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct TextEvaluatorInput {
    pub text: String,
}

pub fn execute_text_evaluator(args: Value) -> Result<Value, String> {
    let input: TextEvaluatorInput =
        serde_json::from_value(args).map_err(|e| format!("Invalid arguments for math_eval_text: {}", e))?;

    validate_text_bound(&input.text, "text", MAX_TEXT_BYTES)?;

    let metrics = TextEvaluator::evaluate(&input.text);
    Ok(json!(metrics))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_text_evaluator_valid() {
        let args = json!({ "text": "Quick brown fox jumps over the lazy dog." });
        let res = execute_text_evaluator(args);
        assert!(res.is_ok());
        let val = res.unwrap();
        assert!(val.get("shannon_entropy_bits").is_some());
    }

    #[test]
    fn test_execute_text_evaluator_invalid() {
        let args = json!({ "wrong": 123 });
        let res = execute_text_evaluator(args);
        assert!(res.is_err());
    }
}
