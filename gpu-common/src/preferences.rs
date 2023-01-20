use crate::actions::Action;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct UserPrefs {
    pub keybinds: HashMap<String, FilteredAction>,
    pub editor: UserEditorPrefs,
    pub theme: HashMap<String, String>,
    pub general: UserGeneralPrefs,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct UserKeymap {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub map: HashMap<String, FilteredAction>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct FilteredAction {
    pub action: Action,
    pub condition: Option<String>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct UserEditorPrefs {
    pub font_family: Option<String>,
    pub font_size: Option<u32>,
    pub line_numbers: LineNumberPrefs,
    pub vim_mode: bool,
    pub minimap: bool,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum LineNumberPrefs {
    #[default]
    On,
    Interval,
    Relative,
    Off,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct UserGeneralPrefs {
    pub project_panel_size: f32,
    pub editor_panel_size: f32,
    pub resource_panel_size: f32,
    pub console_wrap: bool,
}
