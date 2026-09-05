use serde::{Deserialize, Serialize};

/// Report generated after computing code differences and regression risk.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiffReport {
    pub lines_before: usize,
    pub lines_after: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub lines_modified: usize,
    pub change_ratio: f64,
    pub complexity_before: ComplexitySnapshot,
    pub complexity_after: ComplexitySnapshot,
    pub complexity_delta: ComplexityDelta,
    pub functions_affected: Vec<String>,
    pub regression_risk: f64,
    pub risk_level: String,
    pub suggestions: Vec<String>,
}

/// Snapshot of complexity metrics for a piece of code.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ComplexitySnapshot {
    pub cyclomatic: usize,
    pub maintainability_index: f64,
    pub loc: usize,
}

/// Differences in complexity metrics between two code snapshots.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ComplexityDelta {
    pub cyclomatic: i64,
    pub maintainability: f64,
    pub loc: i64,
}

/// A unit struct containing methods to analyze code diffs.
pub struct DiffAnalyzer;

impl DiffAnalyzer {
    /// Analyzes the difference between `before` and `after` code strings, computing metrics and regression risk.
    pub fn analyze(before: &str, after: &str, _language: &str) -> DiffReport {
        let lines_before: Vec<&str> = before.lines().collect();
        let lines_after: Vec<&str> = after.lines().collect();
        let m = lines_before.len();
        let n = lines_after.len();

        let lcs_dp = Self::lcs_length(&lines_before, &lines_after);
        let lcs_len = lcs_dp[m][n];

        let lines_added = n.saturating_sub(lcs_len);
        let lines_removed = m.saturating_sub(lcs_len);
        let lines_modified = lines_added.min(lines_removed);

        let change_ratio = (lines_added + lines_removed) as f64 / (m.max(1) as f64);

        let comp_before = Self::compute_complexity(before);
        let comp_after = Self::compute_complexity(after);

        let delta = ComplexityDelta {
            cyclomatic: comp_after.cyclomatic as i64 - comp_before.cyclomatic as i64,
            maintainability: comp_after.maintainability_index - comp_before.maintainability_index,
            loc: comp_after.loc as i64 - comp_before.loc as i64,
        };

        let functions_affected = Self::detect_functions(after);

        let mut suggestions = Vec::new();

        let change_risk = (change_ratio * 40.0).min(40.0);
        let complexity_risk = if delta.cyclomatic > 0 {
            (delta.cyclomatic as f64 * 5.0).min(30.0)
        } else {
            0.0
        };
        let mi_risk = if delta.maintainability < 0.0 {
            (-delta.maintainability).min(20.0)
        } else {
            0.0
        };
        let scope_risk = (functions_affected.len() as f64 * 3.0).min(10.0);

        let regression_risk = (change_risk + complexity_risk + mi_risk + scope_risk).min(100.0);

        let risk_level = if regression_risk < 30.0 {
            "LOW".to_string()
        } else if regression_risk <= 60.0 {
            "MEDIUM".to_string()
        } else {
            "HIGH".to_string()
        };

        if change_ratio > 0.5 {
            suggestions.push("Large change scope: Consider breaking into smaller PRs".to_string());
        }
        if delta.cyclomatic > 5 {
            suggestions.push("Significant complexity increase: Consider refactoring".to_string());
        }
        if delta.maintainability < -10.0 {
            suggestions.push("Maintainability degradation: Review code structure".to_string());
        }
        if functions_affected.len() > 5 {
            suggestions.push("Wide blast radius: Many functions affected".to_string());
        }

        DiffReport {
            lines_before: m,
            lines_after: n,
            lines_added,
            lines_removed,
            lines_modified,
            change_ratio,
            complexity_before: comp_before,
            complexity_after: comp_after,
            complexity_delta: delta,
            functions_affected,
            regression_risk,
            risk_level,
            suggestions,
        }
    }

    fn lcs_length(a: &[&str], b: &[&str]) -> Vec<Vec<usize>> {
        let m = a.len();
        let n = b.len();
        let mut dp = vec![vec![0usize; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if a[i - 1].trim() == b[j - 1].trim() {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp
    }

    fn compute_complexity(code: &str) -> ComplexitySnapshot {
        let loc = code.lines().count();
        let mut cyclomatic = 1;

        let tokens = [
            "if ", "for ", "while ", "match ", "case ", "catch ", "&&", "||", "?",
        ];
        for line in code.lines() {
            let t_line = line.trim();
            for &token in &tokens {
                if t_line.contains(token) {
                    cyclomatic += 1;
                }
            }
        }

        let mi = (100.0 - (cyclomatic as f64 * 3.0) - (loc as f64 / 10.0)).max(0.0);

        ComplexitySnapshot {
            cyclomatic,
            maintainability_index: mi,
            loc,
        }
    }

    fn detect_functions(code: &str) -> Vec<String> {
        let mut funcs = Vec::new();
        let prefixes = ["fn ", "def ", "func ", "function "];
        for line in code.lines() {
            let t = line.trim();
            for &prefix in &prefixes {
                if t.starts_with(prefix) {
                    if let Some(rest) = t.strip_prefix(prefix) {
                        let name: String = rest
                            .chars()
                            .take_while(|&c| c.is_alphanumeric() || c == '_')
                            .collect();
                        if !name.is_empty() {
                            funcs.push(name);
                        }
                    }
                }
            }
        }
        funcs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_change() {
        let code = "fn main() {\n    println!(\"hello\");\n}";
        let report = DiffAnalyzer::analyze(code, code, "rust");
        assert_eq!(report.change_ratio, 0.0);
        assert_eq!(report.risk_level, "LOW");
        assert_eq!(report.lines_added, 0);
    }

    #[test]
    fn test_simple_addition() {
        let before = "fn main() {\n    println!(\"hello\");\n}";
        let after = "fn main() {\n    println!(\"hello\");\n    println!(\"world\");\n}";
        let report = DiffAnalyzer::analyze(before, after, "rust");
        assert!(report.change_ratio > 0.0);
        assert_eq!(report.lines_added, 1);
        assert_eq!(report.risk_level, "LOW");
    }

    #[test]
    fn test_major_refactor() {
        let before = "fn old() { /* ... */ }";
        let after = "fn new() {\n    if true {\n        println!(\"hi\");\n    }\n}";
        let report = DiffAnalyzer::analyze(before, after, "rust");
        assert!(report.change_ratio > 0.5);
        assert!(report.regression_risk > 20.0);
        assert!(report.functions_affected.contains(&"new".to_string()));
    }

    #[test]
    fn test_function_detection() {
        let code = "fn foo() {}\nfn bar() {}\ndef python_func():\nfunc go_func() {";
        let report = DiffAnalyzer::analyze("", code, "any");
        assert!(report.functions_affected.contains(&"foo".to_string()));
        assert!(report.functions_affected.contains(&"bar".to_string()));
        assert!(report.functions_affected.contains(&"python_func".to_string()));
        assert!(report.functions_affected.contains(&"go_func".to_string()));
    }

    #[test]
    fn test_empty_before() {
        let after = "fn main() {}";
        let report = DiffAnalyzer::analyze("", after, "rust");
        assert_eq!(report.lines_before, 0);
        assert_eq!(report.lines_added, 1);
        assert_eq!(report.lines_removed, 0);
    }
}
