use crate::{Config, Files, Layout};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Project {
    pub files: Files,
    pub layout: Option<Layout>,
    pub config: Option<Config>,
}
