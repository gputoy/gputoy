use super::{ExportProcess, ImportProcess, Stage};
use crate::{
    analyzer::ProcessStore,
    diagnostics::{CatchDiagnostic, DiagnosticSink},
    model::ModelKey,
    types::linker::*,
};

pub struct LinkerProcess;

crate::store_for!(LinkerStore => LinkerProcess);

impl super::Process for LinkerProcess {
    type Result = crate::types::linker::LinkResult;
    type Store = LinkerStore;

    const PROCESS_NAME: &'static str = "linker";
    const STAGE_VARIANT: Stage = Stage::Linker;

    fn process(
        analyzer: &mut crate::Analyzer,
        key: ModelKey,
        sink: &mut DiagnosticSink,
    ) -> crate::Result<Self::Result> {
        let models = &mut analyzer.model_cache;

        let export_store = analyzer.store.get::<ExportProcess>();
        let import_store = analyzer.store.get::<ImportProcess>();

        let model = models.get_model(key)?;
        let imports = import_store.get(key)?;

        let exports = {
            let mut exports = Vec::with_capacity(imports.len());
            for import in imports {
                if let Some(export_decl) = export_store
                    .resolve_import(import)
                    .and_then(|export| {
                        models
                            .get_location(&export.location)
                            .map_err(crate::Error::ModelError)
                    })
                    .catch(sink)
                {
                    exports.push(export_decl);
                }
            }
            exports
        };

        sink.throw_on_fail(key)?;

        let char_len = exports.iter().map(|export| export.len()).sum::<usize>()
            + exports.len()
            + model.value().len()
            + EXTRA_CHAR_PADDING;

        let mut processed_src = String::with_capacity(char_len);
        let mut inserts = Vec::with_capacity(exports.len());

        // insert export declarations into new src
        for export in exports {
            let idx = processed_src.len();
            processed_src.push_str(export);
            processed_src.push(NEWLINE);
            let len = processed_src.len() - idx;
            inserts.push(LinkerInsertion::insertion(idx, len));
        }

        let user_src = model.value();
        let user_src = RE_REPLACE_IMPORT.replace_all(user_src, "");
        let user_src = RE_REPLACE_EXPORT.replace_all(&user_src, "");
        processed_src.push_str(&user_src);

        let result = LinkResult {
            processed_src,
            inserts,
        };

        Ok(result)
    }
}
