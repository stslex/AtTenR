use actix_http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::routes::models::{error::ErrorResponse, response::UserResponse};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginRequest {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "password")]
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RegistrationRequest {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "username")]
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    #[serde(rename = "user")]
    pub user: UserResponse,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
}

impl ErrorResponse<'static> {
    pub const EMPTY_LOGIN: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "Login cannot be empty",
        status: StatusCode::BAD_REQUEST,
    };
    pub const EMPTY_PASSWORD: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "Password cannot be empty",
        status: StatusCode::BAD_REQUEST,
    };
    pub const EMPTY_USERNAME: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "Username cannot be empty",
        status: StatusCode::BAD_REQUEST,
    };
    pub const USER_ALREADY_EXISTS: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "user already exists",
        status: StatusCode::CONFLICT,
    };
}
