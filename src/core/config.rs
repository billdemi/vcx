use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// VCX configuration from .vcx.toml
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VcxConfig {
    #[serde(default)]
    pub vcx: VcxSection,
    #[serde(default)]
    pub scan: ScanSection,
    #[serde(default)]
    pub lint: LintSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VcxSection {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub tools: Vec<String>,
    pub source_of_truth: Option<String>,
}

impl Default for VcxSection {
    fn default() -> Self {
        Self {
            version: default_version(),
            tools: Vec::new(),
            source_of_truth: None,
        }
    }
}

fn default_version() -> String {
    "0.1".to_string()
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScanSection {
    #[serde(default)]
    pub extra_files: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LintSection {
    #[serde(default)]
    pub disable: Vec<String>,
    #[serde(default)]
    pub severity: HashMap<String, String>,
}

impl VcxConfig {
    pub fn load(project_dir: &Path) -> Self {
        let config_path = project_dir.join(".vcx.toml");
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn is_rule_disabled(&self, rule_id: &str) -> bool {
        self.lint.disable.contains(&rule_id.to_string())
            || self
                .lint
                .severity
                .get(rule_id)
                .map_or(false, |s| s == "off")
    }
}
