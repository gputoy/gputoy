use regex::Regex;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::completion::CompletionKey;
use crate::describe::{Describe, Manifest};
use crate::parse::{Parse, ParseArgs};
use crate::{FilePath, Path, RootError};

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct CopyMove {
    pub(crate) src: Path,
    pub(crate) dest: Path,
    pub(crate) is_dir: bool,
}

impl Describe<'_> for CopyMove {
    const ARG_COUNT: usize = 2;
    fn describe(manifest: &mut Manifest) {
        let is_file = if let Some(path) = manifest.peek_arg() {
            Path::try_from(path)
                .ok()
                .map(|s| s.is_file())
                .unwrap_or_default()
        } else {
            false
        };
        if is_file {
            FilePath::describe(manifest.with_name_override("src"));
            FilePath::describe(manifest.with_name_override("dest"));
        } else {
            Path::describe(manifest.with_name_override("src"));
            Path::describe(manifest.with_name_override("dest"));
        };
    }
}

impl Parse<'_> for CopyMove {
    fn parse(args: &mut ParseArgs<'_>) -> Result<Self, Self::Error> {
        let src = Path::parse(args)?;
        let dest = Path::parse(args)?;

        if src.is_file() != dest.is_file() {
            return Err(RootError::InvalidCopyMove);
        }

        Ok(Self {
            is_dir: !src.is_file(),
            src,
            dest,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Delete {
    path: Path,
    is_dir: bool,
}

impl Describe<'_> for Delete {
    fn describe(manifest: &mut Manifest) {
        Path::describe(manifest)
    }
}

impl Parse<'_> for Delete {
    fn parse(args: &mut ParseArgs<'_>) -> Result<Self, Self::Error> {
        let path = Path::parse(args)?;
        Ok(Self {
            is_dir: !path.is_file(),
            path,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BindKey {
    pub key: Key,
    pub command: Box<super::Action>,
}

impl Describe<'_> for BindKey {
    fn describe(manifest: &mut Manifest) {
        Key::describe(manifest);
        super::Action::describe(manifest.with_name_override("command"));
    }
}

impl Parse<'_> for BindKey {
    fn parse(args: &mut ParseArgs<'_>) -> Result<Self, Self::Error> {
        let key = Key::parse(args)?;
        let command = Box::new(super::Action::parse(args)?);
        Ok(BindKey { key, command })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Key(String);

impl FromStr for Key {
    type Err = RootError;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        if RE_VALID_KEY.is_match(key) {
            Ok(Key(key.to_owned()))
        } else {
            Err(RootError::InvalidKeycode(key.to_owned()))
        }
    }
}
impl Describe<'_> for Key {
    fn describe(manifest: &mut Manifest) {
        manifest
            .with_name("key")
            .with_completion(CompletionKey::Key)
            .with_description("A keyboard keybind. Example: C-S-k is Ctrl+Shft+k")
            .finish_arg();
    }
}

impl Parse<'_> for Key {
    fn parse(args: &mut ParseArgs<'_>) -> Result<Self, Self::Error> {
        args.next_arg().and_then(Self::from_str)
    }
}

lazy_static::lazy_static! {
    static ref RE_VALID_KEY: Regex = Regex::new(r#"^(C\-)?(S\-)?(A\-)?[a-z1-9]$"#).unwrap();
}
