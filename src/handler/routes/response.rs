use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Result};
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
    fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
        match self {
            ApiResponse::Ok(t) => t.respond_to(request),
            ApiResponse::Err(e) => e.respond_to(request),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ErrorResponse<'a> {
    pub status: Status,
    pub message: &'a str,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ErrorResponse<'r> {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
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
