#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::FilePath;

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct Config {
    #[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(default))]
    pub perf_level: Option<PerformanceLevel>,
    #[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(default))]
    pub limit_fps: u32,
    pub runner: Option<FilePath>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum PerformanceLevel {
    #[default]
    Default,
    PowerSaver,
}
