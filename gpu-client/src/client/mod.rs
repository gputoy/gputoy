use thiserror::Error;

mod client;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Context(#[from] crate::context::Error),
}
