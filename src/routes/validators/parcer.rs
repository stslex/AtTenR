use std::env;

use rocket::Request;

use super::{
    jwt::JwtDecodeResult, AccessToken, ApiKey, ApiKeyError, ApiKeyParcer, RefreshToken, TokenParser,
};

const AUTH_HEADER: &str = "Authorization";
const BEARER_PREFIX: &str = "Bearer ";

impl<'r> TokenParser for Request<'r> {
    fn get_token(&self) -> Option<String> {
        self.headers()
            .get_one(AUTH_HEADER)
            .map(|token| token.replace(BEARER_PREFIX, ""))
    }
}

const API_KEY_HEADER: &str = "X-Api-Key";
const API_KEY_ENV_VAR: &str = "API_KEY";

impl<'a> ApiKeyParcer for Request<'a> {
    fn get_api(&self) -> Result<ApiKey, ApiKeyError> {
        let key = self
            .headers()
            .get_one(API_KEY_HEADER)
            .ok_or(ApiKeyError::MissingApiKey)?
            .to_owned();
        let expected_key = env::var(API_KEY_ENV_VAR).unwrap();
        if key == expected_key {
            Ok(ApiKey {
                key: key.to_owned(),
            })
        } else {
            Err(ApiKeyError::InvalidApiKey)
        }
    }
}

impl Into<AccessToken> for JwtDecodeResult {
    fn into(self) -> AccessToken {
        AccessToken {
            uuid: self.uuid,
            username: self.username,
        }
    }
}

impl Into<RefreshToken> for JwtDecodeResult {
    fn into(self) -> RefreshToken {
        RefreshToken {
            uuid: self.uuid,
            username: self.username,
        }
    }
}
