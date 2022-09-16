use std::sync::Arc;

use crate::{model::User, realm::user::NewUser};
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

    pub async fn create(&self, new_user: NewUser) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING *",
            new_user.username,
            new_user.email,
            new_user.password
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn find_by_username(&self, username: &String) -> Result<User, Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_one(&*self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn find_by_email(&self, email: &String) -> Result<User, Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&*self.pool)
            .await
            .map_err(From::from)
    }
}

// impl FromRequest for UserRepository {
//     type Error = ApiError;

//     type Future = Ready<Result<Self, Self::Error>>;

//     fn from_request(
//         req: &actix_web::HttpRequest,
//         payload: &mut actix_web::dev::Payload,
//     ) -> Self::Future {
//         let pool = Data::<Arc<PgPool>>::from_request(req, payload).into_inner();
//         match pool {
//             Ok(pool) => ready(Ok(UserRepository::new(&pool))),
//             Err(err) => {
//                 log::error!("Failed to retreieve user repository due to: {}", err);
//                 ready(Err((
//                     "Failed to acquire database",
//                     ApiErrorType::InternalServerError,
//                 )
//                     .into()))
//             }
//         }
//     }
// }
