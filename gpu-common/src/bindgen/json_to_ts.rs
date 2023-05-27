use std::{path::PathBuf, str::FromStr};

use super::{Config, Error, Result};

lazy_static::lazy_static! {
    static ref JSON_TO_TS_SCRIPT: String = {
        let root_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let relative_path = std::env::var("JSON_TO_TS_SCRIPT")
            .unwrap_or(String::from("front/generate_common_types.js"));
        let mut path = PathBuf::from_str(&root_path).unwrap().parent().unwrap().to_path_buf();
        path.push(relative_path);
        path.to_str().unwrap().into()
    };
}

pub fn ts_from_json_schema(config: &Config) -> Result<String> {
    let result = std::process::Command::new("node")
        .arg(&config.json_to_ts_path)
        .arg(config.root_schema_path())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(Error::SpawnTask)?
        .wait_with_output()
        .map_err(Error::WaitTask)?;

    if result.status.success() {
        Ok(String::from_utf8_lossy(&result.stdout).to_string())
    } else {
        Err(Error::GenTsTypes(
            String::from_utf8_lossy(&result.stderr).to_string(),
        ))
    }
}
