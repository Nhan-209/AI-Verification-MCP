use crate::engine::resource_limits::{validate_text_bound, MAX_TEXT_BYTES};
use crate::engine::ResearchGate;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct ResearchCheckerInput {
    pub text: String,
}

pub fn execute_research_checker(args: Value) -> Result<Value, String> {
    let input: ResearchCheckerInput =
        serde_json::from_value(args).map_err(|e| format!("Invalid arguments for math_audit_research: {}", e))?;

    validate_text_bound(&input.text, "text", MAX_TEXT_BYTES)?;

    let report = ResearchGate::audit(&input.text);
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
    fn test_execute_research_checker_valid() {
        let args = json!({
            "text": "According to docs.rs and RFC 2024, tree-sitter v0.24 is compatible with Rust 2021. Reference: https://docs.rs/tree-sitter"
        });
        let res = execute_research_checker(args);
        assert!(res.is_ok());
        let val = res.unwrap();
        assert_eq!(val["verdict"], "RESEARCH_GROUNDED");
    }

    #[test]
    fn test_execute_research_checker_invalid() {
        let args = json!({ "wrong_arg": 42 });
        let res = execute_research_checker(args);
        assert!(res.is_err());
    }
}
