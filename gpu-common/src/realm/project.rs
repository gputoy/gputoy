use chrono::NaiveDateTime;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;
use validator_derive::Validate;

use crate::{Config, Files, Layout};

#[derive(Debug, Validate)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct ProjectUpsert {
    pub id: Option<String>,
    #[validate(length(min = 3, max = 50))]
    pub title: String,
    pub description: Option<String>,
    pub files: Files,
    pub layout: Option<Layout>,
    pub config: Option<Config>,
    pub published: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    any(feature = "serialize", feature = "deserialize"),
    serde(rename_all = "camelCase")
)]
pub struct ProjectResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub files: Files,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub layout: Option<Layout>,
    #[cfg_attr(
        any(feature = "serialize", feature = "deserialize"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub config: Option<Config>,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub author_id: Option<String>,
    pub forked_from_id: Option<String>,
}
