mod model;
mod realm;
mod store;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use thiserror::Error;

use std::sync::Arc;

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

    let cors_allowed = std::env::var("CORS_ALLOW").unwrap_or("http://localhost:3000".into());

    let pool = store::db_pool().await?;
    let pool = Arc::new(pool);
    let user_repo = Arc::new(UserRepository::new(&pool));

    log::info!("Connected to database");

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Cors::default().allowed_origin(&cors_allowed))
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(user_repo.clone()))
            .service(crate::realm::user::sign_up)
            .service(crate::realm::user::get_test)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
    .map_err(From::from)
}
