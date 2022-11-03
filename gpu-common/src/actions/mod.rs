use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    /// Toggles pane open and closed
    TogglePane(Pane),
    /// Shifts pane by specified amount
    ShiftPane(Pane, i32),
    /// Play/Pause the project
    PlayPause,
    /// Resets project to default state
    Reset,
    /// Rebuilds project
    Rebuild,
    /// Toggles Console
    ToggleConsole,
    /// Focuses pane
    Focus(Pane),
    /// Closes document in editor
    CloseDocument,
    /// Next document in editor
    NextDocument,
    /// Previous document in editor
    PreviosDocument,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Pane {
    EditorPane,
    ProjectPane,
    ResourcePane,
}
