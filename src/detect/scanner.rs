use anyhow::Result;
use std::path::Path;

use crate::core::rule::{ContextFile, ToolKind};
use crate::detect::patterns::{all_patterns, FilePattern};

/// Scan a project directory for all AI context files
pub fn scan_project(project_dir: &Path) -> Result<Vec<ContextFile>> {
    let mut found = Vec::new();

    // Canonicalize project dir for consistent path handling
    let project_dir = if project_dir.is_relative() {
        std::env::current_dir()?.join(project_dir)
    } else {
        project_dir.to_path_buf()
    };
    let project_dir = &project_dir;

    for pattern in all_patterns() {
        let full_pattern = project_dir.join(pattern.pattern);
        let pattern_str = full_pattern.to_string_lossy().replace('\\', "/");

        match glob::glob(&pattern_str) {
            Ok(paths) => {
                for entry in paths.flatten() {
                    if let Some(cf) = context_file_from_path(&entry, &pattern) {
                        found.push(cf);
                    }
                }
            }
            Err(_) => {
                // Invalid glob pattern, skip
            }
        }
    }

    // Sort by tool name then path
    found.sort_by(|a, b| {
        a.tool
            .display_name()
            .cmp(b.tool.display_name())
            .then_with(|| a.path.cmp(&b.path))
    });

    Ok(found)
}

/// Get the set of tools that have context files
pub fn detected_tools(files: &[ContextFile]) -> Vec<ToolKind> {
    let mut tools: Vec<ToolKind> = files.iter().map(|f| f.tool).collect();
    tools.sort_by_key(|t| t.display_name().to_string());
    tools.dedup();
    tools
}

/// Get tools that have NO context files
pub fn missing_tools(files: &[ContextFile]) -> Vec<ToolKind> {
    let detected = detected_tools(files);
    ToolKind::all()
        .iter()
        .filter(|t| !detected.contains(t))
        .copied()
        .collect()
}

fn context_file_from_path(path: &Path, pattern: &FilePattern) -> Option<ContextFile> {
    let meta = std::fs::metadata(path).ok()?;
    let modified = meta
        .modified()
        .ok()
        .map(|t| chrono::DateTime::<chrono::Utc>::from(t));

    Some(ContextFile {
        tool: pattern.tool,
        path: path.to_path_buf(),
        size_bytes: meta.len(),
        modified,
        format: pattern.format,
        deprecated: pattern.deprecated,
    })
}

/// Format file size in human-readable form
pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}K", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Format time as relative (e.g., "2h ago", "3d ago")
pub fn format_relative_time(dt: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(*dt);

    if diff.num_minutes() < 1 {
        "just now".to_string()
    } else if diff.num_hours() < 1 {
        format!("{}m ago", diff.num_minutes())
    } else if diff.num_days() < 1 {
        format!("{}h ago", diff.num_hours())
    } else if diff.num_weeks() < 1 {
        format!("{}d ago", diff.num_days())
    } else if diff.num_weeks() < 5 {
        format!("{}w ago", diff.num_weeks())
    } else {
        format!("{}mo ago", diff.num_days() / 30)
    }
}
