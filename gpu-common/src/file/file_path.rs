#[cfg(feature = "schema")]
use schemars::{
    schema::{Schema, SchemaObject},
    JsonSchema,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use super::RE_FILE_PATH;
use crate::{completion::CompletionKey, RootError};

/// FilePath
///
/// Represents a path to a file in the gputoy file system.
/// File paths must start with '/' and end with an extension i.e. '.wgsl'.
/// ```
/// use gpu_common::FilePath;
///
/// let file_path = FilePath::try_from("/some/path/to/file.txt");
/// assert!(file_path.is_ok());
///
/// let invalid_file_path = FilePath::try_from("/some/path/no/file");
/// assert!(invalid_file_path.is_err());
/// ```
#[derive(Debug, Hash, PartialEq, Eq, Clone, Validate)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct FilePath {
    #[validate(regex = "RE_FILE_PATH")]
    path: String,
}

impl FilePath {
    /// Escape hatch from file path validation.
    ///
    /// Indicates a missing path where one is expected. Should only be used in
    /// diagnostics.
    pub fn unknown() -> Self {
        Self {
            path: String::from("unknown"),
        }
    }

    /// Try to parse file path from string
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

impl TryFrom<&str> for FilePath {
    type Error = RootError;
    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Self::try_from(path)
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        self.path.as_ref()
    }
}

impl core::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.path)
    }
}

impl crate::describe::Describe<'_> for FilePath {
    fn describe(manifest: &mut crate::describe::Manifest) {
        manifest
            .with_name("file-path")
            .with_description("A path to a file")
            .with_completion(CompletionKey::FilePath)
            .finish_arg()
    }
}

impl crate::parse::Parse<'_> for FilePath {
    fn parse(args: &mut crate::parse::ParseArgs) -> Result<Self, Self::Error> {
        args.next_arg().and_then(Self::try_from)
    }
}

#[cfg(feature = "schema")]
impl JsonSchema for FilePath {
    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut schema = SchemaObject::default();
        schema.instance_type = Some(schemars::schema::SingleOrVec::Single(Box::new(
            schemars::schema::InstanceType::String,
        )));
        schema.string().pattern = Some(RE_FILE_PATH.to_string());
        Schema::Object(schema)
    }

    fn schema_name() -> String {
        "FilePath".to_owned()
    }
}

#[cfg(feature = "wasm")]
impl TryFrom<&js_sys::JsString> for FilePath {
    type Error = RootError;

    fn try_from(value: &js_sys::JsString) -> Result<Self, Self::Error> {
        FilePath::try_from(
            value
                .as_string()
                .ok_or(Self::Error::InvalidStringFormat(value.clone()))?,
        )
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
        assert!(FilePath::try_from(path).is_ok(), "{path}");
        let path = "test.wgsl";
        assert!(FilePath::try_from(path).is_err(), "{path}");

        let path = "/._test.wgsl";
        assert!(FilePath::try_from(path).is_ok(), "{path}");
        let path = "/..test.wgsl";
        assert!(FilePath::try_from(path).is_err(), "{path}");

        let path = "/.hidden/path-with-dash/test.wgsl";
        assert!(FilePath::try_from(path).is_ok(), "{path}");
        let path = "/.hidden/path-with-dash/test";
        assert!(FilePath::try_from(path).is_err(), "{path}");
    }
}
