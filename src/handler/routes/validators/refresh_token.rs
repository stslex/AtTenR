use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use super::{
    jwt::{DecodeRefreshToken, JwtDecoder},
    AccessTokenError, ApiKeyParcer, RefreshToken, TokenParser,
};

#[async_trait]
impl<'r> FromRequest<'r> for RefreshToken {
    type Error = AccessTokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if request.get_api().is_err() {
            return Outcome::Error((Status::Unauthorized, AccessTokenError::InvalidApiKey));
        }

        let token = match request.get_token() {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, AccessTokenError::InvalidToken)),
        };

        let decode_access_token = DecodeRefreshToken { token: &token };

        match decode_access_token.decode() {
            Ok(claims) => Outcome::Success(claims.into()),
            Err(err) => {
                println!("Invalid access token: {}", err);
                Outcome::Error((Status::Unauthorized, AccessTokenError::InvalidToken))
            }
        }
    }
}
