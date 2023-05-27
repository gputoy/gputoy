#![cfg(feature = "bindgen")]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait ConfigValue: Default {
    fn metadata(prefix: &str) -> ConfigValueSchema;
    fn keys(prefix: &str) -> Vec<String>;
}

#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct ConfigValueSchema {
    pub name: String,
    pub description: String,
    pub path: String,
    pub class: ConfigValueClass,
}

/// Describes the various classes of user interactables
///
/// View `front/src/components/preferences/*Controller.svelte` for more info
/// on how these input classes are implemented in the frontend.
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
#[serde(tag = "ty", content = "c")]
pub enum ConfigValueClass {
    /// <input type='number'/> with no fractional values
    IntClass(IntClass),
    /// <input type='number'/> with fractional values
    FloatClass(FloatClass),
    /// <input/> plain string value
    StrClass(StrClass),
    /// <select/> with list of values as options
    EnumClass(EnumClass),
    /// <input type='checkbox'/>
    BoolClass(BoolClass),
    /// category of interactables mapped by the children's identifiers
    CategoryClass(CategoryClass),
    /// category with an inserted `enabled` field which can be used to
    /// toggle the entire feature on and off
    ToggledCategoryClass(CategoryClass),
    /// command input
    CmdClass(CmdClass),
}

/// ### Int
/// Describes the constraints of an int interactable.
///
/// Used to generate html markup for changing fields
/// of config structs.
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct IntClass {
    /// The minimum value this input can be
    pub min: Option<i32>,
    /// The maximum value this input can be
    pub max: Option<i32>,
    /// The amount this input will step up and down
    pub step: u32,
    // The unit or other related information of this value
    pub postfix: Option<String>,
}

impl IntClass {
    pub fn min(mut self, val: i32) -> Self {
        self.min = Some(val);
        self
    }
    pub fn max(mut self, val: i32) -> Self {
        self.max = Some(val);
        self
    }
    pub fn step(mut self, val: u32) -> Self {
        self.step = val;
        self
    }
    pub fn postfix(mut self, val: impl ToString) -> Self {
        self.postfix = Some(val.to_string());
        self
    }
}

impl Default for IntClass {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
            step: 1,
            postfix: None,
        }
    }
}

/// ### FloatInput
/// Describes the constraints of a float input.
///
/// Used to generate html markup for changing fields
/// of config structs.
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct FloatClass {
    /// The minimum value this input can be
    pub min: Option<f32>,
    /// The maximum value this input can be
    pub max: Option<f32>,
    /// The amount this input will step up and down
    pub step: Option<f32>,
    // The unit or other related information of this value
    pub postfix: Option<String>,
    // How many decimal places to display
    pub scale: i8,
}

impl FloatClass {
    pub fn min(mut self, val: f32) -> Self {
        self.min = Some(val);
        self
    }
    pub fn max(mut self, val: f32) -> Self {
        self.max = Some(val);
        self
    }
    pub fn step(mut self, val: f32) -> Self {
        self.step = Some(val);
        self
    }
    pub fn postfix(mut self, val: impl ToString) -> Self {
        self.postfix = Some(val.to_string());
        self
    }
    pub fn scale(mut self, val: i8) -> Self {
        self.scale = val;
        self
    }
}

impl Default for FloatClass {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
            step: None,
            postfix: None,
            scale: 2,
        }
    }
}

#[derive(Default, Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct StrClass {
    pub regex: Option<String>,
}

#[cfg(feature = "bindgen")]
impl StrClass {
    pub fn regex(mut self, val: String) -> Self {
        self.regex = Some(val);
        self
    }
}

#[derive(Default, Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct BoolClass {}

#[derive(Default, Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct EnumClass {
    pub variants: Vec<String>,
}

#[derive(Default, Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct CategoryClass {
    #[serde(flatten)]
    pub inner: HashMap<String, ConfigValueSchema>,
}

#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct CmdClass {
    completions: bool,
}
