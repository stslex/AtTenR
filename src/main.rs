use db::{run_db_migrations, DbConn};
use handler::{catcher::AppCatcher, routes::Router};
use rocket::{fairing::AdHoc, Build, Rocket};

mod config;
mod db;
mod handler;

#[rocket::launch]
async fn launch() -> Rocket<Build> {
    dotenv::dotenv().ok();

    rocket::custom(config::from_env())
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", run_db_migrations))
        .mount_catcher()
        .mount_routes()
}
