use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMetrics {
    pub word_count: usize,
    pub sentence_count: usize,
    pub unique_words: usize,
    pub type_token_ratio: f64,
    pub shannon_entropy_bits: f64,
    pub compression_ratio: f64,
    pub flesch_reading_ease: f64,
    pub gunning_fog_index: f64,
    pub information_density: f64,
    pub is_verbose: bool,
    pub is_too_complex: bool,
    pub suggestions: Vec<String>,
}

pub struct TextEvaluator;

impl TextEvaluator {
    /// Evaluates information theory and readability metrics on given text.
    pub fn evaluate(text: &str) -> TextMetrics {
        let cleaned = text.trim();
        if cleaned.is_empty() {
            return TextMetrics {
                word_count: 0,
                sentence_count: 0,
                unique_words: 0,
                type_token_ratio: 0.0,
                shannon_entropy_bits: 0.0,
                compression_ratio: 0.0,
                flesch_reading_ease: 100.0,
                gunning_fog_index: 0.0,
                information_density: 0.0,
                is_verbose: false,
                is_too_complex: false,
                suggestions: vec!["Text is empty".to_string()],
            };
        }

        let words: Vec<&str> = cleaned
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| !w.is_empty())
            .collect();

        let word_count = words.len();
        if word_count == 0 {
            return TextMetrics {
                word_count: 0,
                sentence_count: 0,
                unique_words: 0,
                type_token_ratio: 0.0,
                shannon_entropy_bits: 0.0,
                compression_ratio: 0.0,
                flesch_reading_ease: 100.0,
                gunning_fog_index: 0.0,
                information_density: 0.0,
                is_verbose: false,
                is_too_complex: false,
                suggestions: vec!["No valid words found".to_string()],
            };
        }

        let sentence_count = cleaned
            .split(['.', '!', '?', '\n'])
            .filter(|s| !s.trim().is_empty())
            .count()
            .max(1);

        // Word frequency & Shannon entropy
        let mut freq_map = HashMap::new();
        let mut total_syllables = 0;
        let mut complex_words = 0;

        for word in &words {
            let lower = word.to_lowercase();
            *freq_map.entry(lower.clone()).or_insert(0usize) += 1;

            let syl = Self::count_syllables(&lower);
            total_syllables += syl;
            if syl >= 3 {
                complex_words += 1;
            }
        }

        let unique_words = freq_map.len();
        let type_token_ratio = unique_words as f64 / word_count as f64;

        // Shannon Entropy: H(X) = - sum p(x) * log2(p(x))
        let mut shannon_entropy_bits = 0.0;
        for &count in freq_map.values() {
            let p = count as f64 / word_count as f64;
            if p > 0.0 {
                shannon_entropy_bits -= p * p.log2();
            }
        }

        // Compression ratio via gzip (Kolmogorov complexity approximation)
        let compression_ratio = Self::estimate_compression_ratio(cleaned);

        // Flesch Reading Ease: 206.835 - 1.015 * (words / sentences) - 84.6 * (syllables / words)
        let words_per_sentence = word_count as f64 / sentence_count as f64;
        let syllables_per_word = total_syllables as f64 / word_count as f64;
        let flesch_reading_ease =
            206.835 - (1.015 * words_per_sentence) - (84.6 * syllables_per_word);

        // Gunning Fog Index: 0.4 * ((words / sentences) + 100 * (complex_words / words))
        let complex_ratio = complex_words as f64 / word_count as f64;
        let gunning_fog_index = 0.4 * (words_per_sentence + (100.0 * complex_ratio));

        // Information Density = Entropy * TTR
        let information_density = shannon_entropy_bits * type_token_ratio;

        let mut suggestions = Vec::new();
        let is_verbose = word_count > 450 && (type_token_ratio < 0.45 || compression_ratio < 0.35);
        if is_verbose {
            suggestions.push(
                "Text exhibits high redundancy / filler tokens. Consider condensing explanations."
                    .to_string(),
            );
        }

        let is_too_complex = flesch_reading_ease < 30.0 || gunning_fog_index > 17.0;
        if is_too_complex {
            suggestions.push(
                "Text is overly dense, academic, or hard to parse. Shorten sentences and simplify syntax.".to_string(),
            );
        }

        if type_token_ratio < 0.35 && word_count > 100 {
            suggestions.push("Low vocabulary variety: Repeated phrasing detected.".to_string());
        }

        TextMetrics {
            word_count,
            sentence_count,
            unique_words,
            type_token_ratio,
            shannon_entropy_bits,
            compression_ratio,
            flesch_reading_ease,
            gunning_fog_index,
            information_density,
            is_verbose,
            is_too_complex,
            suggestions,
        }
    }

    fn count_syllables(word: &str) -> usize {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        let mut count = 0;
        let mut prev_is_vowel = false;

        for c in word.chars() {
            let is_vowel = vowels.contains(&c);
            if is_vowel && !prev_is_vowel {
                count += 1;
            }
            prev_is_vowel = is_vowel;
        }

        // Silent 'e' at the end
        if word.ends_with('e') && count > 1 {
            count -= 1;
        }

        count.max(1)
    }

    fn estimate_compression_ratio(text: &str) -> f64 {
        let original_bytes = text.as_bytes();
        if original_bytes.is_empty() {
            return 1.0;
        }

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        if encoder.write_all(original_bytes).is_err() {
            return 1.0;
        }

        match encoder.finish() {
            Ok(compressed) => compressed.len() as f64 / original_bytes.len() as f64,
            Err(_) => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_evaluator_basic() {
        let sample = "The quick brown fox jumps over the lazy dog. It was a bright sunny day in the green forest.";
        let metrics = TextEvaluator::evaluate(sample);

        assert!(metrics.word_count > 10);
        assert!(metrics.unique_words > 10);
        assert!(metrics.shannon_entropy_bits > 0.0);
        assert!(metrics.type_token_ratio > 0.5);
        assert!(!metrics.is_verbose);
    }

    #[test]
    fn test_highly_repetitive_text() {
        let repetitive =
            "test test test test test test test test test test test test test test test test "
                .repeat(30);
        let metrics = TextEvaluator::evaluate(&repetitive);

        assert!(metrics.type_token_ratio < 0.1);
        assert!(metrics.is_verbose);
        assert!(!metrics.suggestions.is_empty());
    }

    #[test]
    fn test_empty_string() {
        let metrics = TextEvaluator::evaluate("");
        assert_eq!(metrics.word_count, 0);
    }
}
