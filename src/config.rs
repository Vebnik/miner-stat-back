use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::{Pool, Sqlite};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub app_name: String,
    pub host: String,
    pub port: u16,
    pub db_url: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub cfg: Arc<Config>,
    pub db: Arc<Pool<Sqlite>>,
}