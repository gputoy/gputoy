use std::sync::Arc;

use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::{realm::ApiResult, store::user::UserRepository};

#[derive(Debug, Validate, Deserialize)]
pub struct NewUser {
    #[validate(length(min = 3, max = 31))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 40))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct NewUserResponse {
    pub id: uuid::Uuid,
}

#[post("/signup")]
pub async fn sign_up(
    form: web::Form<NewUser>,
    user_repository: web::Data<Arc<UserRepository>>,
) -> ApiResult {
    let new_user = form.into_inner();
    let response = user_repository.create(new_user).await?;
    Ok(HttpResponse::Ok().json(NewUserResponse { id: response.id }))
}

#[get("/test")]
pub async fn get_test() -> ApiResult {
    Ok(HttpResponse::Ok().body("Hello, world!"))
}
