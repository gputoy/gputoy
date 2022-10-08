use std::sync::Arc;
use uuid::Uuid;

use crate::{realm::project::ProjectUpsert, store::model::ProjectRow};

use sqlx::types::Json;
use sqlx::PgPool;

use super::Error;

const UPDATE_QUERY: &str = r#"
    UPDATE projects SET 
        (title, description, files, layout, config, published) =
        ($1, $2, $3, $4, $5, $6)
    WHERE id = $7
    RETURNING *
"#;

const INSERT_QUERY: &str = r#"
    Insert INTO projects 
        (author_id, title, description, files, layout, config, published)
    VALUES ($1, $2, $3, $4, $5, $6, $7) 
    RETURNING *
"#;

const FIND_BY_ID_QUERY: &str = r#"
    SELECT * FROM projects WHERE id = $1
"#;

const FIND_BY_USER_QUERY: &str = r#"
    SELECT * FROM projects WHERE author_id = $1
"#;

const DELETE_QUERY: &str = r#"
    DELETE FROM projects WHERE id = $1
"#;

pub struct ProjectRepository {
    pool: Arc<PgPool>,
}

impl ProjectRepository {
    pub fn new(pool: &Arc<PgPool>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn upsert(
        &self,
        author_id: &Uuid,
        decoded_project_id: Option<Uuid>,
        project: ProjectUpsert,
    ) -> Result<ProjectRow, Error> {
        log::info!("Upsert args: {:?}", project);

        if let Some(project_id) = decoded_project_id {
            self.update(&project_id, project).await
        } else {
            self.insert(author_id, project).await
        }
    }

    pub async fn insert(
        &self,
        author_id: &Uuid,
        project: ProjectUpsert,
    ) -> Result<ProjectRow, Error> {
        let files = Json(project.files);
        let config = project.config.map(Json);
        sqlx::query_as(INSERT_QUERY)
            .bind(author_id)
            .bind(project.title)
            .bind(project.description)
            .bind(files)
            .bind(project.layout)
            .bind(config)
            .bind(project.published)
            .fetch_one(&*self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn update(
        &self,
        project_id: &Uuid,
        project: ProjectUpsert,
    ) -> Result<ProjectRow, Error> {
        let files = Json(project.files);
        let config = project.config.map(Json);
        sqlx::query_as(UPDATE_QUERY)
            .bind(project.title)
            .bind(project.description)
            .bind(files)
            .bind(project.layout)
            .bind(config)
            .bind(project.published)
            .bind(project_id)
            .fetch_one(&*self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn find_by_id(&self, project_id: &Uuid) -> Result<ProjectRow, Error> {
        sqlx::query_as(FIND_BY_ID_QUERY)
            .bind(project_id)
            .fetch_one(&*self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn find_by_user(&self, user_id: &Uuid) -> Result<Vec<ProjectRow>, Error> {
        sqlx::query_as(FIND_BY_USER_QUERY)
            .bind(user_id)
            .fetch_all(&*self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn delete(&self, project_id: &Uuid) -> Result<(), Error> {
        sqlx::query(DELETE_QUERY)
            .bind(project_id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(Error::Db)?;
        Ok(())
    }
}
