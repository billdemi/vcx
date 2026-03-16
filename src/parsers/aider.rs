use std::path::Path;

use crate::core::rule::*;
use crate::parsers::claude;

/// Parse Aider configuration file (.aider.conf.yml)
/// Extracts `read` entries as rules pointing to convention files
pub fn parse(file: &ContextFile, content: &str, project_dir: &Path) -> Vec<Rule> {
    let mut rules = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Look for read: entries (convention files loaded as context)
        if trimmed.starts_with("read:") || trimmed.starts_with("- ") {
            let value = if trimmed.starts_with("read:") {
                trimmed.trim_start_matches("read:").trim()
            } else {
                trimmed.trim_start_matches("- ").trim()
            };

            if value.is_empty() {
                continue;
            }

            // Check if the read target is a file that exists
            let target_path = project_dir.join(value);
            if target_path.exists() {
                // Parse the referenced file as markdown rules
                if let Ok(ref_content) = std::fs::read_to_string(&target_path) {
                    let ref_file = ContextFile {
                        tool: ToolKind::Aider,
                        path: target_path,
                        size_bytes: ref_content.len() as u64,
                        modified: None,
                        format: FileFormat::Markdown,
                        deprecated: false,
                    };
                    rules.extend(claude::parse(&ref_file, &ref_content));
                }
            } else {
                // Record the read directive itself as a rule
                rules.push(Rule {
                    id: claude::content_hash(trimmed),
                    source: Source {
                        tool: file.tool,
                        file_path: file.path.clone(),
                        line_start: i + 1,
                        line_end: i + 1,
                    },
                    topics: vec![Topic::Custom("config".to_string())],
                    content: trimmed.to_string(),
                    activation: Activation::Always,
                });
            }
        }
    }

    rules
}
