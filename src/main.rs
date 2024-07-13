#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use catcher::AppCatcher;
use db::run_db_migrations;
use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_sync_db_pools::database;
use routes::Router;

mod catcher;
mod config;
mod data;
pub mod db;
mod routes;
mod schemas;
pub mod utils;

#[rocket::launch]
async fn launch() -> Rocket<Build> {
    dotenv::dotenv().ok();

    rocket::custom(config::from_env())
        .attach(Conn::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", run_db_migrations))
        .mount_catcher()
        .mount_routes()
}

#[database("diesel_postgres_pool")]
pub struct Conn(pub diesel::PgConnection);
