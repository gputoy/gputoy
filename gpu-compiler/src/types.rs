use naga::FastHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub deps: FastHashMap<String, FileDependencyInfo>,
}

impl DependencyInfo {
    pub fn find_imports_for_file<S: AsRef<str>>(&self, fileid: S) -> Vec<&str> {
        if let Some(dep_info) = self.deps.get(fileid.as_ref()) {
            dep_info
                .imports
                .iter()
                .map(|import| self.find_export(&import.text))
                .filter_map(|import| import)
                .collect()
        } else {
            vec![]
        }
    }

    pub fn find_export<S: AsRef<str>>(&self, ident: S) -> Option<&str> {
        self.deps
            .iter()
            .find(|(_, dep)| dep.exports.contains_key(ident.as_ref()))
            .map(|(_, dep)| dep.exports.get(ident.as_ref()).map(From::from))
            .flatten()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDependencyInfo {
    pub(crate) imports: Vec<crate::regex::Match>,
    pub(crate) exports: FastHashMap<String, crate::regex::Match>,
    pub(crate) errors: Option<Vec<crate::error::CompileError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePrebuildResult {
    pub(crate) processed_shader: gpu_common::File,
    pub(crate) raw_module: Option<naga::Module>,
    pub(crate) errors: Option<Vec<crate::error::CompileError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrebuildResult {
    pub dependency_info: DependencyInfo,
    pub file_builds: FastHashMap<String, FilePrebuildResult>,
}
