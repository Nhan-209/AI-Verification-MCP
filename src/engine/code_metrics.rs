use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tree_sitter::{Node, Parser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalsteadMetrics {
    pub distinct_operators: usize,
    pub distinct_operands: usize,
    pub total_operators: usize,
    pub total_operands: usize,
    pub program_vocabulary: usize,
    pub program_length: usize,
    pub volume: f64,
    pub difficulty: f64,
    pub effort: f64,
    pub estimated_bugs: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub language: String,
    pub lines_of_code: usize,
    pub cyclomatic_complexity: usize,
    pub maintainability_index: f64,
    pub halstead: HalsteadMetrics,
    pub has_syntax_errors: bool,
    pub syntax_error_count: usize,
    pub boundary_warnings: Vec<String>,
    pub is_production_ready: bool,
}

pub struct CodeAnalyzer;

impl CodeAnalyzer {
    /// Analyzes source code using AST parsing (when available) and mathematical metrics.
    pub fn analyze(code: &str, lang_hint: &str) -> CodeMetrics {
        let lines: Vec<&str> = code.lines().collect();
        let loc = lines.iter().filter(|l| !l.trim().is_empty()).count().max(1);

        let (mut cyclomatic, syntax_errors) = Self::ast_analyze(code, lang_hint);
        if cyclomatic == 0 {
            cyclomatic = Self::heuristic_cyclomatic(code);
        }

        let halstead = Self::calculate_halstead(code);

        // Maintainability Index (MI) formula:
        // MI = 171 - 5.2 * ln(V) - 0.23 * M - 16.2 * ln(LOC)
        let v = if halstead.volume > 0.0 { halstead.volume } else { 1.0 };
        let m = cyclomatic as f64;
        let l = loc as f64;
        let raw_mi = 171.0 - (5.2 * v.ln()) - (0.23 * m) - (16.2 * l.ln());
        let normalized_mi = ((raw_mi * 100.0) / 171.0).clamp(0.0, 100.0);

        let boundary_warnings = Self::check_boundary_conditions(code, lang_hint);

        let is_production_ready = syntax_errors == 0
            && normalized_mi >= 55.0
            && cyclomatic <= 25
            && boundary_warnings.is_empty();

        CodeMetrics {
            language: lang_hint.to_string(),
            lines_of_code: loc,
            cyclomatic_complexity: cyclomatic,
            maintainability_index: normalized_mi,
            halstead,
            has_syntax_errors: syntax_errors > 0,
            syntax_error_count: syntax_errors,
            boundary_warnings,
            is_production_ready,
        }
    }

    fn ast_analyze(code: &str, lang: &str) -> (usize, usize) {
        let mut parser = Parser::new();
        let lang_lower = lang.to_lowercase();

        let lang_supported = match lang_lower.as_str() {
            "rust" | "rs" => parser.set_language(&tree_sitter_rust::LANGUAGE.into()).is_ok(),
            "typescript" | "ts" | "tsx" | "javascript" | "js" => {
                parser
                    .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
                    .is_ok()
            }
            "python" | "py" => parser.set_language(&tree_sitter_python::LANGUAGE.into()).is_ok(),
            _ => false,
        };

        if !lang_supported {
            return (0, 0);
        }

        if let Some(tree) = parser.parse(code, None) {
            let root = tree.root_node();
            let mut errors = 0;
            let mut decisions = 1; // Base complexity = 1

            Self::traverse_ast(root, &mut decisions, &mut errors);
            (decisions, errors)
        } else {
            (0, 0)
        }
    }

    fn traverse_ast(node: Node, decisions: &mut usize, errors: &mut usize) {
        if node.is_error() || node.is_missing() {
            *errors += 1;
        }

        let kind = node.kind();
        // Common decision branch kinds across Rust, TS, Python
        if kind.contains("if")
            || kind.contains("while")
            || kind.contains("for")
            || kind.contains("match_arm")
            || kind.contains("case")
            || kind.contains("catch")
            || kind == "&&"
            || kind == "||"
            || kind == "ternary_expression"
            || kind == "conditional_expression"
        {
            *decisions += 1;
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::traverse_ast(child, decisions, errors);
        }
    }

    fn heuristic_cyclomatic(code: &str) -> usize {
        let mut count = 1;
        let patterns = [
            "if ", "else if ", "for ", "while ", "match ", "case ", "catch ", "&&", "||", "?",
        ];
        for word in patterns {
            count += code.matches(word).count();
        }
        count
    }

    fn calculate_halstead(code: &str) -> HalsteadMetrics {
        let operators_list = [
            "+", "-", "*", "/", "%", "=", "==", "!=", "<", ">", "<=", ">=", "&&", "||", "!", "&",
            "|", "^", "<<", ">>", "+=", "-=", "*=", "/=", "->", "=>", "::", ".", ",", ";", "(",
            ")", "[", "]", "{", "}",
        ];

        let mut distinct_operators = HashSet::new();
        let mut distinct_operands = HashSet::new();
        let mut total_operators = 0;
        let mut total_operands = 0;

        let tokens: Vec<&str> = code.split_whitespace().collect();

        for token in tokens {
            if operators_list.contains(&token) {
                distinct_operators.insert(token.to_string());
                total_operators += 1;
            } else {
                distinct_operands.insert(token.to_string());
                total_operands += 1;
            }
        }

        let n1 = distinct_operators.len();
        let n2 = distinct_operands.len();
        let big_n1 = total_operators;
        let big_n2 = total_operands;

        let vocab = n1 + n2;
        let length = big_n1 + big_n2;

        let volume = if vocab > 0 {
            length as f64 * (vocab as f64).log2()
        } else {
            0.0
        };

        let difficulty = if n2 > 0 {
            (n1 as f64 / 2.0) * (big_n2 as f64 / n2 as f64)
        } else {
            0.0
        };

        let effort = difficulty * volume;
        let estimated_bugs = volume / 3000.0;

        HalsteadMetrics {
            distinct_operators: n1,
            distinct_operands: n2,
            total_operators: big_n1,
            total_operands: big_n2,
            program_vocabulary: vocab,
            program_length: length,
            volume,
            difficulty,
            effort,
            estimated_bugs,
        }
    }

    fn check_boundary_conditions(code: &str, lang: &str) -> Vec<String> {
        let mut warnings = Vec::new();
        let lang_lower = lang.to_lowercase();

        if lang_lower == "rust" || lang_lower == "rs" {
            let unwrap_count = code.matches(".unwrap()").count();
            if unwrap_count > 0 {
                warnings.push(format!(
                    "Found {} instance(s) of .unwrap() without explicit error handling or ? operator",
                    unwrap_count
                ));
            }
            if code.contains("unsafe ") {
                warnings.push("Found 'unsafe' block: verify invariant guarantees".to_string());
            }
        }

        if lang_lower.contains("python") || lang_lower == "py" {
            if code.contains("except:") {
                warnings.push("Bare 'except:' found. Catch specific exceptions instead.".to_string());
            }
        }

        if lang_lower.contains("script") || lang_lower == "ts" || lang_lower == "js" {
            if code.contains("any") {
                warnings.push("Potential type hole: 'any' type keyword detected.".to_string());
            }
        }

        // Universal check: unchecked index access [0] without length verification
        if code.contains("[0]")
            && !code.contains(".is_empty()")
            && !code.contains(".len()")
            && !code.contains(".length")
        {
            warnings.push(
                "Potential IndexOutOfBounds / Panic: '[0]' used without apparent length check."
                    .to_string(),
            );
        }

        warnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_code_metrics() {
        let rust_code = r#"
            pub fn add(a: i32, b: i32) -> i32 {
                if a > 0 {
                    a + b
                } else {
                    b
                }
            }
        "#;
        let metrics = CodeAnalyzer::analyze(rust_code, "rust");
        assert_eq!(metrics.language, "rust");
        assert!(metrics.cyclomatic_complexity >= 2);
        assert!(!metrics.has_syntax_errors);
        assert!(metrics.maintainability_index > 60.0);
    }

    #[test]
    fn test_boundary_warning() {
        let risky_code = r#"
            let val = my_list[0];
            let item = some_option.unwrap();
        "#;
        let metrics = CodeAnalyzer::analyze(risky_code, "rust");
        assert!(!metrics.boundary_warnings.is_empty());
    }
}
