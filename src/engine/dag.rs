use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanTask {
    pub id: String,
    pub name: String,
    pub dependencies: Vec<String>,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagMetrics {
    pub total_planned: usize,
    pub completed_planned: usize,
    pub coverage_ratio: f64,
    pub scope_creep_count: usize,
    pub unapproved_tasks: Vec<String>,
    #[serde(default)]
    pub justified_explorations: Vec<String>,
    pub dependency_violations: Vec<String>,
    pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDag {
    pub tasks: HashMap<String, PlanTask>,
    pub execution_log: Vec<String>,
}

impl Default for PlanDag {
    fn default() -> Self {
        Self::new()
    }
}

impl PlanDag {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            execution_log: Vec::new(),
        }
    }

    pub fn add_task(&mut self, id: impl Into<String>, name: impl Into<String>, dependencies: Vec<String>) {
        let id_str = id.into();
        self.tasks.insert(
            id_str.clone(),
            PlanTask {
                id: id_str,
                name: name.into(),
                dependencies,
                status: TaskStatus::Pending,
            },
        );
    }

    /// Verifies if the planned DAG has cycles or invalid references.
    pub fn validate_graph(&self) -> Result<Vec<String>, String> {
        let mut graph = DiGraph::<String, ()>::new();
        let mut node_indices = HashMap::new();

        for id in self.tasks.keys() {
            let idx = graph.add_node(id.clone());
            node_indices.insert(id.clone(), idx);
        }

        for (id, task) in &self.tasks {
            let to_idx = node_indices[id];
            for dep in &task.dependencies {
                match node_indices.get(dep) {
                    Some(&from_idx) => {
                        // Edge from dependency to dependent task
                        graph.add_edge(from_idx, to_idx, ());
                    }
                    None => {
                        return Err(format!("Task '{}' references unknown dependency '{}'", id, dep));
                    }
                }
            }
        }

        match toposort(&graph, None) {
            Ok(order) => {
                let order_ids: Vec<String> = order.into_iter().map(|idx| graph[idx].clone()).collect();
                Ok(order_ids)
            }
            Err(cycle) => Err(format!(
                "Cycle detected in plan DAG involving task node: {:?}",
                graph[cycle.node_id()]
            )),
        }
    }

    /// Checks if an unplanned task is a benign exploratory or verification step.
    pub fn is_exploratory_action(action_id: &str) -> bool {
        let lower = action_id.to_lowercase();
        if lower.starts_with("implement")
            || lower.starts_with("create")
            || lower.starts_with("build")
            || lower.starts_with("add")
            || lower.starts_with("modify")
            || lower.starts_with("delete")
            || lower.starts_with("remove")
            || lower.starts_with("thực thi")
            || lower.starts_with("xóa")
            || lower.starts_with("tạo")
        {
            return false;
        }
        const EXPLORATORY_KEYWORDS: &[&str] = &[
            "read", "view", "check", "inspect", "grep", "search", "list",
            "find", "stat", "status", "test", "audit", "verify", "diff",
            "khảo sát", "đọc", "kiểm tra", "tìm", "tra cứu"
        ];
        EXPLORATORY_KEYWORDS.iter().any(|&k| lower.contains(k))
    }

    /// Records an execution step and validates it against dependencies and approved scope.
    pub fn record_step(&mut self, task_id: &str) -> Result<String, String> {
        self.execution_log.push(task_id.to_string());

        if let Some(task) = self.tasks.get(task_id) {
            // Check dependencies
            for dep_id in &task.dependencies {
                if let Some(dep_task) = self.tasks.get(dep_id) {
                    if dep_task.status != TaskStatus::Completed {
                        return Err(format!(
                            "Dependency violation: Task '{}' executed before dependency '{}' was completed",
                            task_id, dep_id
                        ));
                    }
                }
            }
        } else {
            // Unplanned action: distinguish justified exploration from actual scope creep
            if Self::is_exploratory_action(task_id) {
                return Ok(format!(
                    "Justified discovery step '{}' recorded (exploratory action outside plan DAG)",
                    task_id
                ));
            } else {
                return Err(format!(
                    "Scope creep violation: Task '{}' was executed but not present in approved plan DAG",
                    task_id
                ));
            }
        }

        if let Some(task) = self.tasks.get_mut(task_id) {
            task.status = TaskStatus::Completed;
        }

        Ok(format!(
            "Task '{}' executed successfully within planned DAG",
            task_id
        ))
    }

    /// Computes formal graph metrics: Coverage (C) and Waste/Scope Creep (W).
    pub fn evaluate_metrics(&self) -> DagMetrics {
        let total_planned = self.tasks.len();
        let mut completed_planned = 0;
        let mut dependency_violations = Vec::new();

        let mut completed_set = HashSet::new();
        for (id, task) in &self.tasks {
            if task.status == TaskStatus::Completed {
                completed_planned += 1;
                completed_set.insert(id.clone());
            }
        }

        // Verify topological order across all executed tasks in log
        let mut seen_in_log = HashSet::new();
        for task_id in &self.execution_log {
            if let Some(task) = self.tasks.get(task_id) {
                for dep in &task.dependencies {
                    if !seen_in_log.contains(dep) {
                        dependency_violations.push(format!(
                            "Task '{}' executed before dependency '{}' in log order",
                            task_id, dep
                        ));
                    }
                }
            }
            seen_in_log.insert(task_id.clone());
        }

        let mut unapproved_tasks = Vec::new();
        let mut justified_explorations = Vec::new();
        for task_id in &self.execution_log {
            if !self.tasks.contains_key(task_id) {
                if Self::is_exploratory_action(task_id) {
                    justified_explorations.push(task_id.clone());
                } else {
                    unapproved_tasks.push(task_id.clone());
                }
            }
        }

        let coverage_ratio = if total_planned == 0 {
            1.0
        } else {
            completed_planned as f64 / total_planned as f64
        };

        let scope_creep_count = unapproved_tasks.len();
        let is_valid = scope_creep_count == 0 && dependency_violations.is_empty();

        DagMetrics {
            total_planned,
            completed_planned,
            coverage_ratio,
            scope_creep_count,
            unapproved_tasks,
            justified_explorations,
            dependency_violations,
            is_valid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_dag_execution() {
        let mut dag = PlanDag::new();
        dag.add_task("task1", "Database Setup", vec![]);
        dag.add_task("task2", "Backend API", vec!["task1".to_string()]);
        dag.add_task("task3", "Frontend UI", vec!["task2".to_string()]);

        assert!(dag.validate_graph().is_ok());

        assert!(dag.record_step("task1").is_ok());
        assert!(dag.record_step("task2").is_ok());
        assert!(dag.record_step("task3").is_ok());

        let metrics = dag.evaluate_metrics();
        assert_eq!(metrics.total_planned, 3);
        assert_eq!(metrics.completed_planned, 3);
        assert!((metrics.coverage_ratio - 1.0).abs() < f64::EPSILON);
        assert_eq!(metrics.scope_creep_count, 0);
        assert!(metrics.is_valid);
    }

    #[test]
    fn test_dependency_violation() {
        let mut dag = PlanDag::new();
        dag.add_task("t1", "Init", vec![]);
        dag.add_task("t2", "Build", vec!["t1".to_string()]);

        // Trying to run t2 before t1
        let res = dag.record_step("t2");
        assert!(res.is_err());
    }

    #[test]
    fn test_scope_creep_detection() {
        let mut dag = PlanDag::new();
        dag.add_task("t1", "Fix bug", vec![]);

        // Execute task not in plan
        let res = dag.record_step("t_unplanned_refactor");
        assert!(res.is_err());

        let metrics = dag.evaluate_metrics();
        assert_eq!(metrics.scope_creep_count, 1);
        assert!(!metrics.is_valid);
    }

    #[test]
    fn test_cycle_detection() {
        let mut dag = PlanDag::new();
        dag.add_task("a", "Task A", vec!["b".to_string()]);
        dag.add_task("b", "Task B", vec!["a".to_string()]);

        assert!(dag.validate_graph().is_err());
    }

    #[test]
    fn test_justified_discovery_step() {
        let mut dag = PlanDag::new();
        dag.add_task("t1", "Implement", vec![]);

        // Execute planned step
        assert!(dag.record_step("t1").is_ok());

        // Execute exploratory steps (read/view/check/test) not explicitly in plan
        assert!(dag.record_step("inspect_logs").is_ok());
        assert!(dag.record_step("view_file_summary").is_ok());
        assert!(dag.record_step("run_cargo_test").is_ok());

        let metrics = dag.evaluate_metrics();
        assert_eq!(metrics.scope_creep_count, 0, "Exploratory steps must not count as scope creep");
        assert_eq!(metrics.justified_explorations.len(), 3);
        assert!(metrics.is_valid);
    }
}
