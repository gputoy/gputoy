#![feature(vec_push_within_capacity)]
#![feature(associated_type_defaults)]

mod analyzer;
pub mod api;
pub mod bundle;
mod config;
mod error;
mod event;
mod file;
mod project;
pub mod runtime;
pub mod sys;
mod traits;
mod workspace;

pub use actions::*;
pub use analyzer::*;
pub use config::*;
pub use error::*;
pub use file::*;
pub use layout::*;
pub use preferences::*;
pub use project::*;
pub use sys::*;
pub use traits::*;
pub use workspace::*;

#[cfg(feature = "method")]
pub mod method;
#[cfg(feature = "method")]
pub use method::method;
#[cfg(feature = "bindgen")]
pub mod bindgen;
#[cfg(feature = "schema")]
pub mod schema;

mod macros;

#[cfg(feature = "bindgen")]
pub use proc_macro::gen_ts;

pub use proc_macro::extract_doc_comment;
pub use proc_macro::to_case;

#[cfg(feature = "bindgen")]
pub fn bindgen(config: crate::bindgen::Config) -> crate::bindgen::Result<()> {
    let builder = crate::bindgen::Builder::new(config.clone());

    crate::preferences::bindgen(builder.dir("prefs")?)?;
    crate::workspace::bindgen(builder.dir("workspace")?)?;

    builder
        .file(config.root_schema_name)
        .containing_schema::<crate::schema::RootSchema>()?;

    builder.finish()
}
