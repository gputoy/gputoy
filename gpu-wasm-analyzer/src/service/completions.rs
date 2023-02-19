use crate::{Analyzer, Error};
use gpu_analyzer::model::ModelChangeText;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Analyzer {
    #[wasm_bindgen(js_name = "getCompletionsAtPosition")]
    pub fn get_completions_at_position(
        &mut self,
        uri: &JsString,
        prev_word: String,
        curr_word: String,
        delim: String,
        position: gpu_analyzer::service::Position,
    ) -> Result<Box<[JsValue]>, Error> {
        let path = uri.try_into()?;

        let completions = self
            .0
            .get_completions_at_position(path, &prev_word, &curr_word, &delim, position)?;

        completions
            .into_iter()
            .map(|comp| serde_wasm_bindgen::to_value(&comp).map_err(From::from))
            .collect()
    }

    #[wasm_bindgen(js_name = "handleFileDeltaChar")]
    pub fn handle_file_delta_char(
        &mut self,
        uri: &JsString,
        ch: char,
        offset: u32,
        version_id: u32,
    ) -> Result<JsValue, Error> {
        let uri = uri.try_into()?;
        Ok(serde_wasm_bindgen::to_value(
            &self
                .0
                .handle_file_delta_char(&uri, ch, offset, version_id)?,
        )?)
    }

    #[wasm_bindgen(js_name = "handleFileDelta")]
    pub fn handle_file_delta(
        &mut self,
        uri: &JsString,
        changes: JsValue,
        version_id: u32,
        is_flush: bool,
    ) -> Result<JsValue, Error> {
        let uri = uri.try_into()?;
        let changes: Vec<ModelChangeText> = serde_wasm_bindgen::from_value(changes)?;
        Ok(serde_wasm_bindgen::to_value(
            &self
                .0
                .handle_file_delta(&uri, changes, version_id, is_flush)?,
        )?)
    }

    #[wasm_bindgen(js_name = "applyFileChanges")]
    pub fn apply_file_changes(&mut self, uri: Option<JsString>) -> Result<(), Error> {
        let uri = uri.as_ref().map(TryInto::try_into).transpose()?;
        Ok(self.0.commit_changes(uri.as_ref())?)
    }
}
