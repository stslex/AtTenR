use rocket::http::Status;
use serde::Serialize;

use crate::{
    data::repository::auth::objects::{UserAuthError, UserAuthResponse},
    handler::routes::response::{ErrorResponse, ERROR_INTERNAL_ERROR, ERROR_UUID_PARCE_ERROR},
    utils::ErrorParser,
};

#[derive(Serialize)]
pub struct LoginOk {
    pub uuid: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

impl Into<LoginOk> for UserAuthResponse {
    fn into(self) -> LoginOk {
        LoginOk {
            uuid: self.uuid.to_string(),
            username: self.username,
            access_token: self.access_token,
            refresh_token: self.refresh_token,
        }
    }
}

impl ErrorParser for UserAuthError {
    fn parse_error(&self) -> &'static ErrorResponse<'static> {
        match self {
            UserAuthError::UserNotFound => ERROR_USER_NOT_FOUND,
            UserAuthError::InternalError => ERROR_INTERNAL_ERROR,
            UserAuthError::InvalidPassword => ERROR_INVALID_PASSWORD,
            UserAuthError::UuidParseError => ERROR_UUID_PARCE_ERROR,
        }
    }
}

pub const ERROR_USER_NOT_FOUND: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::NotFound,
    message: "user not found",
};

pub const ERROR_INVALID_PASSWORD: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::Unauthorized,
    message: "invalid password",
};

pub const ERROR_PASSWORD_INVALID_FORMAT: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::Unauthorized,
    message: "invalid password format",
};

pub const ERROR_LOGIN_INVALID_FORMAT: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::Unauthorized,
    message: "invalid login invalid format",
};

pub const ERROR_USERNAME_INVALID_FORMAT: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::Unauthorized,
    message: "invalid username invalid format",
};
