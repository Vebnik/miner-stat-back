use actix_web::Scope;

use crate::services::whatsminer::route::scoped_config as whatsminer_service;

/// Init service routes
pub fn init_api_service(scope: Scope) -> Scope {
    scope
        .configure(whatsminer_service)
}