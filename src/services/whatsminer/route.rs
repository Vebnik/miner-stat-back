use actix_web::web;

use crate::services::whatsminer::handler::{all, create, all_stat, delete, check};

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/asic")
            .route("/all", web::get().to(all))
            .route("/stat/all", web::get().to(all_stat))
            .route("/create", web::post().to(create))
            .route("/delete/{id}", web::delete().to(delete))
            .route("/check", web::post().to(check)),
    );
}