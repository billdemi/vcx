use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;

use crate::core::config::VcxConfig;
use crate::detect::scanner;
use crate::lint;
use crate::output;
use crate::parsers;

pub fn execute(project_dir: &Path) -> Result<()> {
    let config = VcxConfig::load(project_dir);
    let files = scanner::scan_project(project_dir)?;

    if files.is_empty() {
        println!("No context files found. Run `vcx init` to get started.");
        return Ok(());
    }

    let contexts = parsers::parse_all(&files, project_dir);
    let diagnostics = lint::run_all(&contexts, project_dir, &config);

    // Count rules
    let total_rules: usize = contexts.iter().map(|c| c.rules.len()).sum();
    let unique_ids: HashSet<&str> = contexts
        .iter()
        .flat_map(|c| c.rules.iter().map(|r| r.id.as_str()))
        .collect();

    let output_str = output::table::format_status(
        &files,
        &diagnostics,
        total_rules,
        unique_ids.len(),
    );

    println!("{}", output_str);
    Ok(())
}
