use crate::engine::resource_limits::{
    MAX_EXECUTED_STEPS, MAX_TASKS, MAX_TASK_ID_LEN, MAX_TASK_NAME_LEN,
};
use crate::engine::PlanDag;
use crate::tools::unified_audit::PlanTaskInput;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct PlanTrackerInput {
    pub tasks: Vec<PlanTaskInput>,
    #[serde(default)]
    pub executed_steps: Vec<String>,
}

pub fn execute_plan_tracker(args: Value) -> Result<Value, String> {
    let input: PlanTrackerInput =
        serde_json::from_value(args).map_err(|e| format!("Invalid arguments for math_track_dag: {}", e))?;

    if input.tasks.len() > MAX_TASKS {
        return Err(format!(
            "Resource limit exceeded: tasks count {} > MAX_TASKS {}",
            input.tasks.len(),
            MAX_TASKS
        ));
    }
    if input.executed_steps.len() > MAX_EXECUTED_STEPS {
        return Err(format!(
            "Resource limit exceeded: executed_steps count {} > MAX_EXECUTED_STEPS {}",
            input.executed_steps.len(),
            MAX_EXECUTED_STEPS
        ));
    }

    // Pre-validate task fields (consistent with unified audit contract)
    let mut id_counts: HashMap<&str, usize> = HashMap::new();
    for t in &input.tasks {
        let id_trimmed = t.id.trim();
        let name_trimmed = t.name.trim();

        if id_trimmed.is_empty() {
            return Err("Schema Violation: A task has an empty or whitespace-only ID.".to_string());
        }
        if id_trimmed.len() > MAX_TASK_ID_LEN {
            return Err(format!(
                "Schema Violation: Task ID '{}' exceeds max length {} chars.",
                id_trimmed, MAX_TASK_ID_LEN
            ));
        }
        if name_trimmed.is_empty() {
            return Err("Schema Violation: A task has an empty or whitespace-only name.".to_string());
        }
        if name_trimmed.len() > MAX_TASK_NAME_LEN {
            return Err(format!(
                "Schema Violation: Task name '{}' exceeds max length {} chars.",
                name_trimmed, MAX_TASK_NAME_LEN
            ));
        }
        *id_counts.entry(id_trimmed).or_insert(0) += 1;
    }

    for (id, count) in &id_counts {
        if *count > 1 {
            return Err(format!(
                "Schema Violation: Duplicate task ID '{}' found {} times in plan.",
                id, count
            ));
        }
    }

    let mut dag = PlanDag::new();
    for t in input.tasks {
        dag.add_task(t.id.trim(), t.name.trim(), t.dependencies);
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
        "result_type": "DIAGNOSTIC",
        "authoritative": false,
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
