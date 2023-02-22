#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Panel;

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all = "camelCase", tag = "ty", content = "c")
)]
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
    /// Opens document at specified id
    OpenDocument(String),
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
    /// Sets runner json file
    SetRunner(String),
    /// Move file
    Move(String, String),
    /// Copy file
    Copy(String, String)
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShiftPaneArgs {
    pub pane: Panel,
    pub shift: i32,
}
