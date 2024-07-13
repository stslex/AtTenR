use std::fmt::{Display, Formatter};

mod access_token;
mod default_token;
mod refresh_token;
mod tests;

pub trait JwtDecoder {
    fn decode(&self) -> Result<JwtDecodeResult, JwtDecoderError>;
}

struct DecodeDefaultToken<'a> {
    pub token: &'a str,
    pub secret: &'a [u8],
}

pub struct DecodeAccessToken<'a> {
    pub token: &'a str,
}

pub struct DecodeRefreshToken<'a> {
    pub token: &'a str,
}

#[derive(Debug)]
pub struct JwtDecodeResult {
    pub uuid: String,
    pub username: String,
}

#[derive(Debug)]
pub enum JwtDecoderError {
    InvalidEnvSecret,
    InvalidSignature,
    ExpiredSignature,
    ParceError(&'static str),
}

impl Display for JwtDecoderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtDecoderError::InvalidEnvSecret => write!(f, "Invalid environment secret"),
            JwtDecoderError::InvalidSignature => write!(f, "Invalid signature"),
            JwtDecoderError::ExpiredSignature => write!(f, "Expired signature"),
            JwtDecoderError::ParceError(err) => write!(f, "Parce error: {}", err),
        }
    }
}
