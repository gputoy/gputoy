use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    /// Is the left side status panel open
    is_status_open: bool,
    /// List of file identifiers which is open in workspace.
    /// Order of identifiers in vec is the order it is listed in the editor.
    workspace: Vec<String>,
    /// Currently opened file index within workspace
    #[serde(rename = "fileIndex")]
    workspace_file_index: Option<usize>,
    /// Panel settings for projectPanel
    project_panel: PanelState,
    /// Panel settings for editorPanel
    editor_panel: PanelState,
    /// Panel settings for resourcePanel
    resource_panel: PanelState,
    /// State of file tree
    file_tree_state: HashMap<String, DirNodeState>,
    /// State of project panel accordians
    accordian_open: HashMap<String, bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Panel {
    EditorPanel,
    ProjectPanel,
    ResourcePanel,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PanelState {
    show: bool,
    size: f32,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DirNodeState {
    open: bool,
    is_renaming: bool,
}
