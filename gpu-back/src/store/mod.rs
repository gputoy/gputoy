use std::time::Duration;

use error::Error;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod error;
pub mod model;
pub mod project;
pub mod user;

const DB_POOL_MAX_OPEN: u32 = 32;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub async fn db_pool() -> Result<PgPool, Error> {
    let uri = std::env::var("DATABASE_URL")?;

    PgPoolOptions::new()
        .max_connections(DB_POOL_MAX_OPEN)
        .idle_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .connect(&uri)
        .await
        .map_err(From::from)
}
