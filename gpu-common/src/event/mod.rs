#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod file;
mod misc;
mod resource;

pub use file::*;
pub use misc::*;
pub use resource::*;

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all = "camelCase", tag = "ty", content = "c")
)]
pub enum Event {
    // Resize(misc::ResizeEvent),
    File(file::FileEvent),
    // Control(misc::ControlEvent),
    // Resource(resource::ResourceEvent),
}
