use js_sys::JsString;
use thiserror::Error;
use wasm_bindgen::{prelude::*, JsValue};

mod service;

#[wasm_bindgen(start)]
pub fn __init() -> Result<(), JsValue> {
    // gpu_log::init().map_err(|_| Error::LoggerInit)?;
    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Root error: {0}")]
    Root(#[from] gpu_common::RootError),
    #[error("Analyzer error: {0}")]
    Analyzer(#[from] gpu_analyzer::Error),
    #[error("Could not serialize from JsonValue: {0}")]
    SerdeWasmBindgen(#[from] serde_wasm_bindgen::Error),
    #[error("Could not initialize logger")]
    LoggerInit,
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

#[wasm_bindgen]
pub struct Analyzer(gpu_analyzer::Analyzer);

#[wasm_bindgen]
impl Analyzer {
    #[wasm_bindgen]
    pub fn new(init_files: JsValue) -> Result<Analyzer, Error> {
        let files = serde_wasm_bindgen::from_value(init_files)?;
        Ok(Self(gpu_analyzer::Analyzer::new(files)))
    }

    #[wasm_bindgen]
    #[cfg(feature = "dev")]
    pub fn expose(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.0.to_json()).unwrap()
    }

    #[wasm_bindgen]
    pub fn try_build(&mut self, runner_uri: &JsString) -> Result<JsValue, Error> {
        let runner_path = runner_uri.try_into()?;

        let diagnostics = self.0.try_build(&runner_path)?;
        Ok(serde_wasm_bindgen::to_value(&diagnostics)?)
    }

    // #[wasm_bindgen]
    // pub fn prebuild(&mut self, files: JsValue) -> Result<JsValue, Error> {
    //     let files: gpu_common::Files = serde_wasm_bindgen::from_value(files)?;
    //     let prebuild = self.0.prebuild(&files).map_err(Error::Analyzer)?;
    //     Ok(serde_wasm_bindgen::to_value(&prebuild)?)
    // }
}
