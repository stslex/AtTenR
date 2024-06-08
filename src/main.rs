#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use db::run_db_migrations;
use handler::{catcher::AppCatcher, routes::Router};
use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_sync_db_pools::database;

mod config;
mod data;
pub mod db;
mod handler;
mod schemas;

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
