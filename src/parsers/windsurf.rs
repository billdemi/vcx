use crate::core::rule::*;
use crate::parsers::claude;

/// Parse Windsurf context files (.windsurfrules, .windsurf/rules/*.md)
pub fn parse(file: &ContextFile, content: &str) -> Vec<Rule> {
    // Windsurf rules are plain markdown, same parsing as CLAUDE.md
    claude::parse(file, content)
}
