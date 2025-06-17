use actix_web::{web, App, HttpServer};

use config::{AppStateConfig, ServiceSertConfig};
use env_logger::Env;
use log::info;
use service::api_v1_service;
mod config;
pub mod routes;
pub mod schemas;
mod service;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .bind_app_state()
            .service(web::scope("/api").configure(api_v1_service))
    })
    .bind_simple_server()
    .run()
    .await
}
