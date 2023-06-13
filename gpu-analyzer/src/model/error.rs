use gpu_common::{FilePath, SupportedExtension};

use crate::{
    diagnostics::{Diagnostic, DiagnosticEnricher, IntoDiagnostic, Severity},
    ModelKey,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ModelKey not found for path: {0}")]
    ModelKeyNotFound(FilePath),
    #[error("ModelKey no longer points to valid data: {0:?}")]
    StaleKey(ModelKey),
    #[error("Invalid Json for {0}: {1}")]
    InvalidJson(FilePath, serde_json::Error),
    #[error("Expected .{expected} file, got {actual}")]
    UnexpectedExtension {
        expected: SupportedExtension,
        actual: FilePath,
    },
}

impl IntoDiagnostic for Error {
    fn into_diagnostic(self: Box<Self>, _e: DiagnosticEnricher) -> Diagnostic {
        let severity = self.severity();
        let diagnostic = match *self {
            Self::ModelKeyNotFound(path) => Diagnostic {
                message: format!("failed to fetch model data for path {path}"),
                ..Default::default()
            },
            Self::StaleKey(key) => Diagnostic {
                message: format!("model key {key} no longer points to valid data"),
                ..Default::default()
            },
            Self::InvalidJson(path, error) => Diagnostic {
                message: format!("invalid json in {path}: {error}"),
                ..Default::default()
            },
            Self::UnexpectedExtension { expected, actual } => Diagnostic {
                message: format!("expected .{expected} file, got {actual}"),
                ..Default::default()
            },
        };
        Diagnostic {
            severity,
            ..diagnostic
        }
    }

    fn severity(&self) -> Severity {
        match self {
            Self::InvalidJson(_, _) | Self::UnexpectedExtension { .. } => Severity::Error,
            _ => Severity::Bug,
        }
    }

    #[allow(clippy::match_single_binding)]
    fn code(&self) -> &'static str {
        match self {
            _ => "E001",
        }
    }
}
