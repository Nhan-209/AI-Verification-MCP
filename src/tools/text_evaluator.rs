use crate::engine::TextEvaluator;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct TextEvaluatorInput {
    pub text: String,
}

pub fn execute_text_evaluator(args: Value) -> Result<Value, String> {
    let input: TextEvaluatorInput = serde_json::from_value(args)
        .map_err(|e| format!("Invalid arguments for math_eval_text: {}", e))?;

    let metrics = TextEvaluator::evaluate(&input.text);
    Ok(json!(metrics))
}
