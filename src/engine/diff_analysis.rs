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
    pub fn analyze(before: &str, after: &str, language: &str) -> DiffReport {
        let lines_before: Vec<&str> = before.lines().collect();
        let lines_after: Vec<&str> = after.lines().collect();
        let m = lines_before.len();
        let n = lines_after.len();

        let lcs_len = Self::lcs_length(&lines_before, &lines_after);

        let lines_added = n.saturating_sub(lcs_len);
        let lines_removed = m.saturating_sub(lcs_len);
        let lines_modified = lines_added.min(lines_removed);

        let change_ratio = (lines_added + lines_removed) as f64 / (m.max(1) as f64);

        let comp_before = Self::compute_complexity(before, language);
        let comp_after = Self::compute_complexity(after, language);

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

    fn lcs_length(a: &[&str], b: &[&str]) -> usize {
        if a.is_empty() || b.is_empty() {
            return 0;
        }

        // 1. Fast path: strip common prefix
        let mut start = 0;
        while start < a.len() && start < b.len() && a[start].trim() == b[start].trim() {
            start += 1;
        }
        let common_prefix = start;

        // 2. Fast path: strip common suffix
        let mut a_end = a.len();
        let mut b_end = b.len();
        while a_end > start && b_end > start && a[a_end - 1].trim() == b[b_end - 1].trim() {
            a_end -= 1;
            b_end -= 1;
        }
        let common_suffix = a.len() - a_end;

        let a_mid = &a[start..a_end];
        let b_mid = &b[start..b_end];

        if a_mid.is_empty() || b_mid.is_empty() {
            return common_prefix + common_suffix;
        }

        // 3. DoS Protection: Cap quadratic DP table
        if a_mid.len() * b_mid.len() > crate::engine::resource_limits::MAX_LCS_CELLS {
            // Greedy bounded matching for massive diffs to prevent CPU hang
            let mut matched = 0;
            let mut j = 0;
            for line_a in a_mid {
                let trimmed_a = line_a.trim();
                while j < b_mid.len() {
                    if trimmed_a == b_mid[j].trim() {
                        matched += 1;
                        j += 1;
                        break;
                    }
                    j += 1;
                }
            }
            return common_prefix + common_suffix + matched;
        }

        let (short, long) = if a_mid.len() <= b_mid.len() {
            (a_mid, b_mid)
        } else {
            (b_mid, a_mid)
        };
        let n = short.len();
        let mut prev = vec![0usize; n + 1];
        let mut curr = vec![0usize; n + 1];

        for long_line in long {
            let l_trimmed = long_line.trim();
            for (j, short_line) in short.iter().enumerate() {
                if l_trimmed == short_line.trim() {
                    curr[j + 1] = prev[j] + 1;
                } else {
                    curr[j + 1] = curr[j].max(prev[j + 1]);
                }
            }
            std::mem::swap(&mut prev, &mut curr);
            curr.fill(0);
        }
        common_prefix + common_suffix + prev[n]
    }

    fn compute_complexity(code: &str, language: &str) -> ComplexitySnapshot {
        if code.trim().is_empty() {
            return ComplexitySnapshot::default();
        }

        let m = crate::engine::CodeAnalyzer::analyze(code, language);
        ComplexitySnapshot {
            cyclomatic: m.cyclomatic_complexity,
            maintainability_index: m.maintainability_index,
            loc: m.lines_of_code,
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
                        let name: String = rest.chars().take_while(|&c| c.is_alphanumeric() || c == '_').collect();
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

    #[test]
    fn test_complete_deletion() {
        let before = "fn foo() {\n    let x = 1;\n}\n";
        let report = DiffAnalyzer::analyze(before, "", "rust");
        assert_eq!(report.lines_after, 0);
        assert!(report.lines_removed > 0);
        assert_eq!(report.lines_added, 0);
    }

    #[test]
    fn test_large_diff_efficiency() {
        let before = "let x = 1;\n".repeat(1000);
        let after = "let x = 2;\n".repeat(1000);
        let report = DiffAnalyzer::analyze(&before, &after, "rust");
        assert_eq!(report.lines_before, 1000);
        assert_eq!(report.lines_after, 1000);
    }

    #[test]
    fn test_diff_prefix_suffix_trimming() {
        let prefix = "common_prefix_line();\n".repeat(500);
        let suffix = "common_suffix_line();\n".repeat(500);
        let before = format!("{}let old_val = 1;\n{}", prefix, suffix);
        let after = format!("{}let new_val = 2;\n{}", prefix, suffix);
        let report = DiffAnalyzer::analyze(&before, &after, "rust");
        assert_eq!(report.lines_added, 1);
        assert_eq!(report.lines_removed, 1);
    }
}
