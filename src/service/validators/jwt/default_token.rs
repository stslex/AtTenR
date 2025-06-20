use std::collections::BTreeMap;

use chrono::Utc;
use hmac::{digest::KeyInit, Hmac};
use jwt::{Header, Token, VerifyWithKey};
use sha2::Sha256;

use crate::config::{JWT_PROPERY_EXP, JWT_PROPERY_USERNAME, JWT_PROPERY_UUID};

use super::{DecodeDefaultToken, JwtDecodeResult, JwtDecoder, JwtDecoderError};

impl<'a> JwtDecoder for DecodeDefaultToken<'a> {
    fn decode(&self) -> Result<JwtDecodeResult, JwtDecoderError> {
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(&self.secret).map_err(|_| JwtDecoderError::InvalidEnvSecret)?;

        let token: Token<Header, BTreeMap<String, String>, _> = self
            .token
            .verify_with_key(&key)
            .map_err(|_| JwtDecoderError::InvalidSignature)?;

        let claims = token.claims();

        let current_time = Utc::now().timestamp();

        let exp_time = claims
            .get(JWT_PROPERY_EXP)
            .ok_or(JwtDecoderError::ParceError(JWT_PROPERY_EXP))?
            .parse::<i64>()
            .map_err(|_| JwtDecoderError::ParceError("exp_time not a number"))?;

        if current_time > exp_time {
            return Err(JwtDecoderError::ExpiredSignature);
        }

        let uuid = claims
            .get(JWT_PROPERY_UUID)
            .ok_or(JwtDecoderError::ParceError(JWT_PROPERY_UUID))?;

        let username = claims
            .get(JWT_PROPERY_USERNAME)
            .ok_or(JwtDecoderError::ParceError(JWT_PROPERY_USERNAME))?;

        Ok(JwtDecodeResult {
            uuid: uuid.to_owned(),
            username: username.to_owned(),
        })
    }
}
