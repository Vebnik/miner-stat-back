use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub app_name: String,
    pub host: String,
    pub port: u16,
    pub db_url: String,
}