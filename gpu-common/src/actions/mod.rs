use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Panel;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "ty", content = "c")]
pub enum Action {
    /// Toggles pane open and closed
    TogglePanel(Panel),
    /// Toggles debug panel
    ToggleDebugPanel,
    /// Toggle user preferences
    ToggleUserPreferences,
    /// Shifts pane by specified amount
    ShiftPanel(ShiftPaneArgs),
    /// Play/Pause the project
    PlayPause,
    /// Resets project to default state
    Reset,
    /// Rebuilds project
    Rebuild,
    /// Toggles Console
    ToggleConsole,
    /// Focuses pane
    Focus(Panel),
    /// Closes document in editor
    CloseDocument,
    /// Next document in editor
    NextDocument,
    /// Previous document in editor
    PreviousDocument,

    /// Creates new project
    CreateNewProject,
    /// Creates new file
    CreateNewFile,
    /// Save project to remote
    SaveProjectToRemote,
    /// Save current file
    SaveCurrentFile,
    /// Save all files
    SaveAllFiles,
    /// Fork project
    Fork,
    /// Publish project
    Publish,
    /// Close current file
    CloseFile,
    /// Close project
    CloseProject,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShiftPaneArgs {
    pane: Panel,
    shift: i32,
}
