use crate::preferences::UserPrefs;
use chrono::NaiveDateTime;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
use validator_derive::Validate;

#[derive(Debug, Validate)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct NewUser {
    #[validate(length(min = 3, max = 31))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 40))]
    pub password: String,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct Credentials {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct NewUserResponse {
    pub id: String,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct UserInfoResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub full_name: Option<String>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub bio: Option<String>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub image: Option<String>,
    pub email_verified: bool,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub config: Option<UserPrefs>,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct UpdateUserInfoArgs {
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub full_name: Option<String>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub bio: Option<String>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub image: Option<String>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub config: Option<UserPrefs>,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct LoginResponse {
    pub user_id: String,
}
