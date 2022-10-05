pub mod error;
pub mod project;
pub mod user;
use actix_web::HttpResponse;

pub type ApiResult = Result<HttpResponse, error::ApiError>;
