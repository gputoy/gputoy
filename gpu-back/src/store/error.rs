use thiserror::Error;

use crate::realm::error::{ApiError, ApiErrorType};

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("Error reading from environment variable: {0}")]
    Env(#[from] std::env::VarError),
    #[error("Error parsing uuid")]
    Uuid(#[from] uuid::Error),
}

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        match err {
            Error::Db(_) | Error::Migrate(_) | Error::Env(_) | Error::Uuid(_) => {
                ApiErrorType::InternalServerError.into()
            }
        }
    }
}
