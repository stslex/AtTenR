use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use rocket::{Request, Response};
use serde_json::json;

pub enum ApiResponse<'a, T> {
    Ok(T),
    Err(&'a ErrorResponse<'a>),
}

impl<'r, 'o: 'r, T> Responder<'r, 'o> for ApiResponse<'r, T>
where
    T: Responder<'r, 'o>,
{
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        match self {
            ApiResponse::Ok(t) => t.respond_to(request),
            ApiResponse::Err(e) => e.respond_to(request),
        }
    }
}

impl<'a, T> Into<ApiResponse<'a, T>> for Result<T, &'a ErrorResponse<'a>> {
    fn into(self) -> ApiResponse<'a, T> {
        match self {
            Ok(t) => ApiResponse::Ok(t),
            Err(e) => ApiResponse::Err(e),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ErrorResponse<'a> {
    pub status: Status,
    pub message: &'a str,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ErrorResponse<'r> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        if let Ok(response) = Json(json!({"error": self.message})).respond_to(request) {
            Response::build_from(response)
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        } else {
            Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::JSON)
                .ok()
        }
    }
}

// common errors
pub const ERROR_UNAUTHORIZED: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::Unauthorized,
    message: "unauthorized",
};

pub const ERROR_INVALID_REQUEST: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::BadRequest,
    message: "invalid request",
};

pub const ERROR_INTERNAL_ERROR: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::InternalServerError,
    message: "internal error",
};

pub const ERROR_UUID_PARCE_ERROR: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::BadRequest,
    message: "uuid parse error",
};

pub const ERROR_REQUEST_INVALID_FIELDS: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::BadRequest,
    message: "invalid fields in request",
};

pub const ERROR_USER_NOT_FOUND: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::NotFound,
    message: "user not found",
};

pub const ERROR_NOT_FOUND: &'static ErrorResponse<'static> = &ErrorResponse {
    status: Status::NotFound,
    message: "not found",
};
