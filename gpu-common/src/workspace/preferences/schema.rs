#![cfg(feature = "bindgen")]

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub trait Schema: Default {
    fn _schema_impl(builder: Builder) -> Builder;
    fn schema() -> HashMap<String, SchemaEntry> {
        Self::_schema_impl(Builder::default()).into_inner()
    }
    fn keys() -> Vec<String> {
        let mut keys: Vec<_> = Self::schema().into_keys().collect();
        keys.sort();
        keys
    }
}

impl Schema for bool {
    fn _schema_impl(builder: Builder) -> Builder {
        builder.push_config_class(ConfigClass::Bool(Bool {}))
    }
}

#[derive(Default, Debug)]
pub struct Builder {
    namespace: Vec<&'static str>,
    description: Option<&'static str>,
    config_class: Option<ConfigClass>,
    map: HashMap<String, SchemaEntry>,
}

impl Builder {
    pub fn push_name(mut self, name: &'static str) -> Self {
        self.namespace.push(name);
        self
    }

    pub fn push_description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn push_config_class(mut self, config_class: ConfigClass) -> Self {
        self.config_class = Some(config_class);
        self
    }

    pub fn pop(mut self) -> Self {
        if let (Some(description), Some(config_class)) =
            (self.description.take(), self.config_class.take())
        {
            let name = self.namespace.last().expect("Namespace length > 0");
            let path = self.namespace.join(".");
            let entry = SchemaEntry {
                name: name.to_string(),
                description: description.to_owned(),
                config_class,
            };
            if let Some(duplicate) = self.map.insert(path.clone(), entry) {
                log::error!("Duplicate config schema entry at path '{path}': {duplicate:?}");
            }
        }
        self.namespace.pop();
        self
    }

    fn into_inner(self) -> HashMap<String, SchemaEntry> {
        self.map
    }
}

#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaEntry {
    pub name: String,
    pub description: String,
    pub config_class: ConfigClass,
}

/// Describes the various classes of user interactables
///
/// View `front/src/components/preferences/*Controller.svelte` for more info
/// on how these input classes are implemented in the frontend.
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
#[serde(tag = "ty", content = "c")]
pub enum ConfigClass {
    /// <input type='number'/> with no fractional values
    #[serde(rename = "IntClass")]
    Int(Int),
    /// <input type='number'/> with fractional values
    #[serde(rename = "FloatClass")]
    Float(Float),
    /// <input/> plain string value
    #[serde(rename = "StrClass")]
    Str(Str),
    /// <select/> with list of values as options
    #[serde(rename = "EnumClass")]
    Enum(Enum),
    /// <input type='checkbox'/>
    #[serde(rename = "BoolClass")]
    Bool(Bool),
}

/// ### Int
/// Describes the constraints of an int interactable.
///
/// Used to generate html markup for changing fields
/// of config structs.
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct Int {
    /// The minimum value this input can be
    pub min: Option<i32>,
    /// The maximum value this input can be
    pub max: Option<i32>,
    /// The amount this input will step up and down
    pub step: u32,
    // The unit or other related information of this value
    pub postfix: Option<String>,
}

impl Int {
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

impl Default for Int {
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
pub struct Float {
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

impl Float {
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

impl Default for Float {
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
pub struct Str {
    pub regex: Option<String>,
}

impl Str {
    pub fn regex(mut self, val: String) -> Self {
        self.regex = Some(val);
        self
    }
}

#[derive(Default, Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct Bool {}

#[derive(Default, Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct Enum {
    pub variants: Vec<String>,
}
