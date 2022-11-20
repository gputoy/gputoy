use crate::error::Error;
use gpu_common::{
    DependencyInfo, FastHashMap, FileDependencyInfo, FilePrebuildResult, PrebuildResult,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, vec};

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

impl Compiler {
    pub fn new() -> Self {
        Self {
            wgsl_parser: RefCell::new(naga::front::wgsl::Parser::new()),
            _glsl_parser: RefCell::new(naga::front::glsl::Parser::default()),
            _internal_files: Default::default(),
        }
    }
    pub fn prebuild(&mut self, files: &gpu_common::Files) -> Result<PrebuildResult, Error> {
        let dependency_info = Self::get_dependency_info(files);
        let prebuild: FastHashMap<String, _> = files
            .map
            .iter()
            .filter(|(_, file)| file.extension.is_shader())
            .map(|(fileid, file)| {
                (
                    fileid.clone(),
                    self.prebuild_file(fileid, file, &dependency_info),
                )
            })
            .collect();
        log::info!("Prebuild: {prebuild:#?} \n\n {dependency_info:#?}");

        Ok(PrebuildResult {
            dependency_info,
            file_builds: prebuild,
        })
    }

    pub fn prebuild_file(
        &self,
        fileid: &String,
        file: &gpu_common::File,
        depenency_info: &DependencyInfo,
    ) -> FilePrebuildResult {
        let processed_shader = Self::preprocess_file(fileid, file, depenency_info);
        let (raw_module, errors) = match self.wgsl_parser.borrow_mut().parse(&processed_shader.data)
        {
            Ok(module) => (Some(module), None),
            Err(err) => (
                None,
                Some(vec![(&err, processed_shader.data.as_ref()).into()]),
            ),
        };

        FilePrebuildResult {
            processed_shader,
            raw_module,
            errors,
        }
    }

    fn preprocess_file(
        fileid: &String,
        file: &gpu_common::File,
        depenency_info: &DependencyInfo,
    ) -> gpu_common::File {
        let data = RE_REPLACE_IMPORT.replace_all(&file.data, "");
        let data = RE_REPLACE_EXPORT.replace_all(&data, "");
        let mut imports = depenency_info.find_imports_for_file(fileid);
        imports.push(data.as_ref());
        let data = imports.join("\n");
        gpu_common::File {
            file_name: format!("{}_processed", file.file_name),
            data,
            extension: file.extension,
            dir: String::from(".generated"),
            fetch: None,
        }
    }

    fn get_dependency_info(files: &gpu_common::Files) -> DependencyInfo {
        let deps: FastHashMap<_, _> = files
            .map
            .iter()
            .filter(|(_, file)| file.extension.is_shader())
            .map(|(fileid, file)| {
                (
                    fileid.clone(),
                    FileDependencyInfo {
                        imports: Self::get_file_imports(file),
                        exports: Self::get_file_exports(file),
                        errors: None,
                    },
                )
            })
            .collect();
        DependencyInfo { deps }
    }

    fn get_file_imports(file: &gpu_common::File) -> Vec<gpu_common::Match> {
        RE_CAPTURE_IMPORT
            .captures_iter(&file.data)
            .map(|cap| cap.name("ident").unwrap().into())
            .collect()
    }

    fn get_file_exports(file: &gpu_common::File) -> FastHashMap<String, gpu_common::Match> {
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

    // #[test]
    // fn test_preprocess() {
    //     let files = get_test_files("test_imports");
    //     let shader = &files["/main.wgsl"];
    //     let expected = &files["/.generated/main_processed.wgsl"];

    //     let actual = Compiler::preprocess_file(shader);

    //     assert_eq!(actual.data, expected.data);
    //     assert_eq!(actual.dir, expected.dir);
    //     assert_eq!(actual.file_name, expected.file_name);
    //     assert_eq!(actual.extension, expected.extension);
    // }
}
