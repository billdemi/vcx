use serde::Serialize;
use std::path::PathBuf;

/// Severity levels for lint diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warning => write!(f, "warn"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// A lint diagnostic
#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub rule_id: String,
    pub severity: Severity,
    pub message: String,
    pub file: PathBuf,
    pub line: usize,
    /// Optional related location (for cross-file conflicts)
    pub related: Option<RelatedLocation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RelatedLocation {
    pub file: PathBuf,
    pub line: usize,
    pub message: String,
}

/// Summary of lint results
#[derive(Debug, Default)]
pub struct LintSummary {
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
}

impl LintSummary {
    pub fn add(&mut self, severity: Severity) {
        match severity {
            Severity::Error => self.errors += 1,
            Severity::Warning => self.warnings += 1,
            Severity::Info => self.infos += 1,
        }
    }

    pub fn total(&self) -> usize {
        self.errors + self.warnings + self.infos
    }

    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }
}
