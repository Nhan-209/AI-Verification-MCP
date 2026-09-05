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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_research_checker_valid() {
        let args = json!({
            "text": "Tested on Ubuntu 24.04 LTS. Logs at /var/log/test.log"
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
