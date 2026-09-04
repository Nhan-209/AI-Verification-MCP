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
