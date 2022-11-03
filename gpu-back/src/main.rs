mod realm;
mod store;
mod util;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use thiserror::Error;

use std::sync::Arc;

use crate::store::project::ProjectRepository;
use crate::store::user::UserRepository;
//use tracing_log::LogTracer;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Store(#[from] store::error::Error),
    #[error(transparent)]
    Runtime(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    //tracing_log::LogTracer::init().expect("LogTracer::init");

    let port = std::env::var("PORT_BACK")
        .map(|p| p.parse::<u16>().expect("Port value invalid"))
        .expect("Port environment variable");

    let cors_allowed =
        std::env::var("CORS_ALLOW").unwrap_or_else(|_| "http://localhost:3000".into());

    let redis_url = std::env::var("REDIS_URL").expect("Redis url environment variable");

    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new(&redis_url)
        .await
        .expect("Set up redis session cache");

    let pool = store::db_pool().await?;
    let pool = Arc::new(pool);
    let user_repo = Arc::new(UserRepository::new(&pool));
    let project_repo = Arc::new(ProjectRepository::new(&pool));

    log::info!("Connected to database");

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(
                Cors::default()
                    .allowed_origin(&cors_allowed)
                    .supports_credentials()
                    .expose_any_header(),
            )
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(user_repo.clone()))
            .app_data(Data::new(project_repo.clone()))
            .service(crate::realm::user::sign_up)
            .service(crate::realm::user::login)
            .service(crate::realm::user::user_info)
            .service(crate::realm::user::update_user_info)
            .service(crate::realm::user::logout)
            .service(crate::realm::project::post_project)
            .service(crate::realm::project::get_project)
            .service(crate::realm::project::delete_project)
            .service(crate::realm::project::get_user_projects)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
    .map_err(From::from)
}
