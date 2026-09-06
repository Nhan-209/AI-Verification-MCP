//! Centralized Resource Limits to prevent Denial of Service (DoS) and computational exhaustion.

pub const MAX_TASKS: usize = 200;
pub const MAX_REQUIREMENTS: usize = 200;
pub const MAX_EXECUTED_STEPS: usize = 500;
pub const MAX_CODE_BYTES: usize = 512 * 1024; // 512 KB
pub const MAX_TEXT_BYTES: usize = 64 * 1024; // 64 KB
pub const MAX_DIFF_BYTES: usize = 512 * 1024; // 512 KB
pub const MAX_DIFF_LINES: usize = 10_000;
pub const MAX_LCS_CELLS: usize = 2_500_000; // Computational complexity cap for LCS
pub const MAX_TASK_ID_LEN: usize = 64;
pub const MAX_TASK_NAME_LEN: usize = 256;
pub const MAX_JSON_REQUEST_BYTES: usize = 2 * 1024 * 1024; // 2 MB transport-level frame size limit

/// Validates that an arbitrary text payload stays within safety bounds.
pub fn validate_text_bound(text: &str, field_name: &str, max_bytes: usize) -> Result<(), String> {
    if text.len() > max_bytes {
        Err(format!(
            "Resource limit exceeded: {} size {} bytes > maximum {} bytes",
            field_name,
            text.len(),
            max_bytes
        ))
    } else {
        Ok(())
    }
}
