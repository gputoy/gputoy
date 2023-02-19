use gpu_common::FilePath;

use crate::model::{Model, ModelChange, ModelVersion};

use super::{ExportProcess, ImportProcess, ParserProcess};

#[derive(serde::Serialize)]
pub struct JsonIntermediary {
    #[serde(flatten)]
    inner: std::collections::HashMap<FilePath, JsonIntermediaryModel>,
}

#[derive(serde::Serialize)]
struct JsonIntermediaryModel {
    model: Option<Model>,
    module: Option<naga::Module>,
    changes: Vec<ModelChange>,
    version: ModelVersion,
    exports: Vec<JsonIntermediaryExport>,
    imports: Vec<JsonIntermediaryImport>,
}

#[derive(serde::Serialize)]
struct JsonIntermediaryImport {
    path: Option<String>,
    idx: usize,
}

#[derive(serde::Serialize)]
struct JsonIntermediaryExport {}

impl crate::Analyzer {
    pub fn to_json(&self) -> JsonIntermediary {
        JsonIntermediary {
            inner: self
                .model_cache
                .iter_idents()
                .map(|(path, &key)| {
                    let imports = self
                        .store
                        .get_result::<ImportProcess>(key)
                        .cloned()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|im| JsonIntermediaryImport {
                            path: self.model_cache.path(im.from).map(ToString::to_string).ok(),
                            idx: im.idx,
                        })
                        .collect();
                    let exports = self
                        .store
                        .get_result::<ExportProcess>(key)
                        .cloned()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|_ex| JsonIntermediaryExport {})
                        .collect();
                    (
                        path.clone(),
                        JsonIntermediaryModel {
                            model: self.model_cache.get_model(key).cloned().ok(),
                            module: self.store.get_result::<ParserProcess>(key).cloned().ok(),
                            imports,
                            exports,
                            version: self
                                .model_cache
                                .versions
                                .get(key)
                                .copied()
                                .unwrap_or_default(),
                            changes: self
                                .model_cache
                                .changes
                                .get(key)
                                .cloned()
                                .unwrap_or_default(),
                        },
                    )
                })
                .collect(),
        }
    }
}
