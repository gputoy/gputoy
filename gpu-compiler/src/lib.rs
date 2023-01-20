mod compiler;
mod error;
mod utils;

pub use compiler::Compiler;
pub use error::Error;

// pub struct Builder {
//     linked: crate::Linker,
//     validator: crate::Validator,
// }

// impl Builder {
//     pub fn process_delta(&mut self) {

//     }
//     pub fn build() ->  {

//     }
// }

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
