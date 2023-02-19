use gpu_common::{FilePath, Files};
use naga::front::wgsl;
use std::cell::RefCell;

use crate::{diagnostics::DiagnosticEnricher, model::ModelCache};

#[cfg(feature = "dev")]
pub mod json;
#[cfg(feature = "service")]
pub mod service;

mod dispatch;
mod process;

pub use dispatch::*;
pub use process::*;

pub struct Analyzer {
    pub(crate) wgsl_parser: RefCell<wgsl::Frontend>,
    pub(crate) model_cache: crate::model::ModelCache,
    pub(crate) store: DispatchStorage,
}

impl Analyzer {
    pub fn new(init_files: Files) -> Self {
        let wgsl_parser = RefCell::new(naga::front::wgsl::Frontend::new());
        let model_cache = ModelCache::from_files(init_files);
        let store = DispatchStorage::with_capacity(model_cache.capacity());

        Self {
            wgsl_parser,
            model_cache,
            store,
        }
    }

    pub fn reset(&mut self, files: Files) {
        *self = Self::new(files);
    }

    pub fn try_build(&mut self, _runner_path: &FilePath) -> crate::Result<DispatchResult> {
        // let runner = self.model_cache.get_model_by_path(runner_path)?;
        // let runner: gpu_common::Runner = runner.json()?;

        let diagnostics = Dispatcher::dispatch(self);
        Ok(diagnostics)
    }

    pub fn get_build() -> Option<()> {
        Some(())
    }

    pub fn enricher(&self) -> DiagnosticEnricher {
        DiagnosticEnricher::new(self)
    }
}
