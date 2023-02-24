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
    /// List of file identifiers which is open in workspace.
    /// Order of identifiers in vec is the order it is listed in the editor.
    pub workspace: Vec<String>,
    /// Currently opened file index within workspace
    #[cfg_attr(feature = "serde", serde(rename = "fileIndex"))]
    pub workspace_file_index: Option<usize>,
    /// Pane toggle data
    pub pane_toggled: PaneToggled,
    /// Pane size data
    pub pane_size: PaneSize,
    /// State of file tree
    pub file_tree_state: HashMap<String, DirNodeState>,
    /// State of project panel accordians
    pub accordian_open: HashMap<String, bool>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum Pane {
    ProjectPane = 0,
    EditorPane = 1,
    ResourcePane = 2,
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
    /// What percentage of total window height the resource pane takes up.
    resource_pane_percentage: f32,
}

/// Pane layout information.
#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PaneToggled {
    /// Whether the project pane is open
    project_pane: bool,
    /// Whether the editor pane is open
    editor_pane: bool,
    /// Whether the resource pane is open
    resource_pane: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DirNodeState {
    pub open: bool,
    pub is_renaming: bool,
}
