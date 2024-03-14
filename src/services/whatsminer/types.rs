use std::net::TcpStream;

use serde::{Deserialize, Serialize};

use super::models::Worker;

pub struct Client {
    pub name: String,
    pub addr: String,
    pub port: String,
    pub stream: TcpStream
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Summary,
    GetToken,
    Pools,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Summary {
    #[serde(alias = "MHS av")]
    pub mhs_av: f32,
    #[serde(alias = "Temperature")]
    pub temp: f32,
    #[serde(alias = "Uptime")]
    pub uptime: i32,
    #[serde(alias = "Power_RT")]
    pub power: i32,
    #[serde(alias = "Fan Speed In")]
    pub fan_in: i32,
    #[serde(alias = "Fan Speed Out")]
    pub fan_out: i32,
    #[serde(alias = "Getworks")]
    pub works: i32
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateWorker {
    pub name: String,
    pub host: String,
    pub port: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Statistic {
    pub summary: Summary,
    pub worker: Worker,
}