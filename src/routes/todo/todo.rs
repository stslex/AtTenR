use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use super::{request::ToDoCreateRequest, response::ToDoResponse, ToDoRoute};
use crate::routes::response::ApiResponse;
use crate::routes::validators::RefreshToken;
use crate::Conn;

impl ToDoRoute for Rocket<Build> {
    fn mount_todo_route(self, base_url: &str) -> Self {
        let path = format!("{}/todo", base_url);
        self.mount(path, routes![create_todo])
    }
}

#[post("/create", format = "json", data = "<todo_request>")]
async fn create_todo<'a>(
    todo_request: Option<Json<ToDoCreateRequest<'a>>>,
    refresh_token: RefreshToken,
    db: Conn,
) -> ApiResponse<'static, Json<ToDoResponse>> {
    todo!("Implement create_todo")
}
