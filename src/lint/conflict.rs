use crate::core::diagnostic::*;
use crate::core::rule::*;

/// Style-related conflict patterns to detect
const STYLE_CONFLICTS: &[(&[&str], &[&str], &str)] = &[
    // (pattern_a keywords, pattern_b keywords, description)
    (
        &["use tab", "tabs for indent", "indent with tab"],
        &["use space", "spaces for indent", "indent with space", "2-space", "4-space", "2 space", "4 space"],
        "indentation style",
    ),
    (
        &["single quote", "single-quote", "use '"],
        &["double quote", "double-quote", "use \""],
        "quote style",
    ),
    (
        &["with semicolon", "use semicolon", "always semicolon"],
        &["no semicolon", "without semicolon", "omit semicolon"],
        "semicolon usage",
    ),
    (
        &["camelcase", "camel case", "camel-case"],
        &["snake_case", "snake case"],
        "naming convention",
    ),
];

/// Check for contradictory style instructions across all contexts
pub fn check_style_conflicts(contexts: &[ToolContext]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Collect all rules across all tools
    let all_rules: Vec<&Rule> = contexts.iter().flat_map(|c| c.rules.iter()).collect();

    for (pattern_a, pattern_b, desc) in STYLE_CONFLICTS {
        let matches_a: Vec<&Rule> = all_rules
            .iter()
            .filter(|r| matches_any_pattern(&r.content, pattern_a))
            .copied()
            .collect();

        let matches_b: Vec<&Rule> = all_rules
            .iter()
            .filter(|r| matches_any_pattern(&r.content, pattern_b))
            .copied()
            .collect();

        // Report conflicts between any pair
        for rule_a in &matches_a {
            for rule_b in &matches_b {
                // Skip if same file and same line range (likely same rule)
                if rule_a.source.file_path == rule_b.source.file_path
                    && rule_a.source.line_start == rule_b.source.line_start
                {
                    continue;
                }

                diagnostics.push(Diagnostic {
                    rule_id: "conflict/style".to_string(),
                    severity: Severity::Error,
                    message: format!(
                        "Conflicting {} instructions: \"{}\" vs \"{}\"",
                        desc,
                        truncate(&rule_a.content, 60),
                        truncate(&rule_b.content, 60),
                    ),
                    file: rule_a.source.file_path.clone(),
                    line: rule_a.source.line_start,
                    related: Some(RelatedLocation {
                        file: rule_b.source.file_path.clone(),
                        line: rule_b.source.line_start,
                        message: format!("Conflicting instruction here"),
                    }),
                });
            }
        }
    }

    // Deduplicate (A vs B and B vs A)
    dedup_diagnostics(&mut diagnostics);
    diagnostics
}

/// Tooling conflict patterns
const TOOLING_CONFLICTS: &[(&[&str], &str)] = &[
    (&["npm", "yarn", "pnpm", "bun"], "package manager"),
    (&["webpack", "vite", "esbuild", "turbopack", "rollup"], "bundler"),
    (&["jest", "vitest", "mocha", "ava"], "test framework"),
];

/// Check for contradictory tooling preferences
pub fn check_tooling_conflicts(contexts: &[ToolContext]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let all_rules: Vec<&Rule> = contexts.iter().flat_map(|c| c.rules.iter()).collect();

    for (tools, category) in TOOLING_CONFLICTS {
        // Find rules that mention each tool
        let mut tool_mentions: Vec<(&str, &Rule)> = Vec::new();

        for rule in &all_rules {
            let lower = rule.content.to_lowercase();
            for tool_name in *tools {
                // Check for "use X" or "prefer X" patterns (not just mentioning)
                if lower.contains(&format!("use {}", tool_name))
                    || lower.contains(&format!("prefer {}", tool_name))
                    || lower.contains(&format!("with {}", tool_name))
                {
                    tool_mentions.push((tool_name, rule));
                }
            }
        }

        // Check for conflicting tool preferences
        for i in 0..tool_mentions.len() {
            for j in (i + 1)..tool_mentions.len() {
                let (tool_a, rule_a) = tool_mentions[i];
                let (tool_b, rule_b) = tool_mentions[j];

                if tool_a != tool_b {
                    diagnostics.push(Diagnostic {
                        rule_id: "conflict/tooling".to_string(),
                        severity: Severity::Error,
                        message: format!(
                            "Conflicting {} preference: \"{}\" vs \"{}\"",
                            category, tool_a, tool_b,
                        ),
                        file: rule_a.source.file_path.clone(),
                        line: rule_a.source.line_start,
                        related: Some(RelatedLocation {
                            file: rule_b.source.file_path.clone(),
                            line: rule_b.source.line_start,
                            message: format!("Conflicting preference here"),
                        }),
                    });
                }
            }
        }
    }

    dedup_diagnostics(&mut diagnostics);
    diagnostics
}

fn matches_any_pattern(content: &str, patterns: &[&str]) -> bool {
    let lower = content.to_lowercase();
    patterns.iter().any(|p| lower.contains(p))
}

fn truncate(s: &str, max: usize) -> String {
    let single_line = s.replace('\n', " ");
    if single_line.len() <= max {
        single_line
    } else {
        format!("{}...", &single_line[..max])
    }
}

fn dedup_diagnostics(diagnostics: &mut Vec<Diagnostic>) {
    let mut seen = std::collections::HashSet::new();
    diagnostics.retain(|d| {
        // Create a key that's order-independent for the file pair
        let key = if let Some(ref rel) = d.related {
            let mut files = vec![
                format!("{}:{}", d.file.display(), d.line),
                format!("{}:{}", rel.file.display(), rel.line),
            ];
            files.sort();
            format!("{}|{}|{}", d.rule_id, files[0], files[1])
        } else {
            format!("{}|{}:{}", d.rule_id, d.file.display(), d.line)
        };
        seen.insert(key)
    });
}
