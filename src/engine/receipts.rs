use serde::{Deserialize, Serialize};

/// Machine-verifiable execution receipt produced by a tool executor or sandboxed runtime.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionReceipt {
    /// Optional globally unique receipt identifier (e.g. UUID)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_id: Option<String>,
    /// Unique action or task ID matching a planned task
    pub action_id: String,
    /// Tool or capability executed (e.g. "cargo_test", "write_file", "git_commit")
    pub tool_name: String,
    /// Cryptographic or semantic hash of tool arguments
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments_hash: Option<String>,
    /// Cryptographic or content hash of the tool execution result
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_hash: Option<String>,
    /// Process exit code (0 indicates success). Must be present for machine-verifiable receipts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Repository or workspace commit/revision hash at execution time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_revision: Option<String>,
    /// Timestamp when execution started (ISO 8601)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Timestamp when execution completed (ISO 8601)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}

impl ExecutionReceipt {
    /// True if the receipt contains all mandatory machine-verifiable cryptographic fields:
    /// - non-empty action_id and tool_name
    /// - explicit exit_code (exit_code=None is unverifiable, cannot be assumed success)
    /// - non-empty arguments_hash and result_hash
    pub fn is_machine_verifiable(&self) -> bool {
        !self.action_id.trim().is_empty()
            && !self.tool_name.trim().is_empty()
            && self.exit_code.is_some()
            && self.arguments_hash.as_ref().map(|h| !h.trim().is_empty()).unwrap_or(false)
            && self.result_hash.as_ref().map(|h| !h.trim().is_empty()).unwrap_or(false)
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
    /// SHA-256 hash of the artifact content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// Timestamp of receipt generation (ISO 8601)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Provenance origin metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
    /// Optional claim text or identifier that this artifact directly supports
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_binding: Option<String>,
}

impl EvidenceReceipt {
    /// True if the evidence receipt specifies non-empty kind and source_id
    pub fn is_valid_evidence(&self) -> bool {
        !self.kind.trim().is_empty() && !self.source_id.trim().is_empty()
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
