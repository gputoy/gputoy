use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// CREATE TABLE IF NOT EXISTS users (
//     id uuid default uuid_generate_v4() PRIMARY KEY,
//     username VARCHAR(32) UNIQUE NOT NULL,
//     email VARCHAR(50) UNIQUE NOT NULL,
//     password VARCHAR(150) NOT NULL,
//     full_name VARCHAR(32) NULL,
//     bio VARCHAR NULL,
//     image VARCHAR NULL,
//     email_verified BOOLEAN NOT NULL default false,
//     active BOOLEAN NOT NULL default true,
//     created_at TIMESTAMP NOT NULL default current_timestamp,
//     updated_at TIMESTAMP NOT NULL default current_timestamp
//   );
#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
