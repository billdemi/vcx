use sha2::{Digest, Sha256};

use crate::core::rule::*;

/// Parse a CLAUDE.md (or AGENTS.md) file into rules.
/// Each non-empty line or paragraph is treated as a rule.
pub fn parse(file: &ContextFile, content: &str) -> Vec<Rule> {
    let mut rules = Vec::new();
    let mut current_block = String::new();
    let mut block_start = 0usize;
    let mut in_code_block = false;

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Track code blocks to skip them
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }

        // Skip headings — they're structural, not instructions
        if trimmed.starts_with('#') {
            // Flush current block
            if !current_block.trim().is_empty() {
                rules.push(make_rule(file, &current_block, block_start, i));
                current_block.clear();
            }
            continue;
        }

        // Empty line = block boundary
        if trimmed.is_empty() {
            if !current_block.trim().is_empty() {
                rules.push(make_rule(file, &current_block, block_start, i));
                current_block.clear();
            }
            continue;
        }

        // Accumulate content
        if current_block.is_empty() {
            block_start = i + 1; // 1-indexed
        }
        if !current_block.is_empty() {
            current_block.push('\n');
        }
        current_block.push_str(trimmed);
    }

    // Flush last block
    if !current_block.trim().is_empty() {
        let end = content.lines().count();
        rules.push(make_rule(file, &current_block, block_start, end));
    }

    rules
}

fn make_rule(file: &ContextFile, content: &str, line_start: usize, line_end: usize) -> Rule {
    let id = content_hash(content);
    let topics = extract_topics(content);

    Rule {
        id,
        source: Source {
            tool: file.tool,
            file_path: file.path.clone(),
            line_start,
            line_end,
        },
        topics,
        content: content.to_string(),
        activation: Activation::Always,
    }
}

pub fn content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    // Normalize whitespace for hashing
    let normalized: String = content.split_whitespace().collect::<Vec<_>>().join(" ");
    hasher.update(normalized.as_bytes());
    format!("{:x}", hasher.finalize())[..12].to_string()
}

/// Extract topics from instruction text via keyword matching
pub fn extract_topics(content: &str) -> Vec<Topic> {
    let lower = content.to_lowercase();
    let mut topics = Vec::new();

    // Style keywords
    if has_any(&lower, &[
        "indent", "tab", "space", "semicolon", "quote", "format",
        "camelcase", "snake_case", "kebab-case", "naming", "prettier",
        "eslint", "clippy", "lint", "style",
    ]) {
        topics.push(Topic::Style);
    }

    // Architecture keywords
    if has_any(&lower, &[
        "pattern", "component", "module", "architecture", "composition",
        "inheritance", "oop", "functional", "mvc", "mvvm", "clean architecture",
        "layer", "abstraction", "solid",
    ]) {
        topics.push(Topic::Architecture);
    }

    // Tooling keywords
    if has_any(&lower, &[
        "npm", "yarn", "pnpm", "bun", "cargo", "pip", "poetry",
        "webpack", "vite", "esbuild", "turbopack", "package manager",
        "build", "bundle", "docker", "ci/cd",
    ]) {
        topics.push(Topic::Tooling);
    }

    // Testing keywords
    if has_any(&lower, &[
        "test", "jest", "vitest", "pytest", "mocha", "cypress",
        "playwright", "mock", "fixture", "assertion", "coverage",
        "unit test", "integration test", "e2e",
    ]) {
        topics.push(Topic::Testing);
    }

    // Git keywords
    if has_any(&lower, &[
        "commit", "branch", "merge", "rebase", "git", "pull request",
        "pr ", "conventional commit",
    ]) {
        topics.push(Topic::Git);
    }

    // Error handling keywords
    if has_any(&lower, &[
        "error", "exception", "try", "catch", "result", "option",
        "unwrap", "panic", "throw", "error handling",
    ]) {
        topics.push(Topic::ErrorHandling);
    }

    // Security keywords
    if has_any(&lower, &[
        "security", "auth", "token", "password", "secret", "encrypt",
        "xss", "csrf", "injection", "sanitize", "vulnerability",
    ]) {
        topics.push(Topic::Security);
    }

    if topics.is_empty() {
        topics.push(Topic::Custom("general".to_string()));
    }

    topics
}

fn has_any(text: &str, keywords: &[&str]) -> bool {
    keywords.iter().any(|kw| text.contains(kw))
}
