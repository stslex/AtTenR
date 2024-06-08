use rocket::{Build, Rocket};

use super::response::ApiResponse;

pub trait UserRouter {
    fn mount_user_route<'a>(self, base_url: &'a str) -> Self;
}

impl UserRouter for Rocket<Build> {
    fn mount_user_route<'a>(self, base_url: &'a str) -> Self {
        let path = format!("{}/user", base_url);
        self.mount(path, routes![get_user])
    }
}

#[get("/user_name")]
async fn get_user() -> ApiResponse<'static, &'static str> {
    ApiResponse::Ok("user")
}
