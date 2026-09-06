use serde::{Deserialize, Serialize};

/// Validates if a string is a 64-character lowercase or uppercase ASCII hex SHA-256 hash.
pub fn validate_hex_sha256(s: &str) -> bool {
    let trimmed = s.trim();
    trimmed.len() == 64 && trimmed.chars().all(|c| c.is_ascii_hexdigit())
}

/// Helper: calculates days since Unix epoch (1970-01-01) from year, month, day.
/// Howard Hinnant's civil calendar algorithm.
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = y - if m <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe as i64 - 719468
}

/// Parses an RFC 3339 / ISO 8601 timestamp string into epoch seconds (as f64).
pub fn parse_rfc3339(s: &str) -> Option<f64> {
    let clean = s.trim();
    if clean.len() < 19 {
        return None;
    }
    let b = clean.as_bytes();
    if b[4] != b'-' || b[7] != b'-' || (b[10] != b'T' && b[10] != b't') || b[13] != b':' || b[16] != b':' {
        return None;
    }
    let year = clean[0..4].parse::<i64>().ok()?;
    let month = clean[5..7].parse::<u32>().ok()?;
    let day = clean[8..10].parse::<u32>().ok()?;
    let hour = clean[11..13].parse::<u32>().ok()?;
    let minute = clean[14..16].parse::<u32>().ok()?;
    let second = clean[17..19].parse::<u32>().ok()?;

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || minute > 59 || second > 60 {
        return None;
    }

    let mut rem = &clean[19..];
    let mut fraction = 0.0;
    if rem.starts_with('.') {
        let frac_end = rem[1..]
            .find(|c: char| !c.is_ascii_digit())
            .map(|idx| idx + 1)
            .unwrap_or(rem.len());
        let frac_str = &rem[0..frac_end];
        fraction = frac_str.parse::<f64>().ok()?;
        rem = &rem[frac_end..];
    }

    let mut tz_offset_sec = 0.0;
    let rem_trim = rem.trim();
    if rem_trim.is_empty() || rem_trim.eq_ignore_ascii_case("Z") {
        tz_offset_sec = 0.0;
    } else if rem_trim.starts_with('+') || rem_trim.starts_with('-') {
        let sign = if rem_trim.starts_with('+') { 1.0 } else { -1.0 };
        let tz_body = &rem_trim[1..];
        let parts: Vec<&str> = tz_body.split(':').collect();
        if parts.is_empty() {
            return None;
        }
        let tz_h = parts[0].parse::<u32>().ok()?;
        let tz_m = if parts.len() > 1 { parts[1].parse::<u32>().ok()? } else { 0 };
        if tz_h > 23 || tz_m > 59 {
            return None;
        }
        tz_offset_sec = sign * (tz_h as f64 * 3600.0 + tz_m as f64 * 60.0);
    } else {
        return None;
    }

    let days = days_from_civil(year, month, day);
    let total_sec =
        (days as f64 * 86400.0) + (hour as f64 * 3600.0) + (minute as f64 * 60.0) + second as f64 + fraction
            - tz_offset_sec;

    Some(total_sec)
}

/// Validates that started_at and finished_at are valid RFC 3339 timestamps and finished_at >= started_at.
pub fn validate_rfc3339_sequence(started: Option<&str>, finished: Option<&str>) -> bool {
    match (started, finished) {
        (None, None) => true,
        (Some(s), None) => parse_rfc3339(s).is_some(),
        (None, Some(f)) => parse_rfc3339(f).is_some(),
        (Some(s), Some(f)) => {
            if let (Some(t_start), Some(t_finish)) = (parse_rfc3339(s), parse_rfc3339(f)) {
                t_finish >= t_start
            } else {
                false
            }
        }
    }
}

/// Machine-verifiable execution receipt produced by a tool executor or sandboxed runtime.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionReceipt {
    /// Globally unique receipt identifier (e.g. UUID, receipt-123)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_id: Option<String>,
    /// Unique action or task ID matching a planned task
    pub action_id: String,
    /// Tool or capability executed (e.g. "cargo_test", "write_file", "git_commit")
    pub tool_name: String,
    /// Cryptographic hash of tool arguments (valid 64-char hex SHA-256)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments_hash: Option<String>,
    /// Cryptographic hash of tool execution result (valid 64-char hex SHA-256)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_hash: Option<String>,
    /// Process exit code (0 indicates success). Must be present for machine-verifiable receipts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Optional session/audit binding ID to prevent replay attacks across audits
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audit_id: Option<String>,
    /// Optional workspace/repository ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// Repository or workspace commit/revision hash at execution time (git SHA hex)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_revision: Option<String>,
    /// Freshness nonce for anti-replay verification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// Optional runtime or signer identity issuing this receipt
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// Optional cryptographic signature of the receipt payload
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Timestamp when execution started (RFC 3339 / ISO 8601)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Timestamp when execution completed (RFC 3339 / ISO 8601)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}

impl ExecutionReceipt {
    /// True if the receipt contains all mandatory machine-verifiable cryptographic fields:
    /// - non-empty receipt_id, action_id and tool_name
    /// - explicit exit_code (exit_code=None is unverifiable, cannot be assumed success)
    /// - valid 64-character hex SHA-256 for arguments_hash and result_hash
    /// - valid RFC 3339 timestamps (if present) with finished_at >= started_at
    pub fn is_machine_verifiable(&self) -> bool {
        let has_valid_id = self
            .receipt_id
            .as_ref()
            .map(|id| !id.trim().is_empty())
            .unwrap_or(false);
        let has_action_and_tool = !self.action_id.trim().is_empty() && !self.tool_name.trim().is_empty();
        let has_exit_code = self.exit_code.is_some();
        let has_valid_arg_hash = self
            .arguments_hash
            .as_ref()
            .map(|h| validate_hex_sha256(h))
            .unwrap_or(false);
        let has_valid_res_hash = self
            .result_hash
            .as_ref()
            .map(|h| validate_hex_sha256(h))
            .unwrap_or(false);
        let has_valid_timestamps = validate_rfc3339_sequence(self.started_at.as_deref(), self.finished_at.as_deref());

        has_valid_id
            && has_action_and_tool
            && has_exit_code
            && has_valid_arg_hash
            && has_valid_res_hash
            && has_valid_timestamps
    }
}

/// Machine-verifiable evidence artifact receipt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceReceipt {
    /// Optional unique receipt identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_id: Option<String>,
    /// Artifact kind: "FILE", "RFC", "TEST_RUN", "DOCS_URL", "AST_REPORT"
    pub kind: String,
    /// Source URI, file path, RFC identifier, or reference
    pub source_id: String,
    /// SHA-256 hash of the artifact content (must be valid 64-char hex)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// Timestamp of receipt generation (RFC 3339 / ISO 8601)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Provenance origin metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
    /// Optional structured claim ID (e.g. "C1") that this artifact directly supports
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_id: Option<String>,
    /// Optional claim text or identifier that this artifact directly supports
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_binding: Option<String>,
    /// Optional workspace identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl EvidenceReceipt {
    /// True if the evidence receipt specifies non-empty kind and source_id,
    /// valid SHA-256 hex format for TEST_RUN and AST_REPORT, safe file path (no directory traversal),
    /// and valid timestamp syntax if provided.
    pub fn is_valid_evidence(&self) -> bool {
        if self.kind.trim().is_empty() || self.source_id.trim().is_empty() {
            return false;
        }
        let kind_upper = self.kind.trim().to_uppercase();
        if kind_upper == "FILE" {
            let clean = self.source_id.trim();
            if clean.contains("..") || clean.is_empty() {
                return false;
            }
        }
        if kind_upper == "TEST_RUN" || kind_upper == "AST_REPORT" {
            match self.sha256.as_deref() {
                Some(h) => {
                    if !validate_hex_sha256(h) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        if let Some(ref h) = self.sha256 {
            if !validate_hex_sha256(h) {
                return false;
            }
        }
        if let Some(ref ts) = self.timestamp {
            if parse_rfc3339(ts).is_none() {
                return false;
            }
        }
        true
    }
}

/// Summary of execution receipts verified during an audit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReceiptsVerificationSummary {
    pub total_receipts: usize,
    pub matched_steps_count: usize,
    pub unattested_steps: Vec<String>,
    pub unverifiable_receipts_count: usize,
    pub failed_receipts_count: usize,
    pub has_full_provenance: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_hex_sha256() {
        assert!(validate_hex_sha256(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        ));
        assert!(validate_hex_sha256(
            "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855"
        ));
        assert!(!validate_hex_sha256("too-short"));
        assert!(!validate_hex_sha256(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b85g"
        ));
        assert!(!validate_hex_sha256(""));
    }

    #[test]
    fn test_parse_rfc3339() {
        assert!(parse_rfc3339("2026-09-06T14:30:00Z").is_some());
        assert!(parse_rfc3339("2026-09-06T14:30:00.123456Z").is_some());
        assert!(parse_rfc3339("2026-09-06T14:30:00+07:00").is_some());
        assert!(parse_rfc3339("not-a-date").is_none());
        assert!(parse_rfc3339("2026-02-30T10:00:00Z").is_some()); // civil parser handles day <= 31
    }

    #[test]
    fn test_validate_rfc3339_sequence() {
        assert!(validate_rfc3339_sequence(
            Some("2026-09-06T14:00:00Z"),
            Some("2026-09-06T14:05:00Z")
        ));
        assert!(validate_rfc3339_sequence(
            Some("2026-09-06T14:00:00Z"),
            Some("2026-09-06T14:00:00Z")
        ));
        assert!(!validate_rfc3339_sequence(
            Some("2026-09-06T14:05:00Z"),
            Some("2026-09-06T14:00:00Z")
        ));
        assert!(!validate_rfc3339_sequence(
            Some("invalid"),
            Some("2026-09-06T14:00:00Z")
        ));
    }

    #[test]
    fn test_execution_receipt_machine_verifiable() {
        let valid = ExecutionReceipt {
            receipt_id: Some("rcpt-1".to_string()),
            action_id: "t1".to_string(),
            tool_name: "cargo_test".to_string(),
            arguments_hash: Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()),
            result_hash: Some("88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589".to_string()),
            exit_code: Some(0),
            audit_id: Some("audit-123".to_string()),
            workspace_id: None,
            workspace_revision: None,
            nonce: None,
            issuer: None,
            signature: None,
            started_at: Some("2026-09-06T14:00:00Z".to_string()),
            finished_at: Some("2026-09-06T14:05:00Z".to_string()),
        };
        assert!(valid.is_machine_verifiable());

        let missing_id = ExecutionReceipt {
            receipt_id: None,
            ..valid.clone()
        };
        assert!(!missing_id.is_machine_verifiable());

        let invalid_hash = ExecutionReceipt {
            arguments_hash: Some("short".to_string()),
            ..valid.clone()
        };
        assert!(!invalid_hash.is_machine_verifiable());

        let inverted_time = ExecutionReceipt {
            started_at: Some("2026-09-06T14:10:00Z".to_string()),
            finished_at: Some("2026-09-06T14:00:00Z".to_string()),
            ..valid
        };
        assert!(!inverted_time.is_machine_verifiable());
    }

    #[test]
    fn test_evidence_receipt_validation() {
        let valid_test = EvidenceReceipt {
            receipt_id: Some("ev-1".to_string()),
            kind: "TEST_RUN".to_string(),
            source_id: "cargo test --all".to_string(),
            sha256: Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()),
            timestamp: None,
            provenance: None,
            claim_id: Some("C1".to_string()),
            claim_binding: None,
            workspace_id: None,
        };
        assert!(valid_test.is_valid_evidence());

        let invalid_sha = EvidenceReceipt {
            sha256: Some("fake_hash".to_string()),
            ..valid_test.clone()
        };
        assert!(!invalid_sha.is_valid_evidence());

        let traversal_file = EvidenceReceipt {
            kind: "FILE".to_string(),
            source_id: "../../../etc/passwd".to_string(),
            sha256: None,
            ..valid_test
        };
        assert!(!traversal_file.is_valid_evidence());
    }
}
