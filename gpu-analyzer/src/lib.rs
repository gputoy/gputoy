mod analyzer;
mod diagnostics;
pub mod model;
mod types;

#[cfg(feature = "service")]
pub use analyzer::service;

pub use analyzer::Analyzer;
pub use diagnostics::Diagnostic;
pub use model::{Location, Model, ModelKey};

pub type Result<T> = std::result::Result<T, crate::Error>;

use {analyzer::Stage, types::resolver::ModelImport};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}-stage result not found for model@{1}.")]
    StoreValueVacant(Stage, ModelKey),
    #[error("Store value invalidated for model@{0}")]
    StoreValueInvalidated(ModelKey),
    #[error(transparent)]
    ModelError(#[from] crate::model::Error),
    #[error("Could not find import with identifier at given location: {0}")]
    ImportNotFound(Location),
    #[error("Model import is not valid {0}")]
    StaleImport(ModelImport),
    #[error("Parse error at {0}: {1}")]
    Parse(ModelKey, Box<naga::front::wgsl::ParseError>),
    #[error("Dispatch failed")]
    DispatchFailure(Stage, ModelKey, usize),
}
