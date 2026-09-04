use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintReport {
    pub total_requirements: usize,
    pub satisfied_requirements: usize,
    pub missing_requirements: Vec<String>,
    pub scope_creep_items: Vec<String>,
    pub contradictions: Vec<String>,
    pub alignment_score: f64,
    pub is_aligned: bool,
}

pub struct ConstraintEngine;

impl ConstraintEngine {
    /// Compares required constraints against implementation claims and detects omissions,
    /// scope creep, and direct logical contradictions.
    pub fn verify(requirements: &[String], implementations: &[String]) -> ConstraintReport {
        let req_set: HashSet<String> = requirements.iter().map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect();
        let impl_set: HashSet<String> = implementations.iter().map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect();

        let mut satisfied_count = 0;
        let mut missing = Vec::new();

        for req in &req_set {
            // Check semantic or keyword containment
            let is_covered = impl_set.iter().any(|imp| imp.contains(req) || req.contains(imp));
            if is_covered {
                satisfied_count += 1;
            } else {
                missing.push(req.clone());
            }
        }

        let mut scope_creep = Vec::new();
        for imp in &impl_set {
            let is_demanded = req_set.iter().any(|req| imp.contains(req) || req.contains(imp));
            if !is_demanded {
                scope_creep.push(imp.clone());
            }
        }

        // Mutual exclusivity / Contradiction checks (P and not P)
        let contradictions = Self::detect_contradictions(implementations);

        let total_requirements = req_set.len();
        let alignment_score = if total_requirements == 0 {
            1.0
        } else {
            satisfied_count as f64 / total_requirements as f64
        };

        let is_aligned = missing.is_empty() && contradictions.is_empty() && scope_creep.is_empty();

        ConstraintReport {
            total_requirements,
            satisfied_requirements: satisfied_count,
            missing_requirements: missing,
            scope_creep_items: scope_creep,
            contradictions,
            alignment_score,
            is_aligned,
        }
    }

    fn detect_contradictions(items: &[String]) -> Vec<String> {
        let mut contradictions = Vec::new();
        let joined = items.join(" ").to_lowercase();

        let contradiction_pairs = [
            ("do not run local build", "cargo build"),
            ("no local build", "cargo test"),
            ("public repo", "private repo"),
            ("pure rust", "node c++ addon"),
            ("zero dependencies", "use petgraph"),
        ];

        for (a, b) in contradiction_pairs {
            if joined.contains(a) && joined.contains(b) {
                contradictions.push(format!(
                    "Logical Contradiction (P ∧ ¬P): Contains mutually exclusive directives '{}' and '{}'",
                    a, b
                ));
            }
        }

        contradictions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_alignment() {
        let reqs = vec!["Rust language".to_string(), "DAG plan".to_string()];
        let impls = vec!["Implemented in Rust language".to_string(), "DAG plan engine completed".to_string()];

        let report = ConstraintEngine::verify(&reqs, &impls);
        assert_eq!(report.satisfied_requirements, 2);
        assert!(report.missing_requirements.is_empty());
        assert!(report.is_aligned);
    }

    #[test]
    fn test_missing_and_creep() {
        let reqs = vec!["Authentication".to_string(), "Database".to_string()];
        let impls = vec!["Authentication added".to_string(), "Unrequested AI blockchain added".to_string()];

        let report = ConstraintEngine::verify(&reqs, &impls);
        assert_eq!(report.satisfied_requirements, 1);
        assert_eq!(report.missing_requirements.len(), 1);
        assert_eq!(report.scope_creep_items.len(), 1);
        assert!(!report.is_aligned);
    }

    #[test]
    fn test_contradiction_detection() {
        let reqs = vec!["Rule".to_string()];
        let impls = vec!["Strict rule: no local build, but running cargo build anyway".to_string()];

        let report = ConstraintEngine::verify(&reqs, &impls);
        assert!(!report.contradictions.is_empty());
    }
}
