use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use super::{request::ToDoCreateRequest, response::ToDoResponse, ToDoRoute};
use crate::data::repository::todo::objects::ToDoDataCreateRequest;
use crate::data::repository::todo::ToDoRepository;
use crate::routes::response::{ApiResponse, ERROR_REQUEST_INVALID_FIELDS};
use crate::routes::validators::AccessToken;
use crate::utils::ErrorParser;
use crate::Conn;

impl ToDoRoute for Rocket<Build> {
    fn mount_todo_route(self, base_url: &str) -> Self {
        let path = format!("{}/todo", base_url);
        self.mount(path, routes![create_todo, get_todo, remove_todo])
    }
}

#[post("/create", format = "json", data = "<todo_request>")]
async fn create_todo<'a>(
    todo_request: Option<Json<ToDoCreateRequest<'a>>>,
    access_token: AccessToken,
    db: Conn,
) -> ApiResponse<'static, Json<ToDoResponse>> {
    let todo_request = if todo_request.is_some() {
        todo_request.unwrap()
    } else {
        return ApiResponse::Err(ERROR_REQUEST_INVALID_FIELDS);
    };

    let data_create = ToDoDataCreateRequest {
        user_uuid: &access_token.uuid,
        title: todo_request.title,
        description: todo_request.description,
        expires_at: todo_request.expired_at,
    };
    db.create(data_create)
        .await
        .map(|todo| Json::<ToDoResponse>(todo.into()))
        .map_err(|todo| todo.parse_error())
        .into()
}

#[get("/<uuid>", format = "json")]
async fn get_todo<'a>(
    uuid: &'a str,
    access_token: AccessToken,
    db: Conn,
) -> ApiResponse<'static, Json<ToDoResponse>> {
    db.get_todo_by_uuid(&access_token.uuid, uuid)
        .await
        .map(|todo| Json::<ToDoResponse>(todo.into()))
        .map_err(|todo| todo.parse_error())
        .into()
}

#[delete("/remove/<uuid>", format = "json")]
async fn remove_todo<'a>(
    uuid: &'a str,
    access_token: AccessToken,
    db: Conn,
) -> ApiResponse<'static, ()> {
    db.remove_todo_by_uuid(&access_token.uuid, uuid)
        .await
        .map_err(|todo| todo.parse_error())
        .into()
}
