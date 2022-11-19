use crate::error::{self, Error};
use codespan_reporting::diagnostic::Diagnostic;
use lazy_static::lazy_static;
use naga::{front::wgsl::ParseError, FastHashMap};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

lazy_static! {
    static ref RE_CAPTURE_IMPORT: Regex =
        Regex::new(r#"@import\s+(?P<ident>[a-zA-Z_][a-zA-Z0-9]*)\s*"#).unwrap();
    static ref RE_CAPTURE_EXPORT: Regex = Regex::new(
        r#"@export\s+(?P<struct>struct\s+(?P<ident>[a-zA-Z_][a-zA-Z0-9]*)\s*\{[^}]*}\s*;)"#
    )
    .unwrap();
    static ref RE_REPLACE_IMPORT: Regex = Regex::new(r#"@import\s+"#).unwrap();
    static ref RE_REPLACE_EXPORT: Regex = Regex::new(r#"@export\s+"#).unwrap();
}

pub struct Compiler {
    pub(crate) wgsl_parser: RefCell<naga::front::wgsl::Parser>,
    _glsl_parser: RefCell<naga::front::glsl::Parser>,
    _internal_files: gpu_common::Files,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub text: String,
    pub start: usize,
    pub end: usize,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePrebuildResult {
    imports: Vec<Match>,
    exports: FastHashMap<String, Match>,
    processed_shader: gpu_common::File,
    raw_module: Option<naga::Module>,
    errors: Option<Vec<error::CompileError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompiledProject {
    pub file_builds: FastHashMap<String, FilePrebuildResult>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            wgsl_parser: RefCell::new(naga::front::wgsl::Parser::new()),
            _glsl_parser: RefCell::new(naga::front::glsl::Parser::default()),
            _internal_files: Default::default(),
        }
    }
    pub fn build(&mut self, files: &gpu_common::Files) -> Result<CompiledProject, Error> {
        let prebuild: FastHashMap<String, _> = files
            .map
            .iter()
            .filter(|(_, file)| file.extension.is_shader())
            .map(|(fileid, file)| (fileid.clone(), self.inspect_file(file)))
            .collect();
        log::info!("Prebuild: {prebuild:#?}");

        Ok(CompiledProject {
            file_builds: prebuild,
        })
    }

    pub fn inspect_file(&self, file: &gpu_common::File) -> FilePrebuildResult {
        let imports = Self::get_file_imports(file);
        let exports = Self::get_file_exports(file);
        let processed_shader = self.preprocess_file(file);
        let (raw_module, errors) = match self.wgsl_parser.borrow_mut().parse(&processed_shader.data)
        {
            Ok(module) => (Some(module), None),
            Err(err) => (
                None,
                Some(vec![(&err, processed_shader.data.as_ref()).into()]),
            ),
        };

        FilePrebuildResult {
            imports,
            exports,
            processed_shader,
            raw_module,
            errors,
        }
    }

    fn get_file_imports(file: &gpu_common::File) -> Vec<Match> {
        RE_CAPTURE_IMPORT
            .captures_iter(&file.data)
            .map(|cap| cap.name("ident").unwrap().into())
            .collect()
    }

    fn get_file_exports(file: &gpu_common::File) -> FastHashMap<String, Match> {
        RE_CAPTURE_EXPORT
            .captures_iter(&file.data)
            .map(|cap| {
                (
                    cap.name("ident").unwrap().as_str().to_owned(),
                    cap.name("struct").unwrap().into(),
                )
            })
            .collect()
    }

    fn preprocess_file(&self, file: &gpu_common::File) -> gpu_common::File {
        let data = RE_REPLACE_IMPORT.replace_all(&file.data, "");
        let data = RE_REPLACE_EXPORT.replace_all(&data, "");
        gpu_common::File {
            file_name: format!("{}_processed", file.file_name),
            data: data.into_owned(),
            extension: file.extension,
            dir: String::from(".generated"),
            fetch: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use gpu_common::Files;

    use super::*;
    use std::{path::Path, vec};

    fn get_test_files(path: &str) -> Files {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(&root).join("test_data").join(path);
        crate::utils::files_from_dir(&path).unwrap()
    }

    #[test]
    fn test_imports() {
        let files = get_test_files("test_imports");
        let shader = &files["/main.wgsl"];

        let imports = Compiler::get_file_imports(&shader);
        assert_eq!(
            imports.iter().map(|v| &v.text).collect::<Vec<_>>(),
            vec!["FragIn", "TestStruct0", "TestStruct1"]
        );
    }

    #[test]
    fn test_exports() {
        let files = get_test_files("test_imports");
        let shader = &files["/main.wgsl"];

        let imports = Compiler::get_file_exports(&shader);
        assert_eq!(
            imports.keys().collect::<Vec<_>>(),
            vec![&"MyStruct", &"FragIn"]
        );
        assert_eq!(
            imports.values().map(|v| &v.text).collect::<Vec<_>>(),
            vec![
                &r#"struct MyStruct {
    field: vec3<f32>,
    color: vec4<f32>,
    count: i32,
};"#,
                &r#"struct FragIn {
  @builtin(position) position: vec4<f32>,
  @location(0) uv: vec2<f32>,
};"#
            ]
        );
    }

    #[test]
    fn test_preprocess() {
        let files = get_test_files("test_imports");
        let shader = &files["/main.wgsl"];
        let expected = &files["/.generated/main_processed.wgsl"];

        let compiler = Compiler::new();
        let actual = compiler.preprocess_file(shader);

        assert_eq!(actual.data, expected.data);
        assert_eq!(actual.dir, expected.dir);
        assert_eq!(actual.file_name, expected.file_name);
        assert_eq!(actual.extension, expected.extension);
    }
}
