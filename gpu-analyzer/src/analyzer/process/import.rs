use crate::{
    diagnostics::DiagnosticSink,
    model::{Location, ModelKey},
    types::resolver::{ModelImport, RE_CAPTURE_IMPORT},
};

use super::{ExportProcess, ExportStore, Stage};

pub struct ImportProcess;

crate::store_for!(ImportStore => ImportProcess);

impl super::Process for ImportProcess {
    type Result = Vec<ModelImport>;
    type Store = ImportStore;

    const PROCESS_NAME: &'static str = "imports";
    const STAGE_VARIANT: Stage = Stage::ImportResolver;

    fn process(
        analyzer: &mut crate::Analyzer,
        key: crate::model::ModelKey,
        _error_sink: &mut DiagnosticSink,
    ) -> crate::Result<Self::Result> {
        let model = analyzer.model_cache.get_model(key)?;

        let exports = analyzer.store.get::<ExportProcess>();
        let imports = get_model_imports(key, exports, model)?;

        Ok(imports)
    }
}
fn get_model_imports(
    key: ModelKey,
    exports: &ExportStore,
    model: &crate::model::Model,
) -> crate::Result<Vec<ModelImport>> {
    RE_CAPTURE_IMPORT
        .captures_iter(model.value())
        .map(|cap| {
            let ident = cap.name("ident").unwrap();
            let (from_key, idx) =
                exports
                    .find_exported_ident(ident.as_str())
                    .ok_or(crate::Error::ImportNotFound(Location::new(
                        key,
                        ident.range(),
                    )))?;
            Ok(ModelImport {
                from: from_key,
                idx,
                location: Location::new(key, ident.range()),
            })
        })
        .collect()
}
