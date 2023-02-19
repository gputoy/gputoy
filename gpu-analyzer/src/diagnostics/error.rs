use crate::Diagnostic;

use super::{DiagnosticEnricher, DiagnosticSink, IntoDiagnostic, Label};

pub trait CatchDiagnostic<T, E> {
    fn catch(self, sink: &mut DiagnosticSink) -> Option<T>;
}

impl<T, E> CatchDiagnostic<T, E> for Result<T, E>
where
    E: IntoDiagnostic + 'static,
{
    fn catch(self, sink: &mut DiagnosticSink) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(error) => {
                sink.push(error.boxed());
                None
            }
        }
    }
}

impl IntoDiagnostic for crate::Error {
    fn into_diagnostic(self: Box<Self>, e: DiagnosticEnricher) -> crate::Diagnostic {
        let severity = self.severity();

        let ret = match *self {
            Self::ImportNotFound(location) => Diagnostic {
                message: format!(
                    "import '{}' not found in any project files",
                    e.src(&location)
                ),
                labels: vec![Label::primary(e.path(location.model), location.range)
                    .with_message("unable to resolve")],
                ..Default::default()
            },

            Self::Parse(key, error) => {
                let diagnostic = error.diagnostic();
                Diagnostic {
                    code: diagnostic.code,
                    notes: diagnostic.notes,
                    message: diagnostic.message,
                    severity: diagnostic.severity,
                    labels: diagnostic
                        .labels
                        .into_iter()
                        .map(|label| Label {
                            file_id: e.path(key),
                            message: label.message,
                            range: label.range,
                            style: label.style,
                        })
                        .collect(),
                }
            }
            Self::DispatchFailure(stage, key, num_failures) => Diagnostic {
                message: format!(
                    "dispatch failed for {} at {}-stage with {} error{}",
                    e.path(key),
                    stage,
                    num_failures,
                    if num_failures > 1 { "s" } else { "" }
                ),
                ..Default::default()
            },
            Self::StaleImport(import) => Diagnostic {
                message: format!("import '{}' cannot be resolved", e.src(&import.location)),
                labels: vec![
                    Label::primary(e.path(import.location.model), import.location.range)
                        .with_message("unable to resolve"),
                ],
                ..Default::default()
            },
            Self::StoreValueVacant(stage, key) => Diagnostic {
                message: format!(
                    "cannot resolve {}-stage process result for {}",
                    stage,
                    e.path(key)
                ),
                ..Default::default()
            },
            Self::StoreValueInvalidated(_) => todo!(),
            Self::ModelError(error) => Box::new(error).into_diagnostic(e),
        };

        crate::Diagnostic { severity, ..ret }
    }

    fn severity(&self) -> super::Severity {
        match self {
            crate::Error::ImportNotFound(_)
            | crate::Error::Parse(_, _)
            | crate::Error::DispatchFailure(_, _, _) => super::Severity::Error,
            Self::ModelError(error) => error.severity(),
            _ => super::Severity::Bug,
        }
    }

    #[allow(clippy::match_single_binding)]
    fn code(&self) -> &'static str {
        match self {
            _ => "E001",
        }
    }
}
