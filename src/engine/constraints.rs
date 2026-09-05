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
        let req_set: HashSet<String> = requirements
            .iter()
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        let impl_set: HashSet<String> = implementations
            .iter()
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        let mut satisfied_count = 0;
        let mut missing = Vec::new();

        for req in &req_set {
            // Check semantic or keyword containment
            let is_covered = impl_set
                .iter()
                .any(|imp| Self::is_semantically_matched(req, imp));
            if is_covered {
                satisfied_count += 1;
            } else {
                missing.push(req.clone());
            }
        }

        let mut scope_creep = Vec::new();
        for imp in &impl_set {
            // Ignore long descriptive sentences or paragraphs from being flagged as action scope creep
            if imp.len() > 60 || imp.split_whitespace().count() > 8 {
                continue;
            }
            let is_demanded = req_set
                .iter()
                .any(|req| Self::is_semantically_matched(req, imp));
            if !is_demanded {
                scope_creep.push(imp.clone());
            }
        }

        // Mutual exclusivity / Contradiction checks (P and not P)
        let all_items: Vec<&str> = requirements
            .iter()
            .chain(implementations.iter())
            .map(|s| s.as_str())
            .collect();
        let contradictions = Self::detect_contradictions(&all_items);

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

    fn is_semantically_matched(req: &str, imp: &str) -> bool {
        let req_clean = req.trim().to_lowercase();
        let imp_clean = imp.trim().to_lowercase();

        if imp_clean.contains(&req_clean) || req_clean.contains(&imp_clean) {
            return true;
        }

        let jaccard = Self::char_ngram_jaccard(&req_clean, &imp_clean, 2);
        let word_overlap = Self::word_overlap_ratio(&req_clean, &imp_clean);

        let stop_words = [
            "must", "be", "in", "the", "a", "an", "for", "to", "of", "with", "and", "by", "all",
            "is", "use", "using", "support", "supports", "added", "implemented", "have", "has",
            // Vietnamese stop words
            "và", "là", "của", "trong", "cho", "với", "để", "các", "những", "phải", "được", "có",
            "đã", "sử", "dụng", "thêm",
        ];
        let key_words_req: Vec<&str> = req_clean
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| !w.is_empty() && !stop_words.contains(w))
            .collect();

        if key_words_req.is_empty() {
            return jaccard >= 0.45 || word_overlap >= 0.6;
        }

        let key_words_imp: Vec<&str> = imp_clean
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| !w.is_empty() && !stop_words.contains(w))
            .collect();

        // Entity Substitution Guard:
        // Detect when an essential requirement entity is replaced by an unrequested entity (e.g. "secrets" -> "logs")
        let unfulfilled: Vec<&&str> = key_words_req.iter().filter(|&&w| !imp_clean.contains(w)).collect();
        let extra: Vec<&&str> = key_words_imp.iter().filter(|&&w| !req_clean.contains(w)).collect();
        let has_entity_divergence = !unfulfilled.is_empty()
            && !extra.is_empty()
            && unfulfilled.iter().any(|w| w.len() >= 4);

        if has_entity_divergence && key_words_req.len() <= 6 {
            return false;
        }

        let matched_count = key_words_req.iter().filter(|&&w| imp_clean.contains(w)).count();
        let key_match_ratio = matched_count as f64 / key_words_req.len() as f64;

        let key_match = if key_words_req.len() >= 3 {
            key_match_ratio >= 0.75 && matched_count >= (key_words_req.len() - 1)
        } else if key_words_req.len() == 2 {
            matched_count == 2
        } else {
            matched_count == 1
        };

        (key_match && (jaccard >= 0.35 || word_overlap >= 0.40)) || jaccard >= 0.65 || word_overlap >= 0.70
    }

    fn char_ngram_jaccard(a: &str, b: &str, n: usize) -> f64 {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        if a_chars.len() < n || b_chars.len() < n {
            return if a == b { 1.0 } else { 0.0 };
        }

        let mut set_a = HashSet::new();
        for window in a_chars.windows(n) {
            set_a.insert(window.iter().collect::<String>());
        }

        let mut set_b = HashSet::new();
        for window in b_chars.windows(n) {
            set_b.insert(window.iter().collect::<String>());
        }

        let intersection = set_a.intersection(&set_b).count();
        let union = set_a.union(&set_b).count();
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    fn word_overlap_ratio(a: &str, b: &str) -> f64 {
        let words_a: HashSet<&str> = a.split_whitespace().collect();
        let words_b: HashSet<&str> = b.split_whitespace().collect();
        if words_a.is_empty() || words_b.is_empty() {
            return 0.0;
        }
        let intersection = words_a.intersection(&words_b).count();
        let min_len = words_a.len().min(words_b.len());
        intersection as f64 / min_len as f64
    }

    fn detect_contradictions(items: &[&str]) -> Vec<String> {
        let mut contradictions = Vec::new();
        let joined = items.join(" ").to_lowercase();

        let contradiction_pairs = [
            ("no local build", "cargo build"),
            ("no local build", "cargo test"),
            ("do not run local build", "cargo build"),
            ("do not run local build", "cargo test"),
            ("public repo", "private repo"),
            ("pure rust", "node c++ addon"),
            ("zero dependencies", "use petgraph"),
            // Vietnamese contradiction pairs
            ("không build local", "cargo build"),
            ("không test local", "cargo test"),
            ("không chạy local", "cargo build"),
        ];

        for (a, b) in contradiction_pairs {
            if joined.contains(a) && joined.contains(b) {
                contradictions.push(format!(
                    "Logical Contradiction (P ∧ ¬P): Contains mutually exclusive directives '{}' and '{}'",
                    a, b
                ));
            }
        }

        // Dynamic negation pattern detection
        const NEGATION_PREFIXES: &[&str] = &[
            "no ",
            "not ",
            "don't ",
            "never ",
            "without ",
            "disable ",
            "must not ",
            "do not ",
            "cannot ",
            "shouldn't ",
            "không ",
            "chưa ",
            "đừng ",
            "tuyệt đối không ",
        ];

        for item in items {
            let lower = item.to_lowercase();
            for &neg in NEGATION_PREFIXES {
                if let Some(idx) = lower.find(neg) {
                    let concept = lower[idx + neg.len()..]
                        .split(['.', ',', ';', '!', '?', '\n'])
                        .next()
                        .unwrap_or("")
                        .trim();
                    let concept_words: Vec<&str> = concept.split_whitespace().take(4).collect();
                    let key_concept = concept_words.join(" ");
                    if key_concept.len() >= 4 {
                        for other in items {
                            let other_lower = other.to_lowercase();
                            if !other_lower.contains(neg) && other_lower.contains(&key_concept) {
                                let c_msg = format!(
                                    "Dynamic Contradiction (P ∧ ¬P): Negated concept '{}' conflicts with assertion in '{}'",
                                    key_concept, other.trim()
                                );
                                if !contradictions.contains(&c_msg) {
                                    contradictions.push(c_msg);
                                }
                            }
                        }
                    }
                }
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
        let impls = vec![
            "Implemented in Rust language".to_string(),
            "DAG plan engine completed".to_string(),
        ];

        let report = ConstraintEngine::verify(&reqs, &impls);
        assert_eq!(report.satisfied_requirements, 2);
        assert!(report.missing_requirements.is_empty());
        assert!(report.is_aligned);
    }

    #[test]
    fn test_semantic_matching() {
        let reqs = vec!["must use rust".to_string()];
        let impls = vec!["implemented in rust language".to_string()];

        let report = ConstraintEngine::verify(&reqs, &impls);
        assert_eq!(report.satisfied_requirements, 1);
        assert!(report.missing_requirements.is_empty());
    }

    #[test]
    fn test_missing_and_creep() {
        let reqs = vec!["Authentication".to_string(), "Database".to_string()];
        let impls = vec![
            "Authentication added".to_string(),
            "Unrequested AI blockchain added".to_string(),
        ];

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

    #[test]
    fn test_dynamic_contradiction() {
        let reqs = vec!["Rule: without external api calls".to_string()];
        let impls = vec!["Implemented by calling external api calls directly".to_string()];

        let report = ConstraintEngine::verify(&reqs, &impls);
        assert!(!report.contradictions.is_empty());
    }

    #[test]
    fn test_vietnamese_semantic_matching() {
        let reqs = vec!["phải sử dụng ngôn ngữ rust".to_string()];
        let impls = vec!["đã sử dụng ngôn ngữ rust trong toàn bộ dự án".to_string()];

        let report = ConstraintEngine::verify(&reqs, &impls);
        assert_eq!(report.satisfied_requirements, 1);
        assert!(report.is_aligned);
    }

    #[test]
    fn test_vietnamese_contradiction() {
        let reqs = vec!["Quy tắc: tuyệt đối không build local".to_string()];
        let impls = vec!["Đang chạy cargo build trên máy local".to_string()];

        let report = ConstraintEngine::verify(&reqs, &impls);
        assert!(!report.contradictions.is_empty());
    }

    #[test]
    fn test_entity_substitution_rejected() {
        let reqs = vec!["encrypt all customer secrets at rest".to_string()];
        let impls = vec!["encrypt customer logs at rest".to_string()];

        let report = ConstraintEngine::verify(&reqs, &impls);
        assert_eq!(report.satisfied_requirements, 0);
        assert_eq!(report.missing_requirements.len(), 1);
        assert!(!report.is_aligned);
    }
}
