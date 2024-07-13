mod access_token;
mod api_key;
mod jwt;
mod parcer;
mod refresh_token;

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
