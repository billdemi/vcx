use crate::core::rule::*;
use crate::parsers::claude;

/// Parse a Cursor .mdc file (YAML frontmatter + markdown body)
/// or a .cursorrules file (plain markdown)
pub fn parse(file: &ContextFile, content: &str) -> Vec<Rule> {
    match file.format {
        FileFormat::MdcYamlFrontmatter => parse_mdc(file, content),
        _ => claude::parse(file, content), // .cursorrules = plain markdown
    }
}

fn parse_mdc(file: &ContextFile, content: &str) -> Vec<Rule> {
    // Split YAML frontmatter from body
    let (frontmatter, body) = split_frontmatter(content);

    // Parse frontmatter for activation info
    let activation = if let Some(fm) = &frontmatter {
        parse_activation(fm)
    } else {
        Activation::Always
    };

    // Parse body as markdown rules
    let mut rules = Vec::new();
    let body_offset = if frontmatter.is_some() {
        // Count frontmatter lines
        content
            .lines()
            .position(|l| l.trim() == "---" && content.lines().next().map_or(false, |f| f.trim() == "---"))
            .map(|_| {
                // Find second ---
                let mut count = 0;
                for (i, line) in content.lines().enumerate() {
                    if line.trim() == "---" {
                        count += 1;
                        if count == 2 {
                            return i + 1;
                        }
                    }
                }
                0
            })
            .unwrap_or(0)
    } else {
        0
    };

    let mut current_block = String::new();
    let mut block_start = 0usize;
    let mut in_code_block = false;

    for (i, line) in body.lines().enumerate() {
        let trimmed = line.trim();
        let actual_line = i + body_offset + 1;

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }
        if trimmed.starts_with('#') || trimmed.is_empty() {
            if !current_block.trim().is_empty() {
                rules.push(Rule {
                    id: claude::content_hash(&current_block),
                    source: Source {
                        tool: file.tool,
                        file_path: file.path.clone(),
                        line_start: block_start,
                        line_end: actual_line,
                    },
                    topics: claude::extract_topics(&current_block),
                    content: current_block.clone(),
                    activation: activation.clone(),
                });
                current_block.clear();
            }
            continue;
        }

        if current_block.is_empty() {
            block_start = actual_line;
        }
        if !current_block.is_empty() {
            current_block.push('\n');
        }
        current_block.push_str(trimmed);
    }

    if !current_block.trim().is_empty() {
        let end = body.lines().count() + body_offset;
        rules.push(Rule {
            id: claude::content_hash(&current_block),
            source: Source {
                tool: file.tool,
                file_path: file.path.clone(),
                line_start: block_start,
                line_end: end,
            },
            topics: claude::extract_topics(&current_block),
            content: current_block,
            activation: activation.clone(),
        });
    }

    rules
}

fn split_frontmatter(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content.to_string());
    }

    // Find closing ---
    let after_first = &trimmed[3..].trim_start_matches(['\r', '\n']);
    if let Some(end) = after_first.find("\n---") {
        let fm = after_first[..end].to_string();
        let body = after_first[end + 4..].to_string();
        (Some(fm), body)
    } else {
        (None, content.to_string())
    }
}

fn parse_activation(frontmatter: &str) -> Activation {
    // Simple YAML parsing for activation fields
    let mut globs = Vec::new();
    let mut always_apply = false;
    let mut description = None;

    for line in frontmatter.lines() {
        let line = line.trim();
        if line.starts_with("alwaysApply:") {
            let val = line.trim_start_matches("alwaysApply:").trim();
            always_apply = val == "true";
        } else if line.starts_with("globs:") {
            let val = line.trim_start_matches("globs:").trim().trim_matches('"');
            if !val.is_empty() {
                globs = val.split(',').map(|s| s.trim().to_string()).collect();
            }
        } else if line.starts_with("description:") {
            let val = line.trim_start_matches("description:").trim().trim_matches('"');
            if !val.is_empty() {
                description = Some(val.to_string());
            }
        }
    }

    if always_apply {
        Activation::Always
    } else if !globs.is_empty() {
        Activation::GlobPattern(globs)
    } else if let Some(desc) = description {
        Activation::Description(desc)
    } else {
        Activation::Manual
    }
}
