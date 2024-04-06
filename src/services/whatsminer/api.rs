use serde_json::json;
use std::sync::Arc;
use sqlx::{self, Pool, Sqlite};
use actix_web::web;
use std::{io::{Read, Write}, net::TcpStream};

use crate::services::whatsminer::types::{Client, Command, Summary, CreateWorker, DeleteWorker};
use crate::services::whatsminer::models::{Worker, Statistic as TimeStatistic};
use crate::error::{Result, CustomError};

use super::types::{Statistic, StatisticFilter};

impl Client {
    pub async fn new(addr: String, port: String, name: String, is_write: bool) -> Result<Self> {
        let url = format!("{}:{}", addr, port);

        let stream = TcpStream::connect(&url)
            .map_err(|_| CustomError::TcpError)?;

        let mut client = Self {addr, port, stream, name};
        if is_write { client.get_token() };

        Ok(client)
    }

    fn get_token(&mut self) -> () {
        let mut buffer = String::new();

        self.stream.write("{\"cmd\": \"get_token\"}".as_bytes()).unwrap();
        self.stream.read_to_string(&mut buffer).unwrap();

        let mut data = serde_json::from_str::<serde_json::Value>(&buffer)
            .expect("Error in parse json");

        let token_msg = data["Msg"].take();

        println!("{:?}", token_msg);
    }

    pub fn exec_command(&mut self, cmd_type: Command) -> Result<serde_json::Value> {
        let payload = json!({"cmd": cmd_type});
        let cmd = payload.to_string();
        let mut buffer = String::new();
        
        self.stream.write(cmd.as_bytes()).unwrap();
        self.stream.read_to_string(&mut buffer).unwrap();

        let raw_json: serde_json::Value = serde_json::from_str(&buffer)
            .map_err(|_| CustomError::UbError)?;

        Ok(raw_json)
    }

    pub async fn summary(&mut self) -> Result<Summary> {
        let mut raw_data = self.exec_command(Command::Summary)?;

        let summary = serde_json::from_value::<Summary>(raw_data["SUMMARY"][0].take())
            .map_err(|_| CustomError::DbError)?;

        Ok(summary)
    }
}

impl Worker {
    pub async fn all(db_pool: Arc<Pool<Sqlite>>) -> Result<Vec<Self>> {
        sqlx::query_as!(
            Worker,
            r#"SELECT * FROM "worker""#,
        )
        .fetch_all(db_pool.as_ref())
        .await
        .map_err(|_| CustomError::DbError)  
    }

    pub async fn create(db_pool: Arc<Pool<Sqlite>>, data: web::Json<CreateWorker>) -> Result<Self> {
        sqlx::query_as!(
            Worker,
            r#"
                INSERT INTO "worker" ("port","host","name")
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            data.port,
            data.host,
            data.name,
        )
        .fetch_one(db_pool.as_ref())
        .await
        .map_err(|_| CustomError::DbError)
    }

    pub async fn delete(db_pool: Arc<Pool<Sqlite>>, id: i64) -> Result<DeleteWorker> {
        sqlx::query_as!(
            DeleteWorker,
            r#"
                DELETE FROM "worker" WHERE "id" = $1
                RETURNING "id"
            "#,
            id,
        )
        .fetch_one(db_pool.as_ref())
        .await
        .map_err(|_| CustomError::DbError)
    }
}

impl TimeStatistic {
    pub async fn create(db_pool: Arc<Pool<Sqlite>>, data: Statistic) -> Result<TimeStatistic> {
        sqlx::query_as!(
            TimeStatistic,
            r#"
                INSERT INTO "statistic" ("worker_id","mhs_av","temp","uptime","power","fan_in","fan_out","works")
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING *
            "#,
            data.worker.id,
            data.summary.mhs_av,
            data.summary.temp,
            data.summary.uptime,
            data.summary.power,
            data.summary.fan_in,
            data.summary.fan_out,
            data.summary.works,
        )
        .fetch_one(db_pool.as_ref())
        .await
        .map_err(|_| CustomError::DbError)
    }

    pub async fn by_worker(db_pool: Arc<Pool<Sqlite>>, worker_id: i64, filter_data: web::Query<StatisticFilter>) -> Result<Vec<TimeStatistic>> {
        sqlx::query_as!(
            TimeStatistic,
            r#"
                SELECT * FROM "statistic"
                WHERE "created_at" BETWEEN $1 and $2
                AND "worker_id" = $3
                ORDER by "created_at" ASC
            "#,
            filter_data.start_date,
            filter_data.end_date,
            worker_id,
        )
        .fetch_all(db_pool.as_ref())
        .await
        .map_err(|_| CustomError::DbError)
    }
}