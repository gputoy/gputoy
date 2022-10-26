use std::sync::Arc;
use uuid::Uuid;

use crate::store::model::UserRow;
use gpu_common::realm::NewUser;
//use actix_web::{web::Data, FromRequest};
use sqlx::PgPool;

use super::Error;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: &Arc<PgPool>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(&self, new_user: NewUser) -> Result<UserRow, Error> {
        sqlx::query_as(
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(new_user.username)
        .bind(new_user.email)
        .bind(new_user.password)
        .fetch_one(&*self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<UserRow, Error> {
        sqlx::query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&*self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<UserRow, Error> {
        sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&*self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<UserRow, Error> {
        let uuid = Uuid::parse_str(id)?;
        sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(uuid)
            .fetch_one(&*self.pool)
            .await
            .map_err(From::from)
    }
}
