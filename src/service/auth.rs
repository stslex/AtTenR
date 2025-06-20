use actix_web::web;

use crate::routes::auth;

pub fn auth_api_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(auth::google::google_auth));
}
