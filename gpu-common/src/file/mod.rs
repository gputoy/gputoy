use crate::FastHashMap;
use regex::Regex;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use strum_macros::{AsRefStr, Display, EnumString};

mod file_path;
mod path;

pub use file_path::FilePath;
pub use path::Path;

lazy_static::lazy_static! {
    pub static ref RE_PATH: Regex =
        Regex::new(r#"^\s*/([.]?[_]*[\w_-]+/)*([.]?[_]*[\w_-]*([.][a-z]+)?)\s*$"#).unwrap();
    pub static ref RE_FILE_PATH: Regex =
        Regex::new(r#"^\s*/([.]?[_]*[\w_-]+/)*([.]?[_]*[\w_-]*[.][a-z]+)\s*$"#).unwrap();
}

/// Gputoy virtual directory. Each file in the map
/// has its path from root as key, including file name
/// and extension
///
/// example:
/// ```ts
/// map: {
///     "/shaders/main.wgsl": {
///         "data": "...",
///         "dir": "shaders/",
///         "fileName": "main",
///         "extension": "wgsl",
///     }
/// }
/// ```
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Files {
    pub map: FastHashMap<FilePath, File>,
}

impl Files {
    pub fn get(&self, path: &FilePath) -> Option<&File> {
        self.map.get(path)
    }
}

// impl Index<&str> for Files {
//     type Output = File;

//     fn index(&self, index: &str) -> &Self::Output {
//         let file_id = FilePath::try_from(index).unwrap();
//         self.get(&file_id).unwrap()
//     }
// }

/// Encapsulates all data needed to emulate a file in
/// gputoy virtual directory structure.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct File {
    /// Contents of file in plain text
    pub data: String,
    /// File path starting at / (project root)
    pub dir: String,
    /// Name of file
    #[cfg_attr(feature = "serde", serde(rename = "fileName"))]
    pub file_name: String,
    /// File extension
    pub extension: SupportedExtension,
    /// Fetch url. If exists, then contents will be fetched
    /// from remote URL on project load
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fetch: Option<String>,
}

impl<'a> From<&'a File> for Cow<'a, str> {
    #[inline]
    fn from(file: &'a File) -> Self {
        Cow::Borrowed(&file.data)
    }
}

impl File {
    #[inline]
    pub fn canonical_name(&self) -> String {
        format!("{}.{}", self.file_name, self.extension.as_ref())
    }
}

#[derive(Display, AsRefStr, EnumString, Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[strum(serialize_all = "lowercase")]
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
