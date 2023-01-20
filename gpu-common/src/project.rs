use crate::{Config, Files, Layout};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Project {
    pub files: Files,
    pub layout: Option<Layout>,
    pub config: Option<Config>,
}
