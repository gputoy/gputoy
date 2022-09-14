#![cfg(target_arch = "wasm32")]

use thiserror::Error;
use wasm_bindgen::prelude::*;

use gpu_core::Context;

// #[wasm_bindgen]
// pub fn bind_surface(canvas: &web_sys::Element) -> Result<(), JsValue> {
//     Ok(())
// }

#[wasm_bindgen(start)]
pub fn __init() -> Result<(), JsValue> {
    console_log::init().map_err(|_| Error::LoggerInit)?;
    console_error_panic_hook::set_once();
    log::info!("__init");
    Ok(())
}

#[derive(Error, Debug)]
enum Error {
    #[error("Context init failed: {0}")]
    ContextInit(gpu_core::Error),
    #[error("Could not initialize logger")]
    LoggerInit,
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

#[wasm_bindgen]
pub async fn build() -> Result<Context, JsValue> {
    let context = Context::new().await.map_err(Error::ContextInit)?;
    Ok(context)
}

#[wasm_bindgen]
pub fn print_context(context: &Context) {
    log::trace!("Here is the context: {context:?}");
}
