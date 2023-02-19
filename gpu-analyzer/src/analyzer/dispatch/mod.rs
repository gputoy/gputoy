use crate::{
    diagnostics::{CatchDiagnostic, DiagnosticSink},
    Analyzer, Diagnostic, ModelKey,
};

use self::store::StoreProvider;

use super::{
    ExportProcess, ImportProcess, LinkerProcess, ParserProcess, Process, ProcessRecord,
    ProcessStatus, Stage,
};

#[macro_use]
mod macros;
mod store;

pub use macros::*;
pub use store::{DispatchStorage, ProcessStore};

pub struct Dispatcher;

dispatch! {
    export: ExportProcess,
    import: ImportProcess,
    link: LinkerProcess,
    parser: ParserProcess,
}

impl Dispatcher {
    pub fn dispatch(analyzer: &mut Analyzer) -> DispatchResult {
        let (manifest, diagnostics) = Dispatcher::_dispatch_all(analyzer);
        let enricher = analyzer.enricher();
        DispatchResult {
            manifest,
            diagnostics: diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.into_diagnostic(enricher))
                .collect(),
        }
    }

    fn dispatch_process<Proc: Process>(
        analyzer: &mut Analyzer,
        manifest: &mut DispatchManifest,
    ) -> DiagnosticSink
    where
        DispatchStorage: StoreProvider<Proc>,
    {
        let stage = Proc::STAGE_VARIANT;
        let keys = Self::select_keys_for_stage(analyzer, stage);

        let mut sink = DiagnosticSink::new(stage);

        for key in keys {
            sink.prep_dispatch();
            let maybe_result = Proc::process(analyzer, key, &mut sink).catch(&mut sink);

            let num_diagnostics = sink.num_diagnostics_for_dispatch();
            let should_fail = sink.should_fail_dispatch();

            let status = match (num_diagnostics, should_fail) {
                (0, _) => ProcessStatus::Success,
                (_, true) => ProcessStatus::Failure(num_diagnostics),
                (_, false) => ProcessStatus::Warn(num_diagnostics),
            };

            manifest.records.push(ProcessRecord { stage, key, status });

            if let Some(result) = maybe_result {
                analyzer.store.set_result::<Proc>(key, result);
            }
        }

        sink
    }

    fn select_keys_for_stage(
        analyzer: &mut Analyzer,
        stage: Stage,
    ) -> impl Iterator<Item = ModelKey> {
        let mut batch = Vec::new();
        for (key, _) in analyzer.model_cache.shaders() {
            let progress = analyzer.store.process_progress_for(key);
            if stage.is_after(progress.stage) {
                batch.push(key);
            }
        }
        batch.into_iter()
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "dev", derive(serde::Serialize))]
pub struct DispatchResult {
    manifest: DispatchManifest,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "dev", derive(serde::Serialize))]
pub struct DispatchManifest {
    records: Vec<ProcessRecord>,
}
