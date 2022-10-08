use std::sync::Arc;

use actix_identity::Identity;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::{
    realm::{
        error::{ApiError, ApiErrorType},
        ApiResult,
    },
    store::model::UserRow,
    store::user::UserRepository,
    util::to_base64,
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
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    pub email_verified: bool,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<UserRow> for UserInfoResponse {
    fn from(user: UserRow) -> Self {
        Self {
            id: to_base64(&user.id),
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            bio: user.bio,
            image: user.image,
            email_verified: user.email_verified,
            active: user.active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResult {
    user_id: String,
}

#[post("/signup")]
pub async fn sign_up(
    form: web::Form<NewUser>,
    user_repository: web::Data<Arc<UserRepository>>,
) -> ApiResult {
    let new_user = form.into_inner();
    new_user.validate()?;
    let response = user_repository.create(new_user).await?;
    let id = to_base64(&response.id);
    Ok(HttpResponse::Ok().json(NewUserResponse { id }))
}

#[post("/login")]
pub async fn login(
    request: HttpRequest,
    ident: Option<Identity>,
    credentials: web::Form<Credentials>,
    user_repository: web::Data<Arc<UserRepository>>,
) -> ApiResult {
    log::info!("{request:?}\n{credentials:?}\n");

    if let Some(ident) = ident {
        log::info!("User {:?} already signed in", ident.id());
        ident.logout();
    }

    let credentials = credentials.into_inner();

    let res = tokio::join!(
        user_repository.find_by_username(&credentials.username_or_email),
        user_repository.find_by_email(&credentials.username_or_email),
    );

    let user = match res {
        (Ok(user), _) => Ok::<UserRow, ApiError>(user),
        (_, Ok(user)) => Ok(user),
        _ => Err((ApiErrorType::Unauthorized).into()),
    }?;

    Identity::login(&request.extensions(), user.id.to_string())
        .map_err(|err| ApiError::from((err.to_string(), ApiErrorType::InternalServerError)))?;

    Ok(HttpResponse::Ok().json(LoginResult {
        user_id: to_base64(&user.id),
    }))
}

#[get("/me")]
pub async fn user_info(
    ident: Identity,
    user_repositroy: web::Data<Arc<UserRepository>>,
) -> ApiResult {
    let id = ident
        .id()
        .map_err(|_| ("Invalid indentity", ApiErrorType::InternalServerError))?;
    let user = user_repositroy.find_by_id(&id).await?;
    Ok(HttpResponse::Ok().json(UserInfoResponse::from(user)))
}

#[post("/logout")]
pub async fn logout(ident: Identity) -> ApiResult {
    ident.logout();
    Ok(HttpResponse::Ok().finish())
}
