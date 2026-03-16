use crate::core::config::VcxConfig;
use crate::core::diagnostic::*;
use crate::core::rule::*;

const DEFAULT_MAX_SIZE: u64 = 5 * 1024; // 5KB

/// Check that context files don't exceed recommended size
pub fn check_file_size(contexts: &[ToolContext], config: &VcxConfig) -> Vec<Diagnostic> {
    let max_size = DEFAULT_MAX_SIZE; // TODO: read from config

    let mut diagnostics = Vec::new();

    for ctx in contexts {
        for file in &ctx.files {
            if file.size_bytes > max_size {
                diagnostics.push(Diagnostic {
                    rule_id: "breadth/too-large".to_string(),
                    severity: Severity::Warning,
                    message: format!(
                        "Context file is {:.1}KB, exceeds recommended {}KB limit. Large context files may dilute important instructions.",
                        file.size_bytes as f64 / 1024.0,
                        max_size / 1024,
                    ),
                    file: file.path.clone(),
                    line: 1,
                    related: None,
                });
            }
        }
    }

    diagnostics
}
