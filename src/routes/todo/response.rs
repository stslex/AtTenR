use rocket::http::Status;
use serde::Serialize;

use crate::{
    data::repository::todo::objects::{ToDoCreateError, ToDoDataGetError, ToDoDataResponse},
    routes::response::{
        ErrorResponse, ERROR_INTERNAL_ERROR, ERROR_NOT_FOUND, ERROR_USER_NOT_FOUND,
        ERROR_UUID_PARCE_ERROR,
    },
    utils::ErrorParser,
};

#[derive(Serialize)]
pub struct ToDoResponse {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub expired_at: i64,
    pub status: String,
}

impl Into<ToDoResponse> for ToDoDataResponse {
    fn into(self) -> ToDoResponse {
        ToDoResponse {
            uuid: self.uuid.to_string(),
            title: self.title,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
            expired_at: self.expires_at,
            status: self.status,
        }
    }
}

impl ErrorParser for ToDoCreateError {
    fn parse_error(&self) -> &'static ErrorResponse<'static> {
        match self {
            ToDoCreateError::UserUuidParseError => ERROR_UUID_PARCE_ERROR,
            ToDoCreateError::UserNotFound => ERROR_USER_NOT_FOUND,
            ToDoCreateError::ExpiresAtInPastError => ERROR_NOT_FOUND,
            ToDoCreateError::UnresolvedError => ERROR_INTERNAL_ERROR,
        }
    }
}

impl ErrorParser for ToDoDataGetError {
    fn parse_error(&self) -> &'static ErrorResponse<'static> {
        match self {
            ToDoDataGetError::UuidParseError => ERROR_UUID_PARCE_ERROR,
            ToDoDataGetError::NotFound => ERROR_NOT_FOUND,
            ToDoDataGetError::UnresolvedError => ERROR_INTERNAL_ERROR,
            ToDoDataGetError::UserNotMatchError => ERROR_USER_NOT_MATCH,
        }
    }
}

pub const ERROR_USER_NOT_MATCH: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::NotFound,
    message: "user not match this todo item",
};
