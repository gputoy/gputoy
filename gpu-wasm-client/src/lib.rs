use thiserror::Error;
use wasm_bindgen::{prelude::*, JsValue};

/// Actual struct that can go across wasm boundary.
/// Notice the static lifetime on the inner context.
#[wasm_bindgen]
pub struct Context(gpu_client::Context);

/// First part of the hack, initialize loggers and 'statically' initialize inner context
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_log::init().map_err(|_| Error::LoggerInit)?;
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<Context, Error> {
        Ok(Context(
            gpu_client::Context::new()
                .await
                .map_err(Error::ContextBuild)?,
        ))
    }

    #[wasm_bindgen(js_name = debug)]
    pub fn debug_to_console(&self) {
        log::info!("{:#?}", &self.0);
    }

    #[wasm_bindgen]
    pub fn resize(&self, width: i32, height: i32) {
        log::info!("Resizing to: {} {}", width, height);
    }

    #[wasm_bindgen]
    pub async fn build(&mut self, runner: JsValue, prebuild_result: JsValue) -> Result<(), Error> {
        let prebuild_result =
            serde_wasm_bindgen::from_value(prebuild_result).map_err(Error::SerdeWasmBindgen)?;
        let runner = serde_wasm_bindgen::from_value(runner).map_err(Error::SerdeWasmBindgen)?;
        self.0
            .build(runner, prebuild_result)
            .await
            .map_err(Error::ContextBuild)
    }

    #[wasm_bindgen]
    pub async fn render(&mut self) -> Result<(), Error> {
        self.0.render().await.map_err(Error::ContextRender)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Context init failed: {0}")]
    ContextInit(gpu_client::Error),
    #[error("Context build failed: {0}")]
    ContextBuild(gpu_client::Error),
    #[error("Context introspection failed: {0}")]
    ContextIntrospect(gpu_client::Error),
    #[error("Context render failed: {0}")]
    ContextRender(gpu_client::Error),
    #[error("Could not serialize from JsonValue: {0}")]
    SerdeWasmBindgen(serde_wasm_bindgen::Error),
    #[error("Could not initialize logger")]
    LoggerInit,
    #[error("Context failed init in previous step.")]
    ContextFailed,
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}
