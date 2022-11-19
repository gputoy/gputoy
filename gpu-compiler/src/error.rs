use naga::front::wgsl::ParseError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing file {0}: {1}")]
    ParseError(String, naga::front::wgsl::ParseError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompileError {
    pub message: String,
    pub span: Option<SourceLocation>,
}

impl From<(&ParseError, &str)> for CompileError {
    fn from((err, src): (&ParseError, &str)) -> Self {
        Self {
            message: err.emit_to_string(src),
            span: err.location(src).map(From::from),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceLocation {
    /// 1-based line number.
    pub line_number: u32,
    /// 1-based column of the start of this span
    pub line_position: u32,
    /// 0-based Offset in code units (in bytes) of the start of the span.
    pub offset: u32,
    /// Length in code units (in bytes) of the span.
    pub length: u32,
}

impl From<naga::SourceLocation> for SourceLocation {
    fn from(loc: naga::SourceLocation) -> Self {
        Self {
            line_number: loc.line_number,
            line_position: loc.line_position,
            offset: loc.offset,
            length: loc.length,
        }
    }
}
