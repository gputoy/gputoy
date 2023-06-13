#[cfg(feature = "schema")]
use schemars::{
    schema::{Schema, SchemaObject},
    JsonSchema,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::{completion::CompletionKey, RootError, RE_FILE_PATH, RE_PATH};

/// Path
///
/// Represents a path in the gputoy file system.
/// Paths must start with '/'.
/// Every [`super::FilePath`] is also a Path, as paths can represent both files and directories.
/// ```
/// use gpu_common::Path;
///
/// let path = Path::try_from("/some/path/to/file.txt");
/// assert!(path.is_ok());
///
/// let path = Path::try_from("/some/path/no/file");
/// assert!(path.is_ok());
/// ```
#[derive(Debug, Hash, PartialEq, Eq, Clone, Validate)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Path {
    #[validate(regex = "RE_PATH")]
    path: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    is_file: bool,
}

impl Path {
    /// Try to parse file path from string
    pub fn try_from<A: AsRef<str>>(value: A) -> Result<Self, RootError> {
        let path = Path {
            path: value.as_ref().to_owned(),
            is_file: RE_FILE_PATH.is_match(value.as_ref()),
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
    pub fn is_file(&self) -> bool {
        self.is_file
    }
}

impl crate::describe::Describe<'_> for Path {
    fn describe(manifest: &mut crate::describe::Manifest<'_>) {
        manifest
            .with_name("path")
            .with_description("A path to a directory or file")
            .with_completion(CompletionKey::Path)
            .finish_arg()
    }
}

impl crate::parse::Parse<'_> for Path {
    fn parse(args: &mut crate::parse::ParseArgs) -> Result<Self, Self::Error> {
        args.next_arg().and_then(Self::try_from)
    }
}

#[cfg(feature = "schema")]
impl JsonSchema for Path {
    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut schema = SchemaObject::default();
        schema.instance_type = Some(schemars::schema::SingleOrVec::Single(Box::new(
            schemars::schema::InstanceType::String,
        )));
        schema.string().pattern = Some(RE_PATH.to_string());
        Schema::Object(schema)
    }

    fn schema_name() -> String {
        "Path".into()
    }
}

#[cfg(test)]
mod test {
    use super::Path;

    #[test]
    fn test_path() {
        let path = "/test/path/to/file.wgsl";
        assert!(Path::try_from(path).is_ok(), "{path}");
        let path = "/test/path";
        assert!(Path::try_from(path).is_ok(), "{path}");

        let path = "test.wgsl";
        assert!(Path::try_from(path).is_err(), "{path}");
        let path = "test/path";
        assert!(Path::try_from(path).is_err(), "{path}");

        let path = "/._test.wgsl";
        assert!(Path::try_from(path).is_ok(), "{path}");
        let path = "/..test.wgsl";
        assert!(Path::try_from(path).is_err(), "{path}");

        let path = "/.hidden/path-with-dash/test.wgsl";
        assert!(Path::try_from(path).is_ok(), "{path}");
        let path = "/.hidden/path-with-dash/test";
        assert!(Path::try_from(path).is_ok(), "{path}");
    }
}
