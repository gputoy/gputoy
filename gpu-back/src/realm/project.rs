use std::{str::FromStr, sync::Arc};

use actix_identity::Identity;
use actix_web::{delete, get, post, web, HttpResponse};
use chrono::NaiveDateTime;
use gpu_common::{Files, ProjectConfig};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use uuid::Uuid;

use crate::{
    realm::{error::ApiErrorType, ApiResult},
    store::{model::ProjectRow, project::ProjectRepository},
    util::{from_base64, to_base64},
};

#[derive(Debug, Deserialize)]
pub struct ProjectUpsert {
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub files: Files,
    pub layout: Option<JsonValue>,
    pub config: Option<ProjectConfig>,
    pub published: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub files: Files,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ProjectConfig>,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub author_id: Option<String>,
    pub forked_from_id: Option<String>,
}

impl From<ProjectRow> for ProjectResponse {
    fn from(project: ProjectRow) -> Self {
        Self {
            id: to_base64(&project.id),
            title: project.title,
            description: project.description,
            files: project.files.0,
            layout: project.layout,
            config: project.config.map(|c| c.0),
            published: project.published,
            created_at: project.created_at,
            updated_at: project.updated_at,
            author_id: project.author_id.as_ref().map(to_base64),
            forked_from_id: project.forked_from_id.as_ref().map(to_base64),
        }
    }
}

#[post("/project")]
pub async fn post_project(
    web::Json(project): web::Json<ProjectUpsert>,
    project_repository: web::Data<Arc<ProjectRepository>>,
    identity: Identity,
) -> ApiResult {
    let id = identity
        .id()
        .map_err(|_| ("Invalid indentity", ApiErrorType::InternalServerError))?;
    let id = Uuid::from_str(&id).map_err(|_| ("Invalid id", ApiErrorType::InternalServerError))?;

    let mut decoded_project_id = None;

    // If project exists, determine if current user can modify project
    if let Some(ref project_id) = project.id {
        decoded_project_id = Some(
            from_base64(project_id).map_err(|_| ("Invalid id", ApiErrorType::InvalidArguments))?,
        );
        let current_project = project_repository
            .find_by_id(&decoded_project_id.unwrap())
            .await
            .ok();

        if let Some(author_id) = current_project.and_then(|s| s.author_id) {
            if id != author_id {
                return Err(("Project belongs to other user", ApiErrorType::Unauthorized).into());
            }
        }
    }
    let project = project_repository
        .upsert(&id, decoded_project_id, project)
        .await?;

    Ok(HttpResponse::Ok().json(ProjectResponse::from(project)))
}

#[get("/project/{project_id}")]
pub async fn get_project(
    project_id: web::Path<String>,
    project_repository: web::Data<Arc<ProjectRepository>>,
    identity: Option<Identity>,
) -> ApiResult {
    let project_id =
        from_base64(&project_id).map_err(|_| ("Invalid id", ApiErrorType::InvalidArguments))?;
    let project = project_repository
        .find_by_id(&project_id)
        .await
        .map_err(|_| ("Project not found", ApiErrorType::NotFound))?;

    if !project.published
        && project.author_id.map(|id| id.to_string()) != identity.and_then(|i| i.id().ok())
    {
        Err(("Project is private", ApiErrorType::Unauthorized).into())
    } else {
        Ok(HttpResponse::Ok().json(ProjectResponse::from(project)))
    }
}

#[get("/project/user/{user_id}")]
pub async fn get_user_projects(
    user_id: web::Path<String>,
    project_repository: web::Data<Arc<ProjectRepository>>,
    identity: Option<Identity>,
) -> ApiResult {
    let user_id =
        from_base64(&user_id).map_err(|_| ("Invalid id", ApiErrorType::InvalidArguments))?;
    let is_user = if let Some(identity) = identity {
        let id = identity
            .id()
            .map_err(|_| (ApiErrorType::InternalServerError))?;
        user_id == Uuid::from_str(&id).map_err(|_| (ApiErrorType::InternalServerError))?
    } else {
        false
    };
    log::info!("is user: {}. user_id: {}", is_user, user_id);
    let projects = project_repository
        .find_by_user(&user_id)
        .await?
        .into_iter()
        .filter(|proj| proj.published || is_user)
        .map(ProjectResponse::from)
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(projects))
}

#[delete("/project/{project_id}")]
pub async fn delete_project(
    project_id: web::Path<String>,
    project_repository: web::Data<Arc<ProjectRepository>>,
    identity: Identity,
) -> ApiResult {
    let user_id = identity
        .id()
        .map_err(|_| ("Invalid indentity", ApiErrorType::InternalServerError))?;
    let user_id =
        Uuid::from_str(&user_id).map_err(|_| ("Invalid id", ApiErrorType::InternalServerError))?;
    let project_id =
        from_base64(&project_id).map_err(|_| ("Invalid id", ApiErrorType::InternalServerError))?;

    let project = project_repository
        .find_by_id(&project_id)
        .await
        .map_err(|_| ("Project not found", ApiErrorType::NotFound))?;

    if user_id != project.author_id.unwrap_or_default() {
        Err(("Project belongs to other user", ApiErrorType::Unauthorized).into())
    } else {
        project_repository
            .delete(&project_id)
            .await
            .map_err(|_| (ApiErrorType::InternalServerError))?;
        Ok(HttpResponse::Ok().finish())
    }
}
