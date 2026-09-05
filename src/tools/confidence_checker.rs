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
    let input: ConfidenceCheckerInput = serde_json::from_value(args)
        .map_err(|e| format!("Invalid arguments for math_confidence: {}", e))?;
    
    let report = ConfidenceAnalyzer::analyze(&input.text);
    
    Ok(json!(report))
}
