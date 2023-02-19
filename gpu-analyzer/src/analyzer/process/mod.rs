use std::fmt::Display;

use crate::{
    diagnostics::DiagnosticSink,
    model::{ModelKey, ModelVersion},
    Analyzer,
};

mod export;
mod import;
mod linker;
mod parser;

pub use export::{ExportProcess, ExportStore};
pub use import::{ImportProcess, ImportStore};
pub use linker::{LinkerProcess, LinkerStore};
pub use parser::{ParserProcess, ParserStore};

use super::ProcessStore;

#[derive(Debug, Default, Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum Stage {
    #[default]
    Unprocessed,
    ExportResolver,
    ImportResolver,
    Linker,
    Parser,
}

impl Stage {
    pub fn is_after(self, other: Self) -> bool {
        use Stage::*;
        matches!(
            (self, other),
            (Unprocessed, Unprocessed)
                | (ExportResolver, Unprocessed)
                | (ImportResolver, ExportResolver)
                | (Linker, ImportResolver)
                | (Parser, Linker)
        )
    }
}

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unprocessed => "unprocessed",
                Self::ExportResolver => "exports",
                Self::ImportResolver => "imports",
                Self::Linker => "linker",
                Self::Parser => "parser",
            }
        )
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub struct ProcessProgress {
    pub version: ModelVersion,
    pub stage: Stage,
}

pub trait Process: Sized {
    type Result: Sized + std::fmt::Debug;
    type Store: ProcessStore<Self>;

    const PROCESS_NAME: &'static str;
    const STAGE_VARIANT: Stage;

    fn process(
        analyzer: &mut Analyzer,
        key: ModelKey,
        sink: &mut DiagnosticSink,
    ) -> crate::Result<Self::Result>;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "dev", derive(serde::Serialize))]
pub struct ProcessRecord {
    pub stage: Stage,
    #[serde(serialize_with = "crate::model::serialize_key")]
    pub key: ModelKey,
    pub status: ProcessStatus,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "dev", derive(serde::Serialize))]
pub enum ProcessStatus {
    Success,
    Warn(usize),
    Failure(usize),
}
