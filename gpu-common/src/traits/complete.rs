#![cfg(feature = "bindgen")]

pub trait Complete {
    fn static_completions() -> Vec<crate::completion::CompletionEntry>;
}
