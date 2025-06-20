use actix_http::StatusCode;
use actix_web::HttpResponse;

#[derive(Debug, Clone)]
pub struct ErrorResponse<'a> {
    pub cause: &'a str,
    pub status: StatusCode,
}

impl ErrorResponse<'static> {
    pub const OTHER: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "An unknown error occurred",
        status: StatusCode::INTERNAL_SERVER_ERROR,
    };
    pub const INTERNAL_SERVER_ERROR: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "Internal server error",
        status: StatusCode::INTERNAL_SERVER_ERROR,
    };
    pub const JSON_PARSE: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "Failed to parse JSON",
        status: StatusCode::BAD_REQUEST,
    };
    pub const TOKEN_INVALID: &'static ErrorResponse<'static> = &ErrorResponse {
        cause: "Invalid token",
        status: StatusCode::UNAUTHORIZED,
    };
}

impl Into<actix_web::HttpResponse> for &'static ErrorResponse<'static> {
    fn into(self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status).body(self.cause)
    }
}
