use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    #[serde(default)]
    perf_level: PerformanceLevel,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum PerformanceLevel {
    #[default]
    Default,
    PowerSaver,
}
