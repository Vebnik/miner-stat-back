mod services;
mod router;
mod config;
mod error;
mod tasks;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;
use sqlx::sqlite::SqlitePool;

use config::{Config, AppState};
use router::init_api_service;
use tasks::register_tasks;

use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok().expect("Error on parsing .env");
    env_logger::init();

    let cfg = envy::from_env::<Config>().expect("Error on parsing .env");
    let db_pool = SqlitePool::connect(&cfg.db_url).await.expect("Erro to connect db");
    
    let app_state = AppState {
        cfg: Arc::new(cfg.clone()),
        db: Arc::new(db_pool),
    };

    log::info!("Try to register tasks");
    register_tasks(Arc::new(app_state.clone())).await.expect("Error in register tasks");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(init_api_service(web::scope("/api")))
    })
    .workers(4)
    .bind((cfg.host, cfg.port)).unwrap()
    .run()
    .await
    .expect("Error in create HttpServer");

    Ok(())
}
