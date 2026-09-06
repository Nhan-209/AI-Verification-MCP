use serde::{Deserialize, Serialize};

/// Machine-verifiable execution receipt produced by a tool executor or sandboxed runtime.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionReceipt {
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
    /// Process exit code (0 indicates success)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Repository or workspace commit/revision hash at execution time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_revision: Option<String>,
}

/// Machine-verifiable evidence artifact receipt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceReceipt {
    /// Artifact kind: "FILE", "RFC", "TEST_RUN", "DOCS_URL", "AST_REPORT"
    pub kind: String,
    /// Source URI or path
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
}

/// Summary of execution receipts verified during an audit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReceiptsVerificationSummary {
    pub total_receipts: usize,
    pub matched_steps_count: usize,
    pub unattested_steps: Vec<String>,
    pub failed_receipts_count: usize,
    pub has_full_provenance: bool,
}
