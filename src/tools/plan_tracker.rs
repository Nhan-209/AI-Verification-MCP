use crate::engine::PlanDag;
use crate::tools::unified_audit::PlanTaskInput;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct PlanTrackerInput {
    pub tasks: Vec<PlanTaskInput>,
    #[serde(default)]
    pub executed_steps: Vec<String>,
}

pub fn execute_plan_tracker(args: Value) -> Result<Value, String> {
    let input: PlanTrackerInput =
        serde_json::from_value(args).map_err(|e| format!("Invalid arguments for math_track_dag: {}", e))?;

    let mut dag = PlanDag::new();
    for t in input.tasks {
        dag.add_task(t.id, t.name, t.dependencies);
    }

    let validation = dag.validate_graph();
    let mut step_errors = Vec::new();

    for step in &input.executed_steps {
        if let Err(err) = dag.record_step(step) {
            step_errors.push(err);
        }
    }

    let metrics = dag.evaluate_metrics();

    Ok(json!({
        "graph_valid": validation.is_ok(),
        "graph_validation_error": validation.err(),
        "step_errors": step_errors,
        "metrics": metrics
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_plan_tracker_valid() {
        let args = json!({
            "tasks": [
                {"id": "t1", "name": "setup", "dependencies": []}
            ],
            "executed_steps": ["t1"]
        });
        let res = execute_plan_tracker(args);
        assert!(res.is_ok());
        let val = res.unwrap();
        assert_eq!(val["graph_valid"], true);
    }

    #[test]
    fn test_execute_plan_tracker_invalid() {
        let args = json!({ "tasks": "invalid" });
        let res = execute_plan_tracker(args);
        assert!(res.is_err());
    }
}
