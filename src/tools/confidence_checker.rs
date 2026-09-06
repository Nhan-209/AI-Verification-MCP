use crate::engine::resource_limits::{validate_text_bound, MAX_TEXT_BYTES};
use crate::engine::ConfidenceAnalyzer;
use serde::Deserialize;
use serde_json::{json, Value};

/// Input structure for the confidence checker tool.
#[derive(Debug, Deserialize)]
pub struct ConfidenceCheckerInput {
    /// The text to analyze for confidence metrics.
    pub text: String,
}

/// Executes the confidence checking tool against the provided input arguments.
///
/// Wraps `ConfidenceAnalyzer::analyze` and maps errors if arguments are invalid.
pub fn execute_confidence_checker(args: Value) -> Result<Value, String> {
    let input: ConfidenceCheckerInput =
        serde_json::from_value(args).map_err(|e| format!("Invalid arguments for math_confidence: {}", e))?;

    validate_text_bound(&input.text, "text", MAX_TEXT_BYTES)?;

    let report = ConfidenceAnalyzer::analyze(&input.text);
    let mut val = serde_json::to_value(report).map_err(|e| e.to_string())?;
    if let Some(obj) = val.as_object_mut() {
        obj.insert("result_type".to_string(), json!("DIAGNOSTIC"));
        obj.insert("authoritative".to_string(), json!(false));
    }

    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_confidence_checker_valid() {
        let args = json!({
            "text": "This is a tested statement with evidence at https://example.com"
        });
        let res = execute_confidence_checker(args);
        assert!(res.is_ok());
        let val = res.unwrap();
        assert!(val.get("calibration_score").is_some());
    }

    #[test]
    fn test_execute_confidence_checker_invalid() {
        let args = json!({ "wrong_key": 123 });
        let res = execute_confidence_checker(args);
        assert!(res.is_err());
    }
}
