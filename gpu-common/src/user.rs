use crate::actions::Action;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserConfig {
    pub keybinds: HashMap<String, FilteredAction>,
    pub editor: UserEditorConfig,
    pub theme: HashMap<String, String>,
    pub general: UserGeneralConfig,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserKeymap {
    #[serde(flatten)]
    pub map: HashMap<String, FilteredAction>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FilteredAction {
    pub action: Action,
    pub condition: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserEditorConfig {
    font_family: Option<String>,
    font_size: Option<u32>,
    line_numbers: LineNumberCOnfig,
    vim_mode: bool,
    minimap: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum LineNumberCOnfig {
    #[default]
    On,
    Interval,
    Relative,
    Off,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserGeneralConfig {
    project_panel_size: f32,
    editor_panel_size: f32,
    resource_panel_size: f32,
}
