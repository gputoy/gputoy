#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

/// Identical to regex::Match, except the text is owned
/// and it can be serialized.
/// TODO: get refs to work within the compiler instead of owned strings.
#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
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

pub type FastHashMap<K, V> = rustc_hash::FxHashMap<K, V>;

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct PrebuildResult {
    pub dependency_info: DependencyInfo,
    pub file_builds: FastHashMap<String, FilePrebuildResult>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct DependencyInfo {
    pub deps: FastHashMap<String, FileDependencyInfo>,
}

impl DependencyInfo {
    pub fn find_imports_for_file<S: AsRef<str>>(&self, fileid: S) -> Vec<&str> {
        if let Some(dep_info) = self.deps.get(fileid.as_ref()) {
            dep_info
                .imports
                .iter()
                .filter_map(|import| self.find_export(&import.text))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn find_export<S: AsRef<str>>(&self, ident: S) -> Option<&str> {
        self.deps
            .iter()
            .find(|(_, dep)| dep.exports.contains_key(ident.as_ref()))
            .and_then(|(_, dep)| dep.exports.get(ident.as_ref()).map(From::from))
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct FilePrebuildResult {
    pub processed_shader: crate::File,
    pub raw_module: Option<crate::naga::Module>,
    pub errors: Option<Vec<crate::naga::CompileError>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct FileDependencyInfo {
    pub imports: Vec<Match>,
    // rename to not collide with ts export keyword
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(rename = "exxports")
    )]
    pub exports: FastHashMap<String, Match>,
    pub errors: Option<Vec<crate::naga::CompileError>>,
}
