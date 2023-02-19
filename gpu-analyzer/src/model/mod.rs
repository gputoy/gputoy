use std::fmt::Display;

use gpu_common::{FilePath, SupportedExtension};
use serde::Serialize;
use slotmap::new_key_type;

mod cache;
mod error;
#[cfg(feature = "service")]
mod service;

pub use cache::ModelCache;
pub use error::Error;
#[cfg(feature = "service")]
pub use service::*;

new_key_type! {
    pub struct ModelKey;
}

#[cfg(feature = "serde")]
pub(crate) fn serialize_key<S>(key: &ModelKey, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(&key.to_string())
}

impl Display for ModelKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let encoded = base32::encode(base32::Alphabet::Crockford, &self.0.as_ffi().to_le_bytes());
        write!(f, "{encoded}")
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "dev", derive(Serialize))]
pub struct Model {
    path: FilePath,
    data: String,
    extension: SupportedExtension,
}

impl Model {
    pub fn value(&self) -> &str {
        self.data.as_str()
    }

    pub fn value_in_range(&self, range: std::ops::Range<usize>) -> &str {
        &self.data[range]
    }

    pub fn json<T>(&self) -> Result<T, Error>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        match self.extension {
            SupportedExtension::Json => serde_json::from_str(&self.data)
                .map_err(|e| Error::InvalidJson(self.path.clone(), e)),
            _ => Err(Error::UnexpectedExtension {
                expected: SupportedExtension::Json,
                actual: self.path.clone(),
            }),
        }
    }
}

impl From<(gpu_common::FilePath, gpu_common::File)> for Model {
    fn from((path, file): (gpu_common::FilePath, gpu_common::File)) -> Self {
        Self {
            path,
            data: file.data,
            extension: file.extension,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Location {
    pub(crate) model: ModelKey,
    pub(crate) range: std::ops::Range<usize>,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}..{}]",
            self.model, self.range.start, self.range.end
        )
    }
}

impl Location {
    pub fn new(model: ModelKey, range: std::ops::Range<usize>) -> Self {
        Self { model, range }
    }
}
