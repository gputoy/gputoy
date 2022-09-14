mod model;
mod realm;
mod store;

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

    let pool = store::db_pool().await?;
    let pool = Arc::new(pool);
    let user_repo = Arc::new(UserRepository::new(&pool));

    log::info!("Connected to database");

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(user_repo.clone()))
            .service(crate::realm::user::sign_up)
            .service(crate::realm::user::get_test)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
    .map_err(From::from)
}
