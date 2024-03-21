use serde_json::json;
use std::sync::Arc;
use sqlx::{self, Pool, Sqlite};
use actix_web::web;
use std::{io::{Read, Write}, net::TcpStream};

use crate::services::whatsminer::types::{Client, Command, Summary, CreateWorker, DeleteWorker};
use crate::services::whatsminer::models::{Worker, Statistic as TimeStatistic};
use crate::error::Error;

use super::types::Statistic;

impl Client {
    pub fn new(addr: String, port: String, name: String, is_write: bool) -> Result<Self, Error> {
        let url = format!("{}:{}", addr, port);

        let stream = TcpStream::connect(&url)
            .map_err(|_| Error::TcpError)?;

        let mut client = Self {addr, port, stream, name};

        if is_write { client.get_token() };

        Ok(client)
    }

    // private
    fn get_token(&mut self) -> () {
        let mut buffer = String::new();

        self.stream.write("{\"cmd\": \"get_token\"}".as_bytes()).unwrap();
        self.stream.read_to_string(&mut buffer).unwrap();

        let mut data = serde_json::from_str::<serde_json::Value>(&buffer)
            .expect("Error in parse json");

        let token_msg = data["Msg"].take();

        println!("{:?}", token_msg);
    }

    // pubs
    pub fn exec_command(&mut self, cmd_type: Command) -> serde_json::Value {
        let payload = json!({"cmd": cmd_type});
        let cmd = payload.to_string();
        let mut buffer = String::new();
        
        self.stream.write(cmd.as_bytes()).unwrap();
        self.stream.read_to_string(&mut buffer).unwrap();

        let raw_json: serde_json::Value = serde_json::from_str(&buffer)
            .expect("Error on parse summary");

        raw_json
    }

    pub fn summary(&mut self) -> Summary {
        let mut raw_data = self.exec_command(Command::Summary);

        serde_json::from_value::<Summary>(raw_data["SUMMARY"][0].take())
            .expect("Error on parsing summary")
    }
}

impl Worker {
    pub async fn all(db_pool: Arc<Pool<Sqlite>>) -> Result<Vec<Self>, Error> {
        sqlx::query_as!(
            Worker,
            r#"SELECT * FROM "worker""#,
        )
        .fetch_all(db_pool.as_ref())
        .await
        .map_err(|_| Error::DbError)  
    }

    pub async fn create(db_pool: Arc<Pool<Sqlite>>, data: web::Json<CreateWorker>) -> Result<Self, Error> {
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
        .map_err(|_| Error::DbError)
    }

    pub async fn delete(db_pool: Arc<Pool<Sqlite>>, id: i64) -> Result<DeleteWorker, Error> {
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
        .map_err(|_| Error::DbError)
    }
}

impl TimeStatistic {
    pub async fn create(db_pool: Arc<Pool<Sqlite>>, data: Arc<Statistic>) -> Result<TimeStatistic, Error> {
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
        .map_err(|_| Error::DbError)
    }
}