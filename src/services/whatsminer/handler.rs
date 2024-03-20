use std::sync::Arc;
use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

use crate::AppState;
use super::types::{Client, CreateWorker, Statistic, CheckWorker};
use super::models::Worker;

pub async fn all_stat(state: web::Data<AppState>) -> impl Responder {
    let workers = Worker::all(Arc::clone(&state.db)).await
        .map_err(|_|  HttpResponse::ImATeapot().json(json!({"error": "A some error"})))
        .expect("A some db error");
    
    let stats: Vec<Statistic> = workers
        .iter()
        .map(|el| {
            let summary = Client::new(el.host.clone(), el.port.clone(), el.name.clone(), false).unwrap().summary();
            Statistic { summary, worker: el.clone() }
        })
        .collect();

    HttpResponse::Ok().json(stats)
}

pub async fn create(state: web::Data<AppState>, data: web::Json<CreateWorker>) -> impl Responder {
    match Worker::create(Arc::clone(&state.db), data).await {
        Ok(res) => HttpResponse::Created().json(res),
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::ImATeapot().json(json!({"error": "A some error"}))
        },
    }
}

pub async fn delete(state: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let id: i64 = path.into_inner();

    match Worker::delete(Arc::clone(&state.db), id).await {
        Ok(res) => HttpResponse::Created().json(res),
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::ImATeapot().json(json!({"error": "A some error"}))
        },
    }
}

pub async fn all(state: web::Data<AppState>) -> impl Responder {
    match Worker::all(Arc::clone(&state.db)).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::ImATeapot().json(json!({"error": "A some error"}))
        },
    }
}

pub async fn check(_state: web::Data<AppState>, data: web::Json<CheckWorker>) -> impl Responder {
    match Client::new(data.host.clone(), data.port.clone(), "Test".to_string(), false) {
        Ok(mut clinet) => {
            let summary = clinet.summary();
            HttpResponse::Ok().json(summary)
        },
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::ImATeapot().json(json!({"error": "A some error"}))
        },
    }
}