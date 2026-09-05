//! Shared text utilities for smart sentence segmentation and bilingual (EN/VI) semantic keyword matching.

/// Common abbreviations in English and Vietnamese that should not trigger sentence boundaries.
pub const COMMON_ABBREVIATIONS: &[&str] = &[
    "e.g.", "i.e.", "vs.", "etc.", "approx.", "dept.", "fig.", "prof.", "dr.", "mr.", "mrs.", "vd.", "tp.", "th.s",
    "ts.", "ths.", "bs.", "ks.",
];

/// English and Vietnamese hedging phrases expressing uncertainty or evasiveness.
pub const HEDGING_PHRASES: &[&str] = &[
    // English
    "maybe",
    "probably",
    "i think",
    "might",
    "could be",
    "not sure",
    "perhaps",
    "possibly",
    "it seems",
    "i believe",
    "i guess",
    "it appears",
    "likely",
    "unlikely",
    "not certain",
    "unclear",
    "debatable",
    "in my opinion",
    "it depends",
    "hard to say",
    // Vietnamese
    "có lẽ",
    "tôi nghĩ",
    "chắc là",
    "hình như",
    "dường như",
    "có thể là",
    "không chắc",
    "chưa rõ",
    "tùy thuộc",
    "khó nói",
    "theo tôi thấy",
    "dường như là",
    "phỏng đoán",
];

/// English and Vietnamese absolute or overconfident claims.
pub const OVERCONFIDENCE_PHRASES: &[&str] = &[
    // English
    "guaranteed",
    "100%",
    "definitely",
    "always",
    "never fails",
    "flawless",
    "completely impossible",
    "undeniably",
    "absolute truth",
    "zero bugs",
    "foolproof",
    "perfect solution",
    // Vietnamese
    "chắc chắn 100%",
    "đảm bảo tuyệt đối",
    "hoàn hảo",
    "không bao giờ lỗi",
    "không thể sai",
    "chắc chắn luôn",
    "tuyệt đối đúng",
    "hoàn toàn không có lỗi",
    "cam kết 100%",
];

/// English and Vietnamese conversational AI filler phrases.
pub const AI_FILLER_PHRASES: &[&str] = &[
    // English
    "as an ai",
    "i'd be happy to",
    "let me explain",
    "certainly!",
    "of course!",
    "great question",
    "i understand",
    "absolutely",
    "sure thing",
    "here's what i",
    "i'll help you",
    "let me help",
    "in conclusion",
    "to summarize",
    // Vietnamese
    "với tư cách là ai",
    "tôi rất vui được",
    "để tôi giải thích",
    "chắc chắn rồi!",
    "câu hỏi rất hay",
    "tôi hiểu rồi",
    "dưới đây là",
    "đây là những gì tôi",
    "tóm lại là",
];

/// English and Vietnamese markers indicating factual evidence or citations.
pub const EVIDENCE_MARKERS: &[&str] = &[
    // English
    "rfc",
    "ieee",
    "iso",
    "documentation",
    "docs.rs",
    "github.com",
    "benchmark",
    "commit",
    "verified via",
    "tested with",
    "log output",
    "reference:",
    "source:",
    "according to",
    // Vietnamese
    "tài liệu",
    "trích dẫn",
    "theo chuẩn",
    "đã kiểm thử",
    "kết quả đo",
    "nhật ký lỗi",
    "nguồn:",
];

/// English and Vietnamese markers indicating proactive foresight (edge cases, error handling).
pub const FORESIGHT_MARKERS: &[&str] = &[
    // English
    "error handling",
    "edge case",
    "boundary condition",
    "fallback",
    "timeout",
    "retry logic",
    "race condition",
    "graceful degradation",
    "backward compatibility",
    "migration path",
    "unit test",
    "integration test",
    "panic recovery",
    // Vietnamese
    "xử lý lỗi",
    "trường hợp biên",
    "ngoại lệ",
    "dự phòng",
    "hết thời gian",
    "thử lại",
    "tương thích ngược",
    "kiểm thử đơn vị",
    "kế hoạch di chuyển",
    "phục hồi lỗi",
];

/// Splits text into clean sentences without shredding URLs, version tags, decimals, or abbreviations.
pub fn smart_split_sentences(text: &str) -> Vec<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let mut sentences = Vec::new();
    let chars: Vec<char> = trimmed.chars().collect();
    let len = chars.len();
    let mut start_idx = 0;
    let mut in_code_block = false;
    let mut in_inline_code = false;

    // Pre-check code fences: ensure dangling unclosed fence doesn't swallow document
    let total_fences = text.matches("```").count();
    let max_toggleable_fences = total_fences - (total_fences % 2);
    let mut fences_seen = 0;

    let mut i = 0;
    while i < len {
        // Toggle markdown code fence
        if i + 2 < len && chars[i] == '`' && chars[i + 1] == '`' && chars[i + 2] == '`' {
            fences_seen += 1;
            if fences_seen <= max_toggleable_fences {
                in_code_block = !in_code_block;
            }
            i += 3;
            continue;
        }

        if in_code_block {
            i += 1;
            continue;
        }

        let c = chars[i];

        // Toggle inline backtick code (reset at newline)
        if c == '`' {
            in_inline_code = !in_inline_code;
            i += 1;
            continue;
        }

        if in_inline_code {
            if c == '\n' {
                in_inline_code = false;
            } else {
                i += 1;
                continue;
            }
        }

        // Break on newline if previous line had content
        if c == '\n' {
            let chunk: String = chars[start_idx..i].iter().collect();
            let cleaned = chunk.trim();
            if !cleaned.is_empty() {
                sentences.push(cleaned.to_string());
            }
            start_idx = i + 1;
            i += 1;
            continue;
        }

        // Punctuation check: '.', '!', '?'
        if c == '.' || c == '!' || c == '?' {
            // Check if '.' is part of a number, URL, version, or abbreviation
            if c == '.' {
                // Decimal check: e.g. 3.14
                let prev_is_digit = i > 0 && chars[i - 1].is_ascii_digit();
                let next_is_digit = i + 1 < len && chars[i + 1].is_ascii_digit();
                if prev_is_digit && next_is_digit {
                    i += 1;
                    continue;
                }

                // Check version: e.g. v1.2.3 or 1.2.3
                let next_is_alpha_or_num = i + 1 < len && chars[i + 1].is_alphanumeric();
                let prev_is_alpha_or_num = i > 0 && chars[i - 1].is_alphanumeric();
                if prev_is_alpha_or_num && next_is_alpha_or_num {
                    // Check if it's like "github.com" or "example.org" or "v1.2"
                    i += 1;
                    continue;
                }

                // Check common abbreviations around index i
                let mut is_abbrev = false;
                for &abbrev in COMMON_ABBREVIATIONS {
                    let abbrev_chars: Vec<char> = abbrev.chars().collect();
                    let abbrev_len = abbrev_chars.len();
                    if i + 1 >= abbrev_len {
                        let potential_start = i + 1 - abbrev_len;
                        let slice: String = chars[potential_start..=i].iter().collect();
                        if slice.to_lowercase() == abbrev {
                            is_abbrev = true;
                            break;
                        }
                    }
                }
                if is_abbrev {
                    i += 1;
                    continue;
                }
            }

            // Consume trailing punctuation (e.g. '...', '?!') and closing delimiters
            let mut end_idx = i;
            while end_idx + 1 < len && matches!(chars[end_idx + 1], '.' | '!' | '?') {
                end_idx += 1;
            }
            while end_idx + 1 < len && matches!(chars[end_idx + 1], ')' | '"' | '\'' | '”' | '’' | ']' | '}' | '»')
            {
                end_idx += 1;
            }

            let is_boundary = if end_idx + 1 >= len {
                true
            } else {
                chars[end_idx + 1].is_whitespace()
            };

            if is_boundary {
                let chunk: String = chars[start_idx..=end_idx].iter().collect();
                let cleaned = chunk.trim();
                if !cleaned.is_empty() {
                    sentences.push(cleaned.to_string());
                }
                start_idx = end_idx + 1;
                i = end_idx;
            }
        }

        i += 1;
    }

    // Capture remaining tail if any
    if start_idx < len {
        let chunk: String = chars[start_idx..len].iter().collect();
        let cleaned = chunk.trim();
        if !cleaned.is_empty() {
            sentences.push(cleaned.to_string());
        }
    }

    sentences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_split_urls_and_versions() {
        let text =
            "Visit https://github.com/Nhan-209/mcp-plugin-math for v0.2.0 updates. The value is 3.1415! Are you ready?";
        let sentences = smart_split_sentences(text);

        assert_eq!(sentences.len(), 3);
        assert!(sentences[0].contains("https://github.com/Nhan-209/mcp-plugin-math"));
        assert!(sentences[0].contains("v0.2.0 updates."));
        assert_eq!(sentences[1], "The value is 3.1415!");
        assert_eq!(sentences[2], "Are you ready?");
    }

    #[test]
    fn test_smart_split_abbreviations() {
        let text = "We use tools e.g. rustc and cargo. Then we deploy.";
        let sentences = smart_split_sentences(text);

        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[0], "We use tools e.g. rustc and cargo.");
        assert_eq!(sentences[1], "Then we deploy.");
    }

    #[test]
    fn test_smart_split_code_block() {
        let text = "Here is code:\n```rust\nlet x = 1.0;\nprintln!(\"hi\");\n```\nDone.";
        let sentences = smart_split_sentences(text);

        assert!(!sentences.is_empty());
        assert_eq!(sentences.last().unwrap(), "Done.");
    }

    #[test]
    fn test_smart_split_closing_quotes() {
        let text = "He said \"Hello.\" She replied \"Hi!\" They walked away.";
        let sentences = smart_split_sentences(text);

        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "He said \"Hello.\"");
        assert_eq!(sentences[1], "She replied \"Hi!\"");
        assert_eq!(sentences[2], "They walked away.");
    }

    #[test]
    fn test_smart_split_inline_code() {
        let text = "Check `foo.bar()` method. Next line.";
        let sentences = smart_split_sentences(text);

        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[0], "Check `foo.bar()` method.");
        assert_eq!(sentences[1], "Next line.");
    }

    #[test]
    fn test_smart_split_unclosed_fence() {
        let text = "Some preamble.\n```\nUnclosed code\nMore text.";
        let sentences = smart_split_sentences(text);

        assert!(sentences.len() >= 2);
        assert_eq!(sentences[0], "Some preamble.");
    }

    #[test]
    fn test_smart_split_multi_punctuation() {
        let text = "Really??? Wow... Done!";
        let sentences = smart_split_sentences(text);

        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "Really???");
        assert_eq!(sentences[1], "Wow...");
        assert_eq!(sentences[2], "Done!");
    }
}
