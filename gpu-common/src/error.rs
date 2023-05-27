#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq, displaydoc::Display)]
pub enum RootError {
    #[cfg(feature = "wasm")]
    /// "Invalid utf-8 for string: {0}"
    InvalidStringFormat(js_sys::JsString),
    /// Invalid path: {0}
    InvalidPath(String),
    /// Invalid file path: {0}
    InvalidFilePath(String),
    /// Missing arugment: {0}
    MissingArgument(String),
    /// Invalid pane: {0}
    InvalidPane(String),
    /// Cannot change dir to file and vice-versa
    InvalidCopyMove,
    /// Unknown action: {0}
    UnknownAction(String),
    /// Invalid ui region: {0}
    InvalidRegion(String),
    /// Invalid keycode: {0}
    InvalidKeycode(String),
    /// Unkown argument: {0}
    UnknownArgument(String),
    /// Invalid {0}: '{1}'
    InvalidValue(&'static str, String),
}

impl IntoClientError for RootError {
    fn into_client_error(self) -> ClientError {
        let message = self.to_string();
        ClientError {
            severity: Severity::Error,
            message,
            source: "gpu-common".into(),
            destination: Destination::Console,
        }
    }
}

pub trait IntoClientError {
    fn into_client_error(self) -> ClientError;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClientError {
    pub severity: Severity,
    pub message: String,
    pub source: String,
    pub destination: Destination,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum Destination {
    Console,
    Toast,
}
