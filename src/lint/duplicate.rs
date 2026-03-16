use crate::core::diagnostic::*;
use crate::core::rule::*;

/// Check for exactly duplicated rules within the same tool
pub fn check_exact_duplicates(contexts: &[ToolContext]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for ctx in contexts {
        let rules = &ctx.rules;
        for i in 0..rules.len() {
            for j in (i + 1)..rules.len() {
                if rules[i].id == rules[j].id
                    && rules[i].source.file_path != rules[j].source.file_path
                {
                    diagnostics.push(Diagnostic {
                        rule_id: "duplicate/exact".to_string(),
                        severity: Severity::Warning,
                        message: format!(
                            "Duplicate instruction: \"{}\"",
                            truncate(&rules[i].content, 80),
                        ),
                        file: rules[i].source.file_path.clone(),
                        line: rules[i].source.line_start,
                        related: Some(RelatedLocation {
                            file: rules[j].source.file_path.clone(),
                            line: rules[j].source.line_start,
                            message: "Same instruction here".to_string(),
                        }),
                    });
                }
            }
        }
    }

    diagnostics
}

/// Check for duplicated rules across different tools
pub fn check_cross_tool_duplicates(contexts: &[ToolContext]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Collect all rules with their tool info
    let all_rules: Vec<(&ToolKind, &Rule)> = contexts
        .iter()
        .flat_map(|c| c.rules.iter().map(move |r| (&c.tool, r)))
        .collect();

    for i in 0..all_rules.len() {
        for j in (i + 1)..all_rules.len() {
            let (tool_a, rule_a) = all_rules[i];
            let (tool_b, rule_b) = all_rules[j];

            // Only report cross-tool duplicates
            if tool_a != tool_b && rule_a.id == rule_b.id {
                diagnostics.push(Diagnostic {
                    rule_id: "duplicate/cross-tool".to_string(),
                    severity: Severity::Info,
                    message: format!(
                        "Same instruction in {} and {}: \"{}\"",
                        tool_a.display_name(),
                        tool_b.display_name(),
                        truncate(&rule_a.content, 60),
                    ),
                    file: rule_a.source.file_path.clone(),
                    line: rule_a.source.line_start,
                    related: Some(RelatedLocation {
                        file: rule_b.source.file_path.clone(),
                        line: rule_b.source.line_start,
                        message: format!("Duplicate in {}", tool_b.display_name()),
                    }),
                });
            }
        }
    }

    diagnostics
}

fn truncate(s: &str, max: usize) -> String {
    let single_line = s.replace('\n', " ");
    if single_line.len() <= max {
        single_line
    } else {
        format!("{}...", &single_line[..max])
    }
}
