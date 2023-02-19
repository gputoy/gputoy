use thiserror::Error;

#[derive(Debug, Error)]
pub enum RootError {
    #[cfg(feature = "wasm")]
    #[error("Invalid utf-8 for string: {0}")]
    InvalidStringFormat(js_sys::JsString),
    #[error("Invalid file path: {0}")]
    InvalidFilePath(String),
}
