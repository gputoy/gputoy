#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

pub struct Bundle;

const TYPE_DECL: &str = r#"
struct System {
    time: f32, 
    dt: f32,
    frame: u64
};
"#;

#[derive(Debug, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[strum(serialize_all = "kebab-case")]
pub enum Resources {
    Buffer,
}

#[cfg(feature = "naga")]
impl super::Bundle for Bundle {
    const TYPE_DECL: &'static str = TYPE_DECL;
}
