use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Identical to regex::Match, except the text is owned
/// and it can be serialized.
/// TODO: get refs to work within the compiler instead of owned strings.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PrebuildResult {
    pub dependency_info: DependencyInfo,
    pub file_builds: FastHashMap<String, FilePrebuildResult>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FilePrebuildResult {
    pub processed_shader: crate::File,
    #[schemars(with = "ModuleProxy")]
    pub raw_module: Option<naga::Module>,
    pub errors: Option<Vec<CompileError>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FileDependencyInfo {
    pub imports: Vec<Match>,
    pub exports: FastHashMap<String, Match>,
    pub errors: Option<Vec<CompileError>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct CompileError {
    pub message: String,
    pub span: Option<SourceLocation>,
}

impl From<(&naga::front::wgsl::ParseError, &str)> for CompileError {
    fn from((err, src): (&naga::front::wgsl::ParseError, &str)) -> Self {
        Self {
            message: err.emit_to_string(src),
            span: err.location(src).map(From::from),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    /// 1-based line number.
    pub line_number: u32,
    /// 1-based column of the start of this span
    pub line_position: u32,
    /// 0-based Offset in code units (in bytes) of the start of the span.
    pub offset: u32,
    /// Length in code units (in bytes) of the span.
    pub length: u32,
}

impl From<naga::SourceLocation> for SourceLocation {
    fn from(loc: naga::SourceLocation) -> Self {
        Self {
            line_number: loc.line_number,
            line_position: loc.line_position,
            offset: loc.offset,
            length: loc.length,
        }
    }
}

/// naga::Module doesn't implement JsonSchema, so this struct will
/// act as an approximate schema for Module.
#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
struct ModuleProxy {
    constants: Vec<()>,
    entry_points: Vec<()>,
    functions: Vec<()>,
    global_variables: Vec<()>,
    types: Vec<()>,
}
