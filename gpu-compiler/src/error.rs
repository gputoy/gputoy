use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing file {0}: {1}")]
    ParseError(String, naga::front::wgsl::ParseError),
}
