use lazy_static::lazy_static;
use regex::Regex;

pub(crate) const EXTRA_CHAR_PADDING: usize = 10;
pub(crate) const NEWLINE: char = 0xA as char;

lazy_static! {
    pub(crate) static ref RE_REPLACE_IMPORT: Regex = Regex::new(r#"@import\s+"#).unwrap();
    pub(crate) static ref RE_REPLACE_EXPORT: Regex = Regex::new(r#"@export\s+"#).unwrap();
}

#[derive(Debug)]
pub struct LinkResult {
    pub processed_src: String,
    pub inserts: Vec<LinkerInsertion>,
}

#[derive(Debug)]
pub struct LinkerInsertion {
    pub(crate) _delete: bool,
    pub(crate) _idx: usize,
    pub(crate) _len: usize,
}
