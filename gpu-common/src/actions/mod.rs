use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Panel;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "ty", content = "c")]
pub enum Action {
    /// Toggles pane open and closed
    TogglePanel(Panel),
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
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShiftPaneArgs {
    pane: Panel,
    shift: i32,
}
