mod builder;
mod json_to_ts;

use std::path::PathBuf;

pub use builder::{Builder, Writer};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub root_path: PathBuf,
    pub json_to_ts_path: PathBuf,
    pub root_schema_name: String,
    pub ts_index_name: String,
}

impl Config {
    pub fn root_schema_path(&self) -> PathBuf {
        let mut path = self.root_path.clone();
        path.push(&self.root_schema_name);
        path
    }
    pub fn ts_index_path(&self) -> PathBuf {
        let mut path = self.root_path.clone();
        path.push(&self.ts_index_name);
        path
    }
}

#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// Environment variable missing: {0}
    MissingEnv(&'static str),
    /// Failed to create dir {0}: {1}
    CreateDirectory(std::path::PathBuf, std::io::Error),
    /// Failed to create file {0}: {1}
    CreateFile(std::path::PathBuf, std::io::Error),
    /// Typescript type generation script failed: {0}
    GenTsTypes(String),
    /// Failed to serialize default value: {0}
    SerializeDefault(serde_json::Error),
    /// Failed to serialize value: {0}
    SerializeValue(serde_json::Error),
    /// Failed to serialize schema: {0}
    SerializeSchema(serde_json::Error),
    /// Failed to spawn task: {0}
    SpawnTask(std::io::Error),
    /// Failed to wait task: {0}
    WaitTask(std::io::Error),
}
