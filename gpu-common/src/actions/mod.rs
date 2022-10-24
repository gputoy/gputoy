use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    // INTERNAL
    TogglePane(Pane),
    ShiftPane(Pane, i32),
    /// Play/Pause the project
    PlayPause,
    /// Resets project to default state
    Reset,
    /// Rebuilds project
    Rebuild,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Pane {
    EditorPane,
    ProjectPane,
    ResourcePane,
}
