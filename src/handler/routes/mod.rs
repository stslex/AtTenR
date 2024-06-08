use std::env;

use auth::AuthRoute;
use rocket::{Build, Rocket};
use user::UserRouter;

mod auth;
pub mod response;
mod user;

pub trait Router {
    fn mount_routes(self) -> Self;
}

impl Router for Rocket<Build> {
    fn mount_routes(self) -> Self {
        let api_version = env::var("API_VERSION").unwrap();
        let base_url = format!("/api/{}", api_version);
        self.mount_user_route(&base_url).mount_auth_route(&base_url)
    }
}
