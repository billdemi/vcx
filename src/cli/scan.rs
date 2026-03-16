use anyhow::Result;
use std::path::Path;

use crate::detect::scanner;
use crate::output;

pub fn execute(project_dir: &Path, format: &str) -> Result<()> {
    let files = scanner::scan_project(project_dir)?;

    let output_str = match format {
        "json" => output::json::format_scan(&files),
        _ => output::table::format_scan(&files, project_dir),
    };

    println!("{}", output_str);
    Ok(())
}
