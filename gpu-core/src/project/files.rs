use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    #[serde(flatten)]
    pub map: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    data: String,
    dir: FilePath,
    #[serde(rename = "fileName")]
    file_name: String,
    extension: SupportedExtension,
    #[serde(skip_serializing_if = "Option::is_none")]
    fetch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FilePath(String);

impl AsRef<String> for FilePath {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportedExtension {
    Wgsl,
    Glsl,
    Txt,
    Md,
    Json,
    Csv,
    Png,
    Jpeg,
    Mp3,
}

impl SupportedExtension {
    pub fn is_shader(&self) -> bool {
        matches!(self, Self::Wgsl | Self::Glsl)
    }

    pub fn is_buffer(&self) -> bool {
        matches!(self, Self::Csv | Self::Png | Self::Jpeg | Self::Mp3)
    }
}

#[cfg(test)]
mod test {
    use super::SupportedExtension;

    #[test]
    fn test_extensions() {
        assert_eq!(false, SupportedExtension::Csv.is_shader());
    }
}
