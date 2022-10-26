use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ApiErrorType {
    #[error("Requested resource not found.")]
    NotFound,
    #[error("Internal server error.")]
    InternalServerError,
    #[error("Invalid arguments")]
    InvalidArguments,
    #[error("Unauthorized")]
    Unauthorized,
}

#[derive(Debug)]
pub struct ApiError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: ApiErrorType,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.message
                .as_ref()
                .unwrap_or(&self.error_type.to_string()),
        )
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(_: ValidationErrors) -> Self {
        (
            "Validation failed for the following inputs: ",
            ApiErrorType::InvalidArguments,
        )
            .into()
    }
}

impl<T: AsRef<str>> From<(T, ApiErrorType)> for ApiError {
    fn from((message, error_type): (T, ApiErrorType)) -> Self {
        Self {
            message: Some((*message.as_ref()).to_string()),
            cause: None,
            error_type,
        }
    }
}

impl<T: AsRef<str>> From<(T, T, ApiErrorType)> for ApiError {
    fn from((message, cause, error_type): (T, T, ApiErrorType)) -> Self {
        Self {
            message: Some((*message.as_ref()).to_string()),
            cause: Some((*cause.as_ref()).to_string()),
            error_type,
        }
    }
}

impl From<ApiErrorType> for ApiError {
    fn from(error_type: ApiErrorType) -> Self {
        Self {
            message: None,
            cause: None,
            error_type,
        }
    }
}

/// Actual error type to be sent across-wire
#[derive(Serialize)]
struct ApiErrorResponse {
    message: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiErrorResponse {
            message: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            ApiErrorType::NotFound => StatusCode::NOT_FOUND,
            ApiErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorType::InvalidArguments => StatusCode::BAD_REQUEST,
            ApiErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
