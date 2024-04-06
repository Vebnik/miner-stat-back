use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Worker {
    pub id: i64,
    pub host: String,
    pub port: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistic {
    pub id: Option<i64>,
    pub worker_id: i64,
    pub mhs_av: i64,
    pub temp: i64,
    pub uptime: i64,
    pub power: i64,
    pub fan_in: i64,
    pub fan_out: i64,
    pub works: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}