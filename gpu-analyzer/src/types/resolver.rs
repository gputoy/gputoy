use std::fmt::Display;

use lazy_static::lazy_static;
use regex::Regex;

use crate::model::{Location, ModelKey};

lazy_static! {
    pub(crate) static ref RE_CAPTURE_IMPORT: Regex =
        Regex::new(r#"@import\s+(?P<ident>[a-zA-Z_][a-zA-Z0-9]*)\s*"#).unwrap();
    pub(crate) static ref RE_CAPTURE_EXPORT: Regex = Regex::new(
        r#"@export\s+(?P<struct>struct\s+(?P<ident>[a-zA-Z_][a-zA-Z0-9]*)\s*\{[^}]*}\s*;)"#
    )
    .unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModelImport {
    /// Which model to resolve import from
    pub from: ModelKey,
    /// Which index in the model's export list to resolve import from
    pub idx: usize,
    /// Location of import declaration
    pub location: Location,
}

impl Display for ModelImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {} at idx {}", self.from, self.location, self.idx)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModelExport {
    /// The export's identifier
    pub ident: String,
    /// Location of export declaration
    pub location: Location,
}
