use std::env;

use auth::AuthRoute;
use rocket::{Build, Rocket};
use todo::ToDoRoute;
use user::UserRouter;

mod auth;
pub mod response;
mod todo;
mod user;
mod validators;

pub trait Router {
    fn mount_routes(self) -> Self;
}

impl Router for Rocket<Build> {
    fn mount_routes(self) -> Self {
        let api_version = env::var("API_VERSION").unwrap();
        let base_url = format!("/api/{}", api_version);
        self.mount_user_route(&base_url)
            .mount_auth_route(&base_url)
            .mount_todo_route(&base_url)
    }
}
