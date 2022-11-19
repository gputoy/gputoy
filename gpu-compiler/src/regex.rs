use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref RE_CAPTURE_IMPORT: Regex =
        Regex::new(r#"@import\s+(?P<ident>[a-zA-Z_][a-zA-Z0-9]*)\s*"#).unwrap();
    pub static ref RE_CAPTURE_EXPORT: Regex = Regex::new(
        r#"@export\s+(?P<struct>struct\s+(?P<ident>[a-zA-Z_][a-zA-Z0-9]*)\s*\{[^}]*}\s*;)"#
    )
    .unwrap();
    pub static ref RE_REPLACE_IMPORT: Regex = Regex::new(r#"@import\s+"#).unwrap();
    pub static ref RE_REPLACE_EXPORT: Regex = Regex::new(r#"@export\s+"#).unwrap();
}

/// Identical to regex::Match, except the text is owned
/// and it can be serialized.
/// TODO: get refs to work within the compiler instead of owned strings.
#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

impl<'a> From<&'a Match> for &'a str {
    #[inline]
    fn from(m: &'a Match) -> Self {
        &m.text
    }
}

impl From<regex::Match<'_>> for Match {
    fn from(m: regex::Match) -> Self {
        Match {
            text: m.as_str().to_owned(),
            start: m.start(),
            end: m.end(),
        }
    }
}

impl From<&regex::Match<'_>> for Match {
    fn from(m: &regex::Match) -> Self {
        Match {
            text: m.as_str().to_owned(),
            start: m.start(),
            end: m.end(),
        }
    }
}
