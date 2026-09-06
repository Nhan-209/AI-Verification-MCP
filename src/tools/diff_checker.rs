use crate::engine::diff_analysis::DiffAnalyzer;
use crate::engine::resource_limits::{validate_text_bound, MAX_DIFF_BYTES};
use serde::Deserialize;
use serde_json::{json, Value};

/// Input structure for the diff checker tool
#[derive(Debug, Deserialize)]
pub struct DiffCheckerInput {
    pub before_code: String,
    pub after_code: String,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "rust".to_string()
}

/// Executes the diff checking tool
pub fn execute_diff_checker(args: Value) -> Result<Value, String> {
    let input: DiffCheckerInput =
        serde_json::from_value(args).map_err(|e| format!("Invalid arguments for math_eval_diff: {}", e))?;

    validate_text_bound(&input.before_code, "before_code", MAX_DIFF_BYTES)?;
    validate_text_bound(&input.after_code, "after_code", MAX_DIFF_BYTES)?;

    let report = DiffAnalyzer::analyze(&input.before_code, &input.after_code, &input.language);
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
    fn test_execute_diff_checker_valid() {
        let args = json!({
            "before_code": "let a = 1;",
            "after_code": "let a = 2;"
        });
        let res = execute_diff_checker(args);
        assert!(res.is_ok());
        let val = res.unwrap();
        assert_eq!(val["lines_before"], 1);
        assert_eq!(val["lines_after"], 1);
    }

    #[test]
    fn test_execute_diff_checker_invalid() {
        let args = json!({ "before_code": 123 });
        let res = execute_diff_checker(args);
        assert!(res.is_err());
    }
}
