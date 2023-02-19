use crate::{
    diagnostics::DiagnosticSink,
    model::{Location, ModelKey},
    types::resolver::{ModelExport, ModelImport, RE_CAPTURE_EXPORT},
};

use super::{ProcessStore, Stage};

pub struct ExportProcess;

crate::store_for!(ExportStore => ExportProcess);

impl ExportStore {
    pub fn resolve_import(&self, import: &ModelImport) -> crate::Result<&ModelExport> {
        self.get(import.from).and_then(|exports| {
            exports
                .get(import.idx)
                .ok_or(crate::Error::StaleImport(import.clone()))
        })
    }

    pub fn find_exported_ident(&self, ident: &str) -> Option<(ModelKey, usize)> {
        self.0.iter().find_map(|(key, exports)| {
            exports
                .iter()
                .enumerate()
                .find_map(|(idx, export)| (export.ident == ident).then_some((key, idx)))
        })
    }
}

impl super::Process for ExportProcess {
    type Result = Vec<ModelExport>;
    type Store = ExportStore;

    const PROCESS_NAME: &'static str = "exports";
    const STAGE_VARIANT: Stage = Stage::ExportResolver;

    fn process(
        analyzer: &mut crate::Analyzer,
        key: ModelKey,
        _error_sink: &mut DiagnosticSink,
    ) -> crate::Result<Self::Result> {
        let model = analyzer.model_cache.get_model(key)?;
        let exports = get_model_exports(key, model);

        Ok(exports)
    }
}

fn get_model_exports(key: ModelKey, model: &crate::model::Model) -> Vec<ModelExport> {
    RE_CAPTURE_EXPORT
        .captures_iter(model.value())
        .map(|cap| {
            let decl = cap.name("struct").unwrap();
            ModelExport {
                ident: cap.name("ident").unwrap().as_str().to_owned(),
                location: Location::new(key, decl.range()),
            }
        })
        .collect()
}
