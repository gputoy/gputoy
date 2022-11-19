use gpu_compiler::PrebuildResult;
use thiserror::Error;
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen(start)]
pub fn __init() -> Result<(), JsValue> {
    console_log::init().map_err(|_| Error::LoggerInit)?;
    console_error_panic_hook::set_once();
    log::info!("__init");
    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Compiler error: {0}")]
    Compiler(gpu_compiler::Error),
    #[error("Could not serialize from JsonValue: {0}")]
    SerdeWasmBindgen(serde_wasm_bindgen::Error),
    #[error("Could not initialize logger")]
    LoggerInit,
    #[error("Failed to acquire file lock")]
    AcquireFileCache,
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

#[wasm_bindgen]
pub struct Compiler(gpu_compiler::Compiler);

#[wasm_bindgen]
impl Compiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(gpu_compiler::Compiler::new())
    }

    #[wasm_bindgen]
    pub fn prebuild(&mut self, files: JsValue) -> Result<JsValue, Error> {
        let files: gpu_common::Files =
            serde_wasm_bindgen::from_value(files).map_err(Error::SerdeWasmBindgen)?;
        let prebuild = self.0.prebuild(&files).map_err(Error::Compiler)?;
        serde_wasm_bindgen::to_value(&prebuild).map_err(Error::SerdeWasmBindgen)
    }
}
