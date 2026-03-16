use regex::Regex;
use std::path::Path;

use crate::core::diagnostic::*;
use crate::core::rule::*;

/// Check for references to files that don't exist
pub fn check_file_references(contexts: &[ToolContext], project_dir: &Path) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Regex to find file path references
    let path_re = Regex::new(
        r#"(?:src|lib|app|test|tests|pkg|cmd|internal)/[a-zA-Z0-9_./-]+\.[a-zA-Z]{1,10}"#,
    )
    .unwrap();

    for ctx in contexts {
        for rule in &ctx.rules {
            for cap in path_re.find_iter(&rule.content) {
                let ref_path = cap.as_str();
                let full_path = project_dir.join(ref_path);

                if !full_path.exists() {
                    diagnostics.push(Diagnostic {
                        rule_id: "stale/file-ref".to_string(),
                        severity: Severity::Warning,
                        message: format!("References \"{}\" which does not exist", ref_path),
                        file: rule.source.file_path.clone(),
                        line: rule.source.line_start,
                        related: None,
                    });
                }
            }
        }
    }

    diagnostics
}

/// Check for deprecated file formats
pub fn check_deprecated_formats(contexts: &[ToolContext]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for ctx in contexts {
        for file in &ctx.files {
            if file.deprecated {
                let suggestion = match file.tool {
                    ToolKind::Cursor => {
                        "Migrate to .cursor/rules/*.mdc format"
                    }
                    _ => "This format is deprecated",
                };

                diagnostics.push(Diagnostic {
                    rule_id: "stale/tool-format".to_string(),
                    severity: Severity::Warning,
                    message: format!(
                        "\"{}\" uses a deprecated format. {}",
                        file.path.file_name().unwrap_or_default().to_string_lossy(),
                        suggestion,
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
