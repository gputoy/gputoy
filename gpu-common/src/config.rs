#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::FilePath;

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Config {
    #[cfg_attr(feature = "serde", serde(default))]
    pub perf_level: Option<PerformanceLevel>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub limit_fps: u32,
    pub runner: Option<FilePath>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub log_level: LogLevel,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerformanceLevel {
    #[default]
    Default,
    PowerSaver,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}
