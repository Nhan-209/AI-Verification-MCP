use crate::engine::ConstraintEngine;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct ConstraintCheckerInput {
    pub requirements: Vec<String>,
    pub implementations: Vec<String>,
}

pub fn execute_constraint_checker(args: Value) -> Result<Value, String> {
    let input: ConstraintCheckerInput = serde_json::from_value(args)
        .map_err(|e| format!("Invalid arguments for math_verify_constraints: {}", e))?;

    let report = ConstraintEngine::verify(&input.requirements, &input.implementations);
    Ok(json!(report))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_constraint_checker_valid() {
        let args = json!({
            "requirements": ["req1"],
            "implementations": ["req1 done"]
        });
        let res = execute_constraint_checker(args);
        assert!(res.is_ok());
        let val = res.unwrap();
        assert_eq!(val["satisfied_requirements"], 1);
    }

    #[test]
    fn test_execute_constraint_checker_invalid() {
        let args = json!({ "requirements": "should be array" });
        let res = execute_constraint_checker(args);
        assert!(res.is_err());
    }
}
