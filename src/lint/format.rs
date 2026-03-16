use crate::core::diagnostic::*;
use crate::core::rule::*;

/// Check that all context files are valid UTF-8
pub fn check_encoding(contexts: &[ToolContext]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for ctx in contexts {
        for file in &ctx.files {
            // Try reading as UTF-8
            match std::fs::read(&file.path) {
                Ok(bytes) => {
                    if std::str::from_utf8(&bytes).is_err() {
                        diagnostics.push(Diagnostic {
                            rule_id: "format/encoding".to_string(),
                            severity: Severity::Error,
                            message: "File is not valid UTF-8".to_string(),
                            file: file.path.clone(),
                            line: 1,
                            related: None,
                        });
                    }
                }
                Err(e) => {
                    diagnostics.push(Diagnostic {
                        rule_id: "format/encoding".to_string(),
                        severity: Severity::Error,
                        message: format!("Cannot read file: {}", e),
                        file: file.path.clone(),
                        line: 1,
                        related: None,
                    });
                }
            }
        }
    }

    diagnostics
}
