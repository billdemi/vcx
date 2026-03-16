use anyhow::Result;
use std::path::Path;

use crate::core::rule::ToolKind;

pub fn execute(project_dir: &Path, tools: &[String]) -> Result<()> {
    // Ensure project dir exists
    std::fs::create_dir_all(project_dir)?;

    // Create .vcx.toml
    let config_path = project_dir.join(".vcx.toml");
    if config_path.exists() {
        println!(".vcx.toml already exists, skipping.");
    } else {
        let tool_list = if tools.is_empty() {
            r#"["claude", "cursor", "copilot", "windsurf", "aider"]"#.to_string()
        } else {
            let quoted: Vec<String> = tools.iter().map(|t| format!("\"{}\"", t)).collect();
            format!("[{}]", quoted.join(", "))
        };

        let config_content = format!(
            r#"[vcx]
version = "0.1"
tools = {}
# source_of_truth = "claude"

[scan]
extra_files = []
exclude = ["node_modules", ".git", "dist", "target"]

[lint]
disable = []

# [lint.severity]
# "duplicate/cross-tool" = "off"
"#,
            tool_list,
        );

        std::fs::write(&config_path, &config_content)?;
        println!("Created .vcx.toml");
    }

    // Create template context files for requested tools
    let target_tools: Vec<ToolKind> = if tools.is_empty() {
        vec![ToolKind::Claude]
    } else {
        tools
            .iter()
            .filter_map(|t| t.parse::<ToolKind>().ok())
            .collect()
    };

    for tool in target_tools {
        create_template(project_dir, tool)?;
    }

    println!("\nRun `vcx scan` to verify your setup.");
    Ok(())
}

fn create_template(project_dir: &Path, tool: ToolKind) -> Result<()> {
    match tool {
        ToolKind::Claude => {
            let path = project_dir.join("CLAUDE.md");
            if !path.exists() {
                std::fs::write(
                    &path,
                    "# Project Guidelines\n\n## Code Style\n\n## Architecture\n\n## Testing\n",
                )?;
                println!("Created CLAUDE.md (template)");
            }
        }
        ToolKind::Cursor => {
            let dir = project_dir.join(".cursor/rules");
            std::fs::create_dir_all(&dir)?;
            let path = dir.join("project.mdc");
            if !path.exists() {
                std::fs::write(
                    &path,
                    "---\ndescription: Project conventions\nalwaysApply: true\n---\n\n# Project Guidelines\n",
                )?;
                println!("Created .cursor/rules/project.mdc (template)");
            }
        }
        ToolKind::Copilot => {
            let dir = project_dir.join(".github");
            std::fs::create_dir_all(&dir)?;
            let path = dir.join("copilot-instructions.md");
            if !path.exists() {
                std::fs::write(&path, "# Copilot Instructions\n\n")?;
                println!("Created .github/copilot-instructions.md (template)");
            }
        }
        ToolKind::Windsurf => {
            let path = project_dir.join(".windsurfrules");
            if !path.exists() {
                std::fs::write(&path, "# Windsurf Rules\n\n")?;
                println!("Created .windsurfrules (template)");
            }
        }
        ToolKind::Aider => {
            let path = project_dir.join(".aider.conf.yml");
            if !path.exists() {
                std::fs::write(&path, "# Aider configuration\n# read:\n#   - CONVENTIONS.md\n")?;
                println!("Created .aider.conf.yml (template)");
            }
        }
        ToolKind::Codex => {
            let path = project_dir.join("AGENTS.md");
            if !path.exists() {
                std::fs::write(&path, "# Agent Instructions\n\n")?;
                println!("Created AGENTS.md (template)");
            }
        }
    }

    Ok(())
}
