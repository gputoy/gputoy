use chrono::NaiveDateTime;
use gpu_core::project::{config::Config, Files};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::{Json, JsonValue},
    FromRow,
};
use uuid::Uuid;

// CREATE TABLE projects (
//   id uuid default uuid_generate_v4() PRIMARY KEY,
//   title VARCHAR(50) NOT NULL,
//   description TEXT,
//   files JSON NOT NULL,
//   layout JSON,
//   config JSON,
//   published BOOLEAN NOT NULL DEFAULT false,
//   created_at TIMESTAMP(3) NOT NULL DEFAULT current_timestamp,
//   updated_at TIMESTAMP(3) NOT NULL DEFAULT current_timestamp,
//   CONSTRAINT author_id FOREIGN KEY(id) REFERENCES users(id) ON DELETE SET NULL,
//   CONSTRAINT forked_from_id FOREIGN KEY(id) REFERENCES projects(id)
// );
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub files: Json<Files>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Json<Config>>,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub author_id: Option<Uuid>,
    pub forked_from_id: Option<Uuid>,
}

impl TryInto<gpu_core::project::Project> for Project {
    type Error = std::convert::Infallible;
    fn try_into(self) -> Result<gpu_core::project::Project, Self::Error> {
        Ok(gpu_core::project::Project {
            config: self.config.map(|s| s.0),
            files: self.files.0,
            layout: Some(()),
        })
    }
}
