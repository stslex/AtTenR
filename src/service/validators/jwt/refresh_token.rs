use crate::config::JWT_REFRESH_SECRET;

use super::{DecodeDefaultToken, DecodeRefreshToken, JwtDecodeResult, JwtDecoder, JwtDecoderError};
use std::env;

impl<'a> JwtDecoder for DecodeRefreshToken<'a> {
    fn decode(&self) -> Result<JwtDecodeResult, JwtDecoderError> {
        let env_secret = env::var(JWT_REFRESH_SECRET).map_err(|err| {
            println!("JWT_REFRESH_SECRET error: {}", err);
            JwtDecoderError::InvalidEnvSecret
        })?;
        DecodeDefaultToken {
            token: self.token,
            secret: env_secret.as_bytes(),
        }
        .decode()
    }
}
