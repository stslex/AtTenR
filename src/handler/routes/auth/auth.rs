use rocket::{Build, Rocket};

use super::AuthRoute;

impl AuthRoute for Rocket<Build> {
    fn mount_auth_route(self, base_url: &str) -> Self {
        let path = format!("{}/auth", base_url);
        self.mount(path, routes![login, register])
    }
}

#[post("/login")]
async fn login() -> &'static str {
    "Login"
}

#[post("/register")]
async fn register() -> &'static str {
    "Register"
}
