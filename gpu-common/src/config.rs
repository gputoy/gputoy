use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    perf_level: Option<PerformanceLevel>,
    #[serde(default)]
    limit_fps: u32,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub enum PerformanceLevel {
    #[default]
    Default,
    PowerSaver,
}
