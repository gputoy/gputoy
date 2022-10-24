use crate::user::UserConfig;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Validate, Deserialize, JsonSchema)]
pub struct NewUser {
    #[validate(length(min = 3, max = 31))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 40))]
    pub password: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Credentials {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct NewUserResponse {
    pub id: String,
}

#[derive(Debug, Serialize, JsonSchema)]
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
    pub config: Option<UserConfig>,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user_id: String,
}
