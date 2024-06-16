use crate::config::JWT_ACCESS_SECRET;

use super::{DecodeAccessToken, DecodeDefaultToken, JwtDecodeResult, JwtDecoder, JwtDecoderError};
use std::env;

impl<'a> JwtDecoder for DecodeAccessToken<'a> {
    fn decode(&self) -> Result<JwtDecodeResult, JwtDecoderError> {
        let env_secret = env::var(JWT_ACCESS_SECRET).map_err(|err| {
            println!("JWT_ACCESS_SECRET error: {}", err);
            JwtDecoderError::InvalidEnvSecret
        })?;
        DecodeDefaultToken {
            token: self.token,
            secret: env_secret.as_bytes(),
        }
        .decode()
    }
}
