use crate::{layout::Layout, Config, Files};

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Project {
    pub files: Files,
    pub layout: Option<Layout>,
    pub config: Option<Config>,
}
