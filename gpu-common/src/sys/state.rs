#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClientState {
    Uninitialized,
    Initialized,
    Prebuilt,
    Ready,
    Running,
    Paused,
}
