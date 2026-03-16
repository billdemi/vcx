use colored::*;
use comfy_table::{Cell, ContentArrangement, Table};

use crate::core::diagnostic::*;
use crate::core::rule::ContextFile;
use crate::detect::scanner;

/// Format scan results as a table
pub fn format_scan(files: &[ContextFile], project_dir: &std::path::Path) -> String {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Tool", "File", "Size", "Modified", "Status"]);

    // Try to get canonical project dir for prefix stripping
    let canonical_dir = std::fs::canonicalize(project_dir).unwrap_or(project_dir.to_path_buf());

    for file in files {
        let canonical_file = std::fs::canonicalize(&file.path).unwrap_or(file.path.clone());
        let rel_path = canonical_file
            .strip_prefix(&canonical_dir)
            .unwrap_or(&file.path)
            .to_string_lossy()
            .to_string();

        let size = scanner::format_size(file.size_bytes);
        let modified = file
            .modified
            .as_ref()
            .map(|m| scanner::format_relative_time(m))
            .unwrap_or_else(|| "unknown".to_string());

        let status = if file.deprecated {
            "deprecated"
        } else {
            "ok"
        };

        table.add_row(vec![
            Cell::new(file.tool.display_name()),
            Cell::new(&rel_path),
            Cell::new(&size),
            Cell::new(&modified),
            Cell::new(status),
        ]);
    }

    // Add missing tools
    let detected = scanner::detected_tools(files);
    let missing = scanner::missing_tools(files);
    for tool in &missing {
        table.add_row(vec![
            Cell::new(tool.display_name()),
            Cell::new("(not found)"),
            Cell::new("-"),
            Cell::new("-"),
            Cell::new("missing"),
        ]);
    }

    let mut output = table.to_string();
    output.push_str(&format!(
        "\n\nFound {} context file(s) for {}/{} tools.",
        files.len(),
        detected.len(),
        detected.len() + missing.len(),
    ));

    output
}

/// Format lint diagnostics as text
pub fn format_lint(diagnostics: &[Diagnostic], project_dir: &std::path::Path) -> String {
    if diagnostics.is_empty() {
        return "No problems found.".to_string();
    }

    let mut output = String::new();
    let canonical_dir = std::fs::canonicalize(project_dir).unwrap_or(project_dir.to_path_buf());

    for d in diagnostics {
        let canonical_file = std::fs::canonicalize(&d.file).unwrap_or(d.file.clone());
        let rel_path = canonical_file
            .strip_prefix(&canonical_dir)
            .unwrap_or(&d.file)
            .to_string_lossy();

        let severity_str = match d.severity {
            Severity::Error => "error".red().bold().to_string(),
            Severity::Warning => "warn".yellow().bold().to_string(),
            Severity::Info => "info".blue().to_string(),
        };

        output.push_str(&format!(
            "{}:{}  {}  {}  {}\n",
            rel_path, d.line, severity_str, d.rule_id, d.message,
        ));

        if let Some(ref related) = d.related {
            let canonical_related = std::fs::canonicalize(&related.file).unwrap_or(related.file.clone());
            let rel_related = canonical_related
                .strip_prefix(&canonical_dir)
                .unwrap_or(&related.file)
                .to_string_lossy();
            output.push_str(&format!(
                "  {}:{}  {}\n",
                rel_related, related.line, related.message,
            ));
        }

        output.push('\n');
    }

    // Summary
    let mut summary = LintSummary::default();
    for d in diagnostics {
        summary.add(d.severity);
    }
    output.push_str(&format!(
        "{} problem(s) ({} error, {} warning, {} info)\n",
        summary.total(),
        summary.errors,
        summary.warnings,
        summary.infos,
    ));

    output
}

/// Format status dashboard
pub fn format_status(
    files: &[ContextFile],
    diagnostics: &[Diagnostic],
    rule_count: usize,
    unique_rules: usize,
) -> String {
    let detected = scanner::detected_tools(files);
    let missing = scanner::missing_tools(files);

    let mut summary = LintSummary::default();
    for d in diagnostics {
        summary.add(d.severity);
    }

    // Health score: start at 100, subtract for issues
    let mut score: i32 = 100;
    score -= (summary.errors as i32) * 15;
    score -= (summary.warnings as i32) * 5;
    score -= (summary.infos as i32) * 1;
    if missing.len() > 2 {
        score -= 10;
    }
    let score = score.max(0).min(100) as usize;

    let bar_filled = score / 10;
    let bar_empty = 10 - bar_filled;
    let bar = format!(
        "{}{}",
        "█".repeat(bar_filled),
        "░".repeat(bar_empty),
    );

    let tools_str = detected
        .iter()
        .map(|t| t.display_name())
        .collect::<Vec<_>>()
        .join(", ");

    let missing_str = if missing.is_empty() {
        "none".to_string()
    } else {
        missing
            .iter()
            .map(|t| t.display_name())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let duplicated = rule_count - unique_rules;

    format!(
        r#"VCX Context Health Report
═════════════════════════

Tools configured:  {} ({})
Tools missing:     {} ({})
Total rules:       {} ({} unique, {} duplicated)
Lint:              {} error, {} warning, {} info

Health score:      {}/100  ({})

Run `vcx lint` for details."#,
        detected.len(),
        tools_str,
        missing.len(),
        missing_str,
        rule_count,
        unique_rules,
        duplicated,
        summary.errors,
        summary.warnings,
        summary.infos,
        score,
        bar,
    )
}
