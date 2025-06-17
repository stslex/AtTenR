use actix_web::{get, web, HttpResponse, Responder};
use auth::auth_api_service;

use crate::config::AppState;

mod auth;
// pub mod validators;;

pub fn api_v1_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").configure(auth_api_service).service(hello));
}

#[get("")]
async fn hello<'a>(data: web::Data<AppState<'a>>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("app running {}", app_name))
}
