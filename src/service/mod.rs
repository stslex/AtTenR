use actix_web::{get, web, HttpResponse, Responder};
use auth::auth_api_service;

use crate::config::AppState;

mod auth;
mod guard;

pub fn api_v1_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            // .guard(api_key_guard())
            .configure(auth_api_service)
            .service(hello),
    );
}

#[get("")]
async fn hello<'a>(data: web::Data<AppState<'a>>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("app running {}", app_name))
}
