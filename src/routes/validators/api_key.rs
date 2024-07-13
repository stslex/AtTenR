use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use super::{ApiKey, ApiKeyError, ApiKeyParcer};

#[async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.get_api() {
            Ok(api_key) => Outcome::Success(api_key),
            Err(error) => Outcome::Error((Status::Unauthorized, error)),
        }
    }
}
