pub mod config;
mod files;
pub use files::*;

use config::Config;

#[derive(Debug)]
pub struct Project {
    pub files: Files,
    pub layout: Option<()>,
    pub config: Option<Config>,
}
