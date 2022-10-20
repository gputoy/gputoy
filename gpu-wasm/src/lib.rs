// #![cfg(target_arch = "wasm32")]

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
    #[error("Context init failed: {0}")]
    ContextInit(gpu_client::context::Error),
    #[error("Context build failed: {0}")]
    ContextBuild(gpu_client::context::Error),
    #[error("Context render failed: {0}")]
    ContextRender(gpu_client::context::Error),
    #[error("Could not serialize from JsonValue: {0}")]
    SerdeWasmBindgen(serde_wasm_bindgen::Error),
    #[error("Could not initialize logger")]
    LoggerInit,
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Context(gpu_client::context::Context);

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<Context, Error> {
        let inner = gpu_client::context::Context::new()
            .await
            .map_err(Error::ContextInit)?;
        Ok(Context(inner))
    }

    #[wasm_bindgen(js_name = debug)]
    pub fn debug_to_console(&self) {
        log::info!("{:#?}", &self);
    }

    #[wasm_bindgen]
    pub fn resize(&self, width: i32, height: i32) {
        log::info!("Resizing to: {} {}", width, height);
    }

    #[wasm_bindgen]
    pub async fn build(&mut self, project: JsValue) -> Result<(), Error> {
        let project = serde_wasm_bindgen::from_value(project).map_err(Error::SerdeWasmBindgen)?;
        log::info!("Recieved project: {:?}", project);
        self.0.build(&project).await.map_err(Error::ContextBuild)
    }

    #[wasm_bindgen]
    pub async fn render(&self) -> Result<(), Error> {
        self.0.render().await.map_err(Error::ContextRender)
    }
}

#[wasm_bindgen]
pub enum FrameResult {
    Placeholder,
}

impl From<FrameResult> for JsValue {
    fn from(_: FrameResult) -> Self {
        JsValue::from_str("Placeholder")
    }
}
