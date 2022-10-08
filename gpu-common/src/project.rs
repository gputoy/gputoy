use crate::{Files, ProjectConfig};

#[derive(Debug)]
pub struct Project {
    pub files: Files,
    pub layout: Option<()>,
    pub config: Option<ProjectConfig>,
}
