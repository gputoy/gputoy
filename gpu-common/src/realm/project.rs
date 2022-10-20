use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::{Config, Files, Layout};

#[derive(Debug, Deserialize, Validate, JsonSchema)]
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

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub files: Files,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub author_id: Option<String>,
    pub forked_from_id: Option<String>,
}
