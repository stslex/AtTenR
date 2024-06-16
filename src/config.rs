use std::{collections::HashMap, env};

use rocket::{figment::Figment, serde::json::Value};

pub fn from_env() -> Figment {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let address = env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");

    database_config.insert("url", Value::from(database_url));
    databases.insert("diesel_postgres_pool", database_config);

    println!("Starting server from_env info");

    Figment::from(rocket::Config::default())
        .merge(("address", address))
        .merge(("port", port))
        .merge(("databases", databases))
}

pub const JWT_ACCESS_SECRET: &'static str = "JWT_ACCESS_SECRET";
pub const JWT_REFRESH_SECRET: &'static str = "JWT_REFRESH_SECRET";

pub const JWT_PROPERY_UUID: &'static str = "uuid";
pub const JWT_PROPERY_EXP: &'static str = "exp_time";
pub const JWT_PROPERY_USERNAME: &'static str = "username";
