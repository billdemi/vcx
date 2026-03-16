use anyhow::Result;
use std::path::Path;
use std::process;

use crate::core::config::VcxConfig;
use crate::detect::scanner;
use crate::lint;
use crate::output;
use crate::parsers;

pub fn execute(project_dir: &Path, format: &str) -> Result<()> {
    let config = VcxConfig::load(project_dir);
    let files = scanner::scan_project(project_dir)?;

    if files.is_empty() {
        println!("No context files found. Run `vcx init` to get started.");
        return Ok(());
    }

    let contexts = parsers::parse_all(&files, project_dir);
    let diagnostics = lint::run_all(&contexts, project_dir, &config);

    let output_str = match format {
        "json" => output::json::format_lint(&diagnostics),
        _ => output::table::format_lint(&diagnostics, project_dir),
    };

    println!("{}", output_str);

    // Exit with code 1 if there are errors (CI-friendly)
    if diagnostics.iter().any(|d| d.severity == crate::core::diagnostic::Severity::Error) {
        process::exit(1);
    }

    Ok(())
}
