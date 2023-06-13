mod region;
mod tab;

pub use region::*;
pub use tab::*;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Layout {
    /// List of files which is open in editor.
    /// Order of identifiers in vec is the order it is listed in the editor.
    pub tabs: Vec<String>,
    /// Currently opened tab within workspace
    pub tab_index: Option<usize>,
    /// Pane toggle data
    pub pane_toggled: PaneToggled,
    /// Pane size data
    pub pane_size: PaneSize,
    /// State of file tree
    pub file_tree_state: HashMap<String, DirNodeState>,
    /// State of project panel accordians
    pub accordian_open: HashMap<String, bool>,
}

/// Pane size information.
///
/// Persistent layout data needed to give 'preferred size' to panes.
#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PaneSize {
    /// How many pixels wide the project pane should be.
    ///
    /// Will change its percentage share of the window if window width changes.
    project_pane_px: u32,
    /// What percentage of total window width the editor pane takes up.
    ///
    /// It is assumed the viewport/resource pane will take up the remaining
    /// space left behind by the project and editor pane.
    editor_pane_percentage: f32,
    /// What percentage of total window height the control pane takes up.
    control_pane_percentage: f32,
}

/// Pane layout information.
#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct PaneToggled {
    /// Whether the project pane is open
    project_pane: bool,
    /// Whether the editor pane is open
    editor_pane: bool,
    /// Whether the control pane is open
    control_pane: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DirNodeState {
    pub open: bool,
    pub is_renaming: bool,
}
