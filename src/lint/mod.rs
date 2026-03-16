pub mod breadth;
pub mod conflict;
pub mod duplicate;
pub mod format;
pub mod stale;

use crate::core::config::VcxConfig;
use crate::core::diagnostic::Diagnostic;
use crate::core::rule::ToolContext;
use std::path::Path;

/// Run all lint rules and collect diagnostics
pub fn run_all(
    contexts: &[ToolContext],
    project_dir: &Path,
    config: &VcxConfig,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let rule_sets: Vec<Box<dyn Fn(&[ToolContext], &Path) -> Vec<Diagnostic>>> = vec![
        Box::new(|ctx, _dir| conflict::check_style_conflicts(ctx)),
        Box::new(|ctx, _dir| conflict::check_tooling_conflicts(ctx)),
        Box::new(|ctx, dir| stale::check_file_references(ctx, dir)),
        Box::new(|ctx, _dir| stale::check_deprecated_formats(ctx)),
        Box::new(|ctx, _dir| duplicate::check_exact_duplicates(ctx)),
        Box::new(|ctx, _dir| duplicate::check_cross_tool_duplicates(ctx)),
        Box::new(|ctx, _dir| format::check_encoding(ctx)),
        Box::new(|ctx, _dir| breadth::check_file_size(ctx, config)),
    ];

    for rule_fn in &rule_sets {
        let results = rule_fn(contexts, project_dir);
        for d in results {
            if !config.is_rule_disabled(&d.rule_id) {
                diagnostics.push(d);
            }
        }
    }

    // Sort by severity (errors first), then file, then line
    diagnostics.sort_by(|a, b| {
        b.severity
            .cmp(&a.severity)
            .then_with(|| a.file.cmp(&b.file))
            .then_with(|| a.line.cmp(&b.line))
    });

    diagnostics
}
