use rocket::http::Status;
use serde::Serialize;

use crate::{
    data::repository::auth::objects::{
        UserAuthError, UserAuthResponse, UserRefreshTokenError, UserRefreshTokenResponse,
        UserRegistrationError,
    },
    handler::routes::response::{ErrorResponse, ERROR_INTERNAL_ERROR, ERROR_UUID_PARCE_ERROR},
    utils::ErrorParser,
};

use super::validator::{LoginValidatorError, PasswordValidatorError};

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

impl Into<LoginOk> for UserRefreshTokenResponse {
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

impl ErrorParser for UserRegistrationError {
    fn parse_error(&self) -> &'static ErrorResponse<'static> {
        match self {
            UserRegistrationError::LoginExists => ERROR_USER_ALREADY_EXIST,
            UserRegistrationError::UsernameExists => ERROR_USER_ALREADY_EXIST,
            UserRegistrationError::UserAlreadyExists => ERROR_USER_ALREADY_EXIST,
            UserRegistrationError::UserNotFound => ERROR_USER_NOT_FOUND,
            UserRegistrationError::UuidParseError => ERROR_UUID_PARCE_ERROR,
            UserRegistrationError::InternalError => ERROR_INTERNAL_ERROR,
        }
    }
}

impl ErrorParser for UserRefreshTokenError {
    fn parse_error(&self) -> &'static ErrorResponse<'static> {
        match self {
            UserRefreshTokenError::UserNotFound => ERROR_USER_NOT_FOUND,
            UserRefreshTokenError::InternalError => ERROR_INTERNAL_ERROR,
            UserRefreshTokenError::UuidParseError => ERROR_UUID_PARCE_ERROR,
        }
    }
}

impl ErrorParser for PasswordValidatorError {
    fn parse_error(&self) -> &'static ErrorResponse<'static> {
        match self {
            PasswordValidatorError::TooShort => ERROR_REGISTRATION_PASS_TOO_SHORT,
            PasswordValidatorError::TooLong => ERROR_REGISTRATION_PASS_TOO_LONG,
            PasswordValidatorError::NotSafe => ERROR_REGISTRATION_PASS_NOT_SAFE,
            PasswordValidatorError::NotSafeChars => ERROR_REGISTRATION_PASS_NOT_SAFE,
            PasswordValidatorError::NotSafeNumbers => ERROR_REGISTRATION_PASS_NOT_SAFE,
        }
    }
}

impl ErrorParser for LoginValidatorError {
    fn parse_error(&self) -> &'static ErrorResponse<'static> {
        match self {
            LoginValidatorError::TooShort => ERROR_REGISTRATION_LOGIN_TOO_SHORT,
            LoginValidatorError::TooLong => ERROR_REGISTRATION_LOGIN_TOO_LONG,
            LoginValidatorError::NotSafe => ERROR_REGISTRATION_LOGIN_NOT_SAFE,
        }
    }
}

pub const ERROR_USER_ALREADY_EXIST: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::Conflict,
    message: "user already exists",
};

pub const ERROR_USER_NOT_FOUND: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::NotFound,
    message: "user not found",
};

pub const ERROR_INVALID_PASSWORD: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::Unauthorized,
    message: "invalid password",
};

pub const ERROR_REGISTRATION_PASS_TOO_SHORT: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::LengthRequired,
    message: "password is too short",
};

pub const ERROR_REGISTRATION_PASS_TOO_LONG: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::LengthRequired,
    message: "password is too long",
};

pub const ERROR_REGISTRATION_PASS_NOT_SAFE: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::LengthRequired,
    message: "password is not safe",
};

pub const ERROR_REGISTRATION_LOGIN_TOO_SHORT: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::LengthRequired,
    message: "password is too short",
};

pub const ERROR_REGISTRATION_LOGIN_TOO_LONG: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::LengthRequired,
    message: "password is too long",
};

pub const ERROR_REGISTRATION_LOGIN_NOT_SAFE: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::LengthRequired,
    message: "password is not safe",
};
