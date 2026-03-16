use serde_json;

use crate::core::diagnostic::Diagnostic;
use crate::core::rule::ContextFile;

/// Format scan results as JSON
pub fn format_scan(files: &[ContextFile]) -> String {
    serde_json::to_string_pretty(files).unwrap_or_else(|_| "[]".to_string())
}

/// Format lint diagnostics as JSON
pub fn format_lint(diagnostics: &[Diagnostic]) -> String {
    serde_json::to_string_pretty(diagnostics).unwrap_or_else(|_| "[]".to_string())
}
