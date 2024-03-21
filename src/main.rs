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
use services::whatsminer::route::scoped_config as whatsminer_cfg;
use tasks::register_tasks;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
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
            .service(web::scope("/api").configure(whatsminer_cfg))
    })
    .workers(4)
    .bind((cfg.host, cfg.port)).unwrap()
    .run()
    .await
}
