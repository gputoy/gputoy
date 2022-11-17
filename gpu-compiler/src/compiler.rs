use crate::error::Error;
use gpu_common::Files;
use lazy_static::lazy_static;
use naga::FastHashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

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
    wgsl_parser: naga::front::wgsl::Parser,
    _glsl_parser: naga::front::glsl::Parser,
    _internal_files: gpu_common::Files,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            wgsl_parser: naga::front::wgsl::Parser::new(),
            _glsl_parser: naga::front::glsl::Parser::default(),
            _internal_files: Default::default(),
        }
    }
    pub fn build(&mut self, files: &gpu_common::Files) -> Result<CompiledProject, Error> {
        let capacity = files.map.len();
        let mut raw_modules = FastHashMap::with_capacity_and_hasher(capacity, Default::default());
        let file = files.map.get("/shaders/main.wgsl").unwrap();
        let _imports = Self::get_file_imports(file);
        let _exports = Self::get_file_exports(file);
        let _p_file = self.preprocess_file(file);
        let module = self
            .wgsl_parser
            .parse(&file.data)
            .map_err(|err| Error::ParseError(file.canonical_name(), err))?;
        module
            .types
            .iter()
            .for_each(|(_, ty)| log::info!("{:?}", ty));
        raw_modules.insert(file.canonical_name(), module);

        Ok(CompiledProject {
            raw_modules,
            transformed_files: Default::default(),
        })
    }

    fn get_file_imports(file: &gpu_common::File) -> Vec<&str> {
        RE_CAPTURE_IMPORT
            .captures_iter(&file.data)
            .map(|cap| cap.name("ident").unwrap().as_str())
            .collect()
    }

    fn get_file_exports(file: &gpu_common::File) -> FastHashMap<&str, &str> {
        RE_CAPTURE_EXPORT
            .captures_iter(&file.data)
            .map(|cap| {
                (
                    cap.name("ident").unwrap().as_str(),
                    cap.name("struct").unwrap().as_str(),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CompiledProject {
    pub raw_modules: naga::FastHashMap<String, naga::Module>,
    pub transformed_files: Files,
    // pub types: naga::FastHashMap<String, TypeDesc>,
}

#[cfg(test)]
mod tests {
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
        assert_eq!(imports, vec!["FragIn", "TestStruct0", "TestStruct1"]);
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
            imports.values().collect::<Vec<_>>(),
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
