use crate::engine::ForesightEngine;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct ForesightCheckerInput {
    pub text: Option<String>,
    pub code: Option<String>,
    #[serde(default)]
    pub requirements_count: usize,
    #[serde(default)]
    pub planned_tasks_count: usize,
}

pub fn execute_foresight_checker(args: Value) -> Result<Value, String> {
    let input: ForesightCheckerInput =
        serde_json::from_value(args).map_err(|e| format!("Invalid arguments for math_eval_foresight: {}", e))?;
    let report = ForesightEngine::evaluate(
        input.text.as_deref(),
        input.code.as_deref(),
        input.requirements_count,
        input.planned_tasks_count,
    );
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
    fn test_execute_foresight_checker_valid() {
        let args = json!({
            "text": "Includes fallback and error handling.",
            "requirements_count": 2,
            "planned_tasks_count": 2
        });
        let res = execute_foresight_checker(args);
        assert!(res.is_ok());
        let val = res.unwrap();
        assert!(val.get("foresight_score").is_some());
    }

    #[test]
    fn test_execute_foresight_checker_invalid() {
        let args = json!("not an object");
        let res = execute_foresight_checker(args);
        assert!(res.is_err());
    }
}
