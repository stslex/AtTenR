pub mod access_token_middleware;
pub mod api_middleware;
mod jwt;
mod parcer;
pub mod refresh_token_middleware;

trait TokenParser {
    fn get_token(&self) -> Option<String>;
}

trait ApiKeyParcer {
    fn get_api(&self) -> Result<ApiKey, ApiKeyError>;
}

pub struct AccessToken {
    pub uuid: String,
    pub username: String,
}

pub struct RefreshToken {
    pub uuid: String,
    pub username: String,
}

pub struct ApiKey {
    pub key: String,
}

#[derive(Debug)]
pub enum ApiKeyError {
    InvalidApiKey,
    MissingApiKey,
}

#[derive(Debug)]
pub enum AccessTokenError {
    InvalidToken,
    InvalidApiKey,
}
