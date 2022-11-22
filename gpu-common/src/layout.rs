#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct Layout {
    /// Is the left side status panel open
    pub is_status_open: bool,
    /// List of file identifiers which is open in workspace.
    /// Order of identifiers in vec is the order it is listed in the editor.
    pub workspace: Vec<String>,
    /// Currently opened file index within workspace
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(rename = "fileIndex")
    )]
    pub workspace_file_index: Option<usize>,
    /// Panel settings for projectPanel
    pub project_panel: PanelState,
    /// Panel settings for editorPanel
    pub editor_panel: PanelState,
    /// Panel settings for resourcePanel
    pub resource_panel: PanelState,
    /// State of file tree
    pub file_tree_state: HashMap<String, DirNodeState>,
    /// State of project panel accordians
    pub accordian_open: HashMap<String, bool>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub enum Panel {
    EditorPanel,
    ProjectPanel,
    ResourcePanel,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct PanelState {
    pub show: bool,
    pub size: f32,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct DirNodeState {
    pub open: bool,
    pub is_renaming: bool,
}
