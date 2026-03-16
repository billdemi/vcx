pub mod aider;
pub mod claude;
pub mod copilot;
pub mod cursor;
pub mod windsurf;

use std::path::Path;

use crate::core::rule::{ContextFile, ToolContext, ToolKind};

/// Parse all context files into tool contexts with extracted rules
pub fn parse_all(files: &[ContextFile], project_dir: &Path) -> Vec<ToolContext> {
    let mut contexts = Vec::new();

    // Group files by tool
    let tools: Vec<ToolKind> = {
        let mut t: Vec<ToolKind> = files.iter().map(|f| f.tool).collect();
        t.sort_by_key(|t| t.display_name().to_string());
        t.dedup();
        t
    };

    for tool in tools {
        let tool_files: Vec<ContextFile> = files.iter().filter(|f| f.tool == tool).cloned().collect();
        let mut all_rules = Vec::new();

        for cf in &tool_files {
            if let Ok(content) = std::fs::read_to_string(&cf.path) {
                let rules = match tool {
                    ToolKind::Claude => claude::parse(&cf, &content),
                    ToolKind::Cursor => cursor::parse(&cf, &content),
                    ToolKind::Copilot => copilot::parse(&cf, &content),
                    ToolKind::Windsurf => windsurf::parse(&cf, &content),
                    ToolKind::Aider => aider::parse(&cf, &content, project_dir),
                    ToolKind::Codex => claude::parse(&cf, &content), // Same markdown format
                };
                all_rules.extend(rules);
            }
        }

        contexts.push(ToolContext {
            tool,
            files: tool_files,
            rules: all_rules,
        });
    }

    contexts
}
