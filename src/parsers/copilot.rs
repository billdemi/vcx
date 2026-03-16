use crate::core::rule::*;
use crate::parsers::claude;

/// Parse GitHub Copilot context files
/// Both copilot-instructions.md and .prompt.md files
pub fn parse(file: &ContextFile, content: &str) -> Vec<Rule> {
    // Both formats are essentially markdown, .prompt.md may have YAML frontmatter
    // but the body is parsed the same way
    claude::parse(file, content)
}
