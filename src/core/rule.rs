use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Supported AI coding tools
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolKind {
    Claude,
    Cursor,
    Copilot,
    Windsurf,
    Aider,
    Codex,
}

impl ToolKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Claude => "Claude Code",
            Self::Cursor => "Cursor",
            Self::Copilot => "Copilot",
            Self::Windsurf => "Windsurf",
            Self::Aider => "Aider",
            Self::Codex => "Codex",
        }
    }

    pub fn all() -> &'static [ToolKind] {
        &[
            Self::Claude,
            Self::Cursor,
            Self::Copilot,
            Self::Windsurf,
            Self::Aider,
            Self::Codex,
        ]
    }
}

impl std::fmt::Display for ToolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl std::str::FromStr for ToolKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "claude" | "claude-code" | "claudecode" => Ok(Self::Claude),
            "cursor" => Ok(Self::Cursor),
            "copilot" | "github-copilot" => Ok(Self::Copilot),
            "windsurf" => Ok(Self::Windsurf),
            "aider" => Ok(Self::Aider),
            "codex" => Ok(Self::Codex),
            _ => Err(format!("Unknown tool: {s}")),
        }
    }
}

/// Where a rule was found
#[derive(Debug, Clone, Serialize)]
pub struct Source {
    pub tool: ToolKind,
    pub file_path: PathBuf,
    pub line_start: usize,
    pub line_end: usize,
}

/// When a rule activates
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum Activation {
    Always,
    GlobPattern(Vec<String>),
    Manual,
    Description(String),
}

/// Topic categories for instructions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Topic {
    Style,
    Architecture,
    Tooling,
    Testing,
    Git,
    ErrorHandling,
    Security,
    Documentation,
    Performance,
    Custom(String),
}

/// A single instruction/rule extracted from a context file
#[derive(Debug, Clone, Serialize)]
pub struct Rule {
    /// Content hash for deduplication
    pub id: String,
    /// Where this rule came from
    pub source: Source,
    /// Normalized topic categories
    pub topics: Vec<Topic>,
    /// The raw instruction text
    pub content: String,
    /// When this rule activates
    pub activation: Activation,
}

/// A discovered context file
#[derive(Debug, Clone, Serialize)]
pub struct ContextFile {
    pub tool: ToolKind,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    pub format: FileFormat,
    pub deprecated: bool,
}

/// File format types
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileFormat {
    Markdown,
    MdcYamlFrontmatter,
    Yaml,
    Toml,
    Json,
}

/// Parsed context for one tool
#[derive(Debug)]
pub struct ToolContext {
    pub tool: ToolKind,
    pub files: Vec<ContextFile>,
    pub rules: Vec<Rule>,
}
