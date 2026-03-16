use crate::core::rule::{FileFormat, ToolKind};

/// A known context file pattern for a specific tool
pub struct FilePattern {
    pub tool: ToolKind,
    /// Glob pattern relative to project root
    pub pattern: &'static str,
    pub format: FileFormat,
    /// Whether this file format is deprecated
    pub deprecated: bool,
}

/// All known context file patterns across tools
pub fn all_patterns() -> Vec<FilePattern> {
    vec![
        // Claude Code
        FilePattern {
            tool: ToolKind::Claude,
            pattern: "CLAUDE.md",
            format: FileFormat::Markdown,
            deprecated: false,
        },
        FilePattern {
            tool: ToolKind::Claude,
            pattern: ".claude/settings.json",
            format: FileFormat::Json,
            deprecated: false,
        },
        // Cursor (modern)
        FilePattern {
            tool: ToolKind::Cursor,
            pattern: ".cursor/rules/*.mdc",
            format: FileFormat::MdcYamlFrontmatter,
            deprecated: false,
        },
        // Cursor (deprecated)
        FilePattern {
            tool: ToolKind::Cursor,
            pattern: ".cursorrules",
            format: FileFormat::Markdown,
            deprecated: true,
        },
        // GitHub Copilot
        FilePattern {
            tool: ToolKind::Copilot,
            pattern: ".github/copilot-instructions.md",
            format: FileFormat::Markdown,
            deprecated: false,
        },
        FilePattern {
            tool: ToolKind::Copilot,
            pattern: ".github/prompts/*.prompt.md",
            format: FileFormat::MdcYamlFrontmatter,
            deprecated: false,
        },
        // Windsurf
        FilePattern {
            tool: ToolKind::Windsurf,
            pattern: ".windsurfrules",
            format: FileFormat::Markdown,
            deprecated: false,
        },
        FilePattern {
            tool: ToolKind::Windsurf,
            pattern: ".windsurf/rules/*.md",
            format: FileFormat::Markdown,
            deprecated: false,
        },
        // Aider
        FilePattern {
            tool: ToolKind::Aider,
            pattern: ".aider.conf.yml",
            format: FileFormat::Yaml,
            deprecated: false,
        },
        // Codex
        FilePattern {
            tool: ToolKind::Codex,
            pattern: "AGENTS.md",
            format: FileFormat::Markdown,
            deprecated: false,
        },
    ]
}

/// Get patterns for a specific tool
pub fn patterns_for(tool: ToolKind) -> Vec<FilePattern> {
    all_patterns()
        .into_iter()
        .filter(|p| p.tool == tool)
        .collect()
}
