use std::sync::Arc;

use actix_identity::Identity;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::{
    model::User,
    realm::{
        error::{ApiError, ApiErrorType},
        ApiResult,
    },
    store::user::UserRepository,
};

#[derive(Debug, Validate, Deserialize)]
pub struct NewUser {
    #[validate(length(min = 3, max = 31))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 40))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username_or_email: String,
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
    new_user.validate()?;
    let response = user_repository.create(new_user).await?;
    Ok(HttpResponse::Ok().json(NewUserResponse { id: response.id }))
}

#[post("/login")]
pub async fn login(
    request: HttpRequest,
    ident: Option<Identity>,
    form: web::Form<Credentials>,
    user_repository: web::Data<Arc<UserRepository>>,
) -> ApiResult {
    if ident.is_some() {
        log::info!("User {form:?} already signed in");
        return Ok(HttpResponse::Ok().body("Already signed in"));
    }

    let credentials = form.into_inner();

    let res = tokio::join!(
        user_repository.find_by_username(&credentials.username_or_email),
        user_repository.find_by_email(&credentials.username_or_email),
    );

    let user = match res {
        (Ok(user), _) => Ok::<User, ApiError>(user),
        (_, Ok(user)) => Ok(user),
        _ => Err((ApiErrorType::Unauthorized).into()),
    }?;

    Identity::login(&request.extensions(), user.id.to_string())
        .map_err(|err| ApiError::from((err.to_string(), ApiErrorType::InternalServerError)))?;

    Ok(HttpResponse::Ok().body("Logged in"))
}

#[get("/test")]
pub async fn get_test() -> ApiResult {
    Ok(HttpResponse::Ok().body("Hello, world!"))
}
