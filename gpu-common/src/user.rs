use crate::actions::Action;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserConfig {
    pub keybinds: HashMap<String, Action>,
}
