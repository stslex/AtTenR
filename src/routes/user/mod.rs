use rocket::{Build, Rocket};

use super::response::ApiResponse;

pub trait UserRouter {
    fn mount_user_route<'a>(self, base_url: &'a str) -> Self;
}

impl UserRouter for Rocket<Build> {
    fn mount_user_route<'a>(self, base_url: &'a str) -> Self {
        let path = format!("{}/user", base_url);
        self.mount(path, routes![get_user, get_user_test_name])
    }
}

#[get("/user_name")]
async fn get_user() -> ApiResponse<'static, &'static str> {
    ApiResponse::Ok("user")
}

#[get("/user_name/<name>")]
async fn get_user_test_name(name: String) -> &'static str {
    let formatted_hello = format!("{}{}", "hello ".to_owned(), name);
    Box::leak(formatted_hello.to_string().into_boxed_str())
}
