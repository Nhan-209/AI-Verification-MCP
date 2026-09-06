use serde::{Deserialize, Serialize};
use std::path::Path;

/// 5-tier Provenance hierarchy for technical assertions and citations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProvenanceLevel {
    /// Zero evidence or ungrounded assertion
    None = 0,
    /// Syntactic marker present (unverified URL, markdown code block, bare claim)
    SyntacticMarker = 1,
    /// Recognized authority domain (e.g. docs.rs, ietf.org) whose hostname is legitimate,
    /// but whose document existence or content entailment has not been fetched
    AuthorityVerified = 2,
    /// Source verified artifact (e.g. curated RFC in known registry or verified existing local workspace file)
    SourceVerified = 3,
    /// Fully verified source bound to a specific factual technical claim
    ClaimSupported = 4,
}

impl ProvenanceLevel {
    pub fn is_grounded(&self) -> bool {
        *self >= ProvenanceLevel::AuthorityVerified
    }
}

/// Established authoritative technical documentation and specification domains.
pub const AUTHORITATIVE_DOMAINS: &[&str] = &[
    "docs.rs",
    "crates.io",
    "github.com",
    "ietf.org",
    "w3.org",
    "developer.mozilla.org",
    "python.org",
    "go.dev",
    "pkg.go.dev",
    "kernel.org",
    "iso.org",
    "ieee.org",
    "rust-lang.org",
    "npmjs.com",
    "pypi.org",
    "golang.org",
    "open-std.org",
];

/// Known placeholder, mock, or adversarial test domains.
pub const UNTRUSTED_OR_PLACEHOLDER_DOMAINS: &[&str] = &[
    "example.com",
    "example.org",
    "test.com",
    "localhost",
    "placeholder.com",
    "attacker.example",
    "evil.com",
    "foo.bar",
    "domain.com",
];

/// Curated registry of verified, widely-implemented IETF RFC standards.
pub const KNOWN_RFC_REGISTRY: &[u32] = &[
    768, 791, 792, 793, 826, 854, 862, 959, 1034, 1035, 1122, 1123, 1157, 1191, 1213, 1234, 1305, 1321, 1332, 1350,
    1541, 1542, 1661, 1918, 1939, 1945, 1997, 2024, 2045, 2046, 2068, 2119, 2131, 2132, 2205, 2246, 2326, 2328, 2401,
    2460, 2616, 2818, 2821, 2822, 3031, 3261, 3315, 3339, 3411, 3412, 3414, 3492, 3550, 3986, 4122, 4251, 4252, 4253,
    4254, 4301, 4346, 4364, 4648, 4861, 4862, 4960, 5246, 5280, 5321, 5322, 5869, 5952, 6066, 6120, 6121, 6265, 6347,
    6455, 6749, 6750, 7230, 7231, 7232, 7233, 7234, 7235, 7252, 7515, 7519, 7540, 7636, 7644, 7946, 8174, 8200, 8259,
    8446, 8484, 8999, 9000, 9001, 9110, 9111, 9112, 9113, 9114, 9204, 9218, 9293, 9440,
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceEvidence {
    pub max_provenance: ProvenanceLevel,
    pub detected_urls: Vec<String>,
    pub detected_rfcs: Vec<u32>,
    pub detected_files: Vec<String>,
    pub has_authoritative_domain: bool,
    pub has_verified_source: bool,
    pub has_bare_code_block: bool,
}

pub struct EvidenceClassifier;

impl EvidenceClassifier {
    /// Classifies an individual URL based on hostname authority.
    pub fn classify_url(url: &str) -> ProvenanceLevel {
        let clean = url.trim();
        let lower = clean.to_lowercase();

        if !lower.starts_with("http://") && !lower.starts_with("https://") {
            return ProvenanceLevel::None;
        }

        let after_scheme = if let Some(stripped) = lower.strip_prefix("https://") {
            stripped
        } else if let Some(stripped) = lower.strip_prefix("http://") {
            stripped
        } else {
            return ProvenanceLevel::None;
        };

        let host = after_scheme.split(&['/', '?', '#', ':'][..]).next().unwrap_or("");
        if host.is_empty() {
            return ProvenanceLevel::SyntacticMarker;
        }

        if UNTRUSTED_OR_PLACEHOLDER_DOMAINS
            .iter()
            .any(|&d| host == d || host.ends_with(&format!(".{}", d)))
        {
            return ProvenanceLevel::SyntacticMarker;
        }

        if AUTHORITATIVE_DOMAINS
            .iter()
            .any(|&d| host == d || host.ends_with(&format!(".{}", d)))
        {
            ProvenanceLevel::AuthorityVerified
        } else {
            ProvenanceLevel::SyntacticMarker
        }
    }

    /// Verifies if an RFC number is in the curated registry.
    pub fn is_curated_rfc(num: u32) -> bool {
        KNOWN_RFC_REGISTRY.contains(&num)
    }

    /// Validates if a string is a clean, non-traversal local file path that exists on disk.
    pub fn is_verified_local_file(path_str: &str) -> bool {
        let clean = path_str
            .trim()
            .trim_matches(|c: char| c == '`' || c == '\'' || c == '"' || c == '(' || c == ')');
        if clean.contains("..") || clean.is_empty() {
            return false;
        }

        let has_separator = clean.contains('/') || clean.contains('\\');
        let has_code_ext = clean.ends_with(".rs")
            || clean.ends_with(".ts")
            || clean.ends_with(".js")
            || clean.ends_with(".py")
            || clean.ends_with(".go")
            || clean.ends_with(".json")
            || clean.ends_with(".toml")
            || clean.ends_with(".md");

        if has_separator && has_code_ext {
            Path::new(clean).exists()
        } else {
            false
        }
    }

    /// Canonical evidence evaluation for an individual sentence.
    pub fn classify_sentence(sentence: &str) -> SentenceEvidence {
        let mut max_prov = ProvenanceLevel::None;
        let mut detected_urls = Vec::new();
        let mut detected_rfcs = Vec::new();
        let mut detected_files = Vec::new();
        let mut has_authoritative = false;
        let mut has_verified_src = false;
        let has_code_block = sentence.contains("```");

        if has_code_block && max_prov < ProvenanceLevel::SyntacticMarker {
            max_prov = ProvenanceLevel::SyntacticMarker;
        }

        for word in sentence.split_whitespace() {
            let clean = word.trim_matches(|c: char| {
                c == '(' || c == ')' || c == '[' || c == ']' || c == '<' || c == '>' || c == ',' || c == ';' || c == '"'
            });
            if clean.starts_with("http://") || clean.starts_with("https://") {
                let level = Self::classify_url(clean);
                detected_urls.push(clean.to_string());
                if level == ProvenanceLevel::AuthorityVerified {
                    has_authoritative = true;
                    if max_prov < ProvenanceLevel::AuthorityVerified {
                        max_prov = ProvenanceLevel::AuthorityVerified;
                    }
                } else if max_prov < ProvenanceLevel::SyntacticMarker {
                    max_prov = ProvenanceLevel::SyntacticMarker;
                }
            } else if Self::is_verified_local_file(clean) {
                detected_files.push(clean.to_string());
                has_verified_src = true;
                if max_prov < ProvenanceLevel::SourceVerified {
                    max_prov = ProvenanceLevel::SourceVerified;
                }
            }
        }

        // RFC scanning
        let lower = sentence.to_lowercase();
        let mut start_idx = 0;
        while let Some(pos) = lower[start_idx..].find("rfc") {
            let rfc_start = start_idx + pos + 3;
            let slice = &lower[rfc_start..];
            let num_str: String = slice
                .chars()
                .skip_while(|c| c.is_whitespace() || *c == '-' || *c == ':')
                .take_while(|c| c.is_ascii_digit())
                .collect();

            if let Ok(num) = num_str.parse::<u32>() {
                detected_rfcs.push(num);
                if Self::is_curated_rfc(num) {
                    has_verified_src = true;
                    if max_prov < ProvenanceLevel::SourceVerified {
                        max_prov = ProvenanceLevel::SourceVerified;
                    }
                } else if max_prov < ProvenanceLevel::SyntacticMarker {
                    max_prov = ProvenanceLevel::SyntacticMarker;
                }
            }
            start_idx = rfc_start;
        }

        // Standards scanning (IEEE 754, ISO/IEC 27001)
        if lower.contains("ieee 754") || lower.contains("iso/iec 27001") || lower.contains("iso 27001") {
            has_verified_src = true;
            if max_prov < ProvenanceLevel::SourceVerified {
                max_prov = ProvenanceLevel::SourceVerified;
            }
        }

        SentenceEvidence {
            max_provenance: max_prov,
            detected_urls,
            detected_rfcs,
            detected_files,
            has_authoritative_domain: has_authoritative,
            has_verified_source: has_verified_src,
            has_bare_code_block: has_code_block,
        }
    }

    /// Helper: returns true if the sentence has genuine empirical grounding (AuthorityVerified or higher).
    /// Bare code blocks or unverified URLs return false.
    pub fn has_grounded_evidence(sentence: &str) -> bool {
        Self::classify_sentence(sentence).max_provenance.is_grounded()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_authoritative_url() {
        assert_eq!(
            EvidenceClassifier::classify_url("https://docs.rs/serde/latest/serde/"),
            ProvenanceLevel::AuthorityVerified
        );
        assert_eq!(
            EvidenceClassifier::classify_url("https://crates.io/crates/tokio"),
            ProvenanceLevel::AuthorityVerified
        );
        assert_eq!(
            EvidenceClassifier::classify_url("https://attacker.example/evil"),
            ProvenanceLevel::SyntacticMarker
        );
        assert_eq!(
            EvidenceClassifier::classify_url("https://example.com/test"),
            ProvenanceLevel::SyntacticMarker
        );
    }

    #[test]
    fn test_curated_rfc() {
        assert!(EvidenceClassifier::is_curated_rfc(2119));
        assert!(EvidenceClassifier::is_curated_rfc(8446));
        assert!(!EvidenceClassifier::is_curated_rfc(9999));
    }

    #[test]
    fn test_sentence_evidence_classification() {
        let bare_code = "Here is code: ```rust fn test() {} ```";
        let ev1 = EvidenceClassifier::classify_sentence(bare_code);
        assert_eq!(ev1.max_provenance, ProvenanceLevel::SyntacticMarker);
        assert!(!ev1.max_provenance.is_grounded());

        let fake_url = "Guaranteed 100% bug free: https://untrusted-fake.org/evidence";
        let ev2 = EvidenceClassifier::classify_sentence(fake_url);
        assert_eq!(ev2.max_provenance, ProvenanceLevel::SyntacticMarker);
        assert!(!ev2.max_provenance.is_grounded());

        let real_rfc = "According to RFC 2119, MUST indicates requirement.";
        let ev3 = EvidenceClassifier::classify_sentence(real_rfc);
        assert_eq!(ev3.max_provenance, ProvenanceLevel::SourceVerified);
        assert!(ev3.max_provenance.is_grounded());

        let auth_doc = "Refer to https://docs.rs/serde for serialization details.";
        let ev4 = EvidenceClassifier::classify_sentence(auth_doc);
        assert_eq!(ev4.max_provenance, ProvenanceLevel::AuthorityVerified);
        assert!(ev4.max_provenance.is_grounded());
    }
}
