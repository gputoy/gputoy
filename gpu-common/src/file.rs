#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap, ops::Index};
use strum_macros::{AsRefStr, EnumString};

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
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Files {
    pub map: HashMap<String, File>,
}

impl<'i> Index<&'i str> for Files {
    type Output = File;

    fn index<'a>(&'a self, name: &'i str) -> &'a File {
        self.map
            .get(name)
            .unwrap_or_else(|| panic!("no file with id '{}'", name))
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
