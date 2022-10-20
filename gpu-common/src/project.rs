use crate::{Config, Files};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Project {
    pub files: Files,
    pub layout: Option<()>,
    pub config: Option<Config>,
}
