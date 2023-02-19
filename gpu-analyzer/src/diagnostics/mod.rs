use codespan_reporting::diagnostic as term;
use gpu_common::FilePath;

mod error;
mod sink;

pub use error::CatchDiagnostic;
pub use sink::DiagnosticSink;

use crate::{Location, ModelKey};

pub type Label = term::Label<FilePath>;
pub type Severity = term::Severity;

/// An exact copy of
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Diagnostic {
    /// The overall severity of the diagnostic
    pub severity: Severity,
    /// An optional code that identifies this diagnostic.
    pub code: Option<String>,
    /// The main message associated with this diagnostic.
    ///
    /// These should not include line breaks, and in order support the 'short'
    /// diagnostic display mod, the message should be specific enough to make
    /// sense on its own, without additional context provided by labels and notes.
    pub message: String,
    /// Source labels that describe the cause of the diagnostic.
    /// The order of the labels inside the vector does not have any meaning.
    /// The labels are always arranged in the order they appear in the source code.
    pub labels: Vec<Label>,
    /// Notes that are associated with the primary cause of the diagnostic.
    /// These can include line breaks for improved formatting.
    pub notes: Vec<String>,
}

impl Default for Diagnostic {
    fn default() -> Self {
        Self {
            severity: Severity::Error,
            code: None,
            message: String::new(),
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }
}

impl From<Diagnostic> for term::Diagnostic<FilePath> {
    fn from(diagnostic: Diagnostic) -> Self {
        Self {
            code: diagnostic.code,
            labels: diagnostic.labels,
            message: diagnostic.message,
            notes: diagnostic.notes,
            severity: diagnostic.severity,
        }
    }
}

pub trait IntoDiagnostic: std::fmt::Debug {
    fn into_diagnostic(self: Box<Self>, e: DiagnosticEnricher) -> Diagnostic;
    fn severity(&self) -> Severity;
    fn code(&self) -> &'static str;

    fn recoverable(&self) -> bool {
        self.severity() > Severity::Warning
    }

    fn boxed(self) -> Box<dyn IntoDiagnostic>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

#[derive(Clone, Copy)]
pub struct DiagnosticEnricher<'a>(&'a crate::Analyzer);

impl<'a> DiagnosticEnricher<'a> {
    const UNKNOWN_SPAN: &'static str = "unknown";

    pub fn new(analyzer: &'a crate::Analyzer) -> Self {
        Self(analyzer)
    }

    pub fn src(&'a self, location: &Location) -> &'a str {
        self.0
            .model_cache
            .get_location(location)
            .unwrap_or(Self::UNKNOWN_SPAN)
    }

    pub fn path(&'a self, key: ModelKey) -> FilePath {
        self.0
            .model_cache
            .path(key)
            .cloned()
            .unwrap_or(FilePath::unknown())
    }
}
