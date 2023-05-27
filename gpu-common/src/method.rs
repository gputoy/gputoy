pub use proc_macro::gpu_method as method;
use wasm_bindgen::JsValue;

pub enum MethodError {
    Method(crate::error::ClientError),
    #[cfg(feature = "wasm")]
    SerdeWasmBindgen(serde_wasm_bindgen::Error),
}

impl From<MethodError> for JsValue {
    fn from(val: MethodError) -> Self {
        match val {
            MethodError::Method(error) => serde_wasm_bindgen::to_value(&error).unwrap(),
            MethodError::SerdeWasmBindgen(error) => JsValue::from_str(&error.to_string()),
        }
    }
}
