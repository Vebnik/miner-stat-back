use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, self};
use actix_web::web;

use crate::error::Error;
use super::types::CreateWorker;

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
pub struct DeleteWorker {
    pub id: i64,
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