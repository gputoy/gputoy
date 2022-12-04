use std::fmt::Display;

#[derive(Debug)]
pub enum RootError {
    InvalidFilePath(String),
}

impl Display for RootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFilePath(path) => write!(f, "Invalid file path: {path}"),
        }
    }
}
