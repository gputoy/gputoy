use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub fn attach_canvas(canvas: JsValue) -> Result<(), JsValue> {
    let _canvas = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|err| err.js_typeof())?;
    Ok(())
}
