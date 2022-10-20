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
}
