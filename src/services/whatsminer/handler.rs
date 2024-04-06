use std::sync::Arc;
use actix_web::{HttpResponse, web};
use serde_json::json;

use crate::error::Result;
use crate::AppState;
use super::types::{Client, CreateWorker, Statistic, CheckWorker, StatisticFilter};
use super::models::{Worker, Statistic as TimeStatistic};

pub async fn all_stat(state: web::Data<AppState>) -> Result<HttpResponse> {
    let workers = Worker::all(Arc::clone(&state.db)).await?;
    let mut stats: Vec<Statistic> = Vec::with_capacity(workers.len());

    for worker in workers.iter() {
        let summary = Client::new(worker.host.clone(), worker.port.clone(), worker.name.clone(), false)
            .await?
            .summary()
            .await?;

        stats.push(Statistic { summary, worker: worker.clone()});
    }

    Ok(HttpResponse::Ok().json(json!(stats)))
}

pub async fn create(state: web::Data<AppState>, data: web::Json<CreateWorker>) -> Result<HttpResponse> {
    let worker = Worker::create(Arc::clone(&state.db), data).await?;

    Ok(HttpResponse::Ok().json(worker))
}

pub async fn delete(state: web::Data<AppState>, path: web::Path<i64>) -> Result<HttpResponse> {
    let id: i64 = path.into_inner();

    let delete_worker = Worker::delete(Arc::clone(&state.db), id).await?;

    Ok(HttpResponse::Ok().json(delete_worker))
}

pub async fn all(state: web::Data<AppState>) -> Result<HttpResponse> {
    let workers = Worker::all(Arc::clone(&state.db)).await?;

    Ok(HttpResponse::Ok().json(workers))
}

pub async fn check(_state: web::Data<AppState>, data: web::Json<CheckWorker>) -> Result<HttpResponse> {
    let mut client = Client::new(
        data.host.clone(),
        data.port.clone(),
        "Test".to_string(),
        false
    ).await?;

    let summary = client.summary().await?;

    Ok(HttpResponse::Ok().json(summary))
}

pub async fn time_statistic(state: web::Data<AppState>, path: web::Path<i64>, filter_data: web::Query<StatisticFilter>) -> Result<HttpResponse> {
    let worker_id: i64 = path.into_inner().into();

    let statistic = TimeStatistic::by_worker(Arc::clone(&state.db), worker_id, filter_data).await?;

    Ok(HttpResponse::Ok().json(statistic))
}