use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gputoy virtual directory. Each file in the map
/// has its path from root as key, including file name
/// and extension
///
/// example:
/// ```ts
/// map: {
///     "shaders/main.wgsl": {
///         "data": "...",
///         "dir": "shaders/",
///         "fileName": "main",
///         "extension": "wgsl",
///     }
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Files {
    pub map: HashMap<String, File>,
}

/// Encapsulates all data needed to emulate a file in
/// gputoy virtual directory structure.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct File {
    /// Contents of file in plain text
    data: String,
    /// File path starting at / (project root)
    dir: String,
    /// Name of file
    #[serde(rename = "fileName")]
    file_name: String,
    /// File extension
    extension: SupportedExtension,
    /// Fetch url. If exists, then contents will be fetched
    /// from remote URL on project load
    #[serde(skip_serializing_if = "Option::is_none")]
    fetch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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
    /// Returns true if file type can be used as a shader
    pub fn is_shader(&self) -> bool {
        matches!(self, Self::Wgsl | Self::Glsl)
    }

    /// Returns true if file type can be used as a gpu buffer
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
