use lazy_static::lazy_static;
use regex::Regex;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap, fmt::Display, ops::Index};
use strum_macros::{AsRefStr, EnumString};
use validator::Validate;
use validator_derive::Validate;

use crate::RootError;

lazy_static! {
    pub static ref RE_FILE_PATH: Regex =
        Regex::new(r#"^\s*/([.]?[_]*[\w_-]+/)*([.]?[_]*[\w_-]*[.][a-z]+)\s*$"#).unwrap();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Validate)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(transparent)
)]
pub struct FilePath {
    #[validate(regex = "RE_FILE_PATH")]
    #[cfg_attr(feature = "schema", schemars(regex = "RE_FILE_PATH"))]
    path: String,
}

impl FilePath {
    pub fn try_from<A: AsRef<str>>(value: A) -> Result<Self, RootError> {
        let path = FilePath {
            path: value.as_ref().to_owned(),
        };
        if path.validate().is_ok() {
            Ok(path)
        } else {
            Err(crate::RootError::InvalidFilePath(path.into_inner()))
        }
    }
    pub fn into_inner(self) -> String {
        self.path
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        self.path.as_ref()
    }
}

impl Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.path)
    }
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Files {
    pub map: HashMap<FilePath, File>,
}

impl Files {
    pub fn get(&self, path: &FilePath) -> Option<&File> {
        self.map.get(path)
    }
}

/// Unsafe index for tests
impl Index<&str> for Files {
    type Output = File;

    fn index(&self, index: &str) -> &Self::Output {
        let file_id = FilePath::try_from(index).unwrap();
        self.get(&file_id).unwrap()
    }
}

/// Encapsulates all data needed to emulate a file in
/// gputoy virtual directory structure.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct File {
    /// Contents of file in plain text
    pub data: String,
    /// File path starting at / (project root)
    pub dir: String,
    /// Name of file
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(rename = "fileName")
    )]
    pub file_name: String,
    /// File extension
    pub extension: SupportedExtension,
    /// Fetch url. If exists, then contents will be fetched
    /// from remote URL on project load
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
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

#[derive(AsRefStr, EnumString, Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
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

#[cfg(test)]
mod tests {
    use super::FilePath;

    #[test]
    fn test_file_path() {
        let path = "/test/path/to/file.wgsl";
        assert!(FilePath::try_from(path).is_ok(), "{path}");
        let path = "test/path/to/file.wgsl";
        assert!(FilePath::try_from(path).is_err(), "{path}");

        let path = "/test.wgsl";
        assert!(FilePath::try_from(path).is_err(), "{path}");
        let path = "test.wgsl";
        assert!(FilePath::try_from(path).is_err(), "{path}");

        let path = "/._test.wgsl";
        assert!(FilePath::try_from(path).is_err(), "{path}");
        let path = "/..test.wgsl";
        assert!(FilePath::try_from(path).is_err(), "{path}");

        let path = "/.hidden/path-with-dash/test.wgsl";
        assert!(FilePath::try_from(path).is_err(), "{path}");
        let path = "/.hidden/path-with-dash/test";
        assert!(FilePath::try_from(path).is_err(), "{path}");
    }
}
