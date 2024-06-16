use std::{collections::BTreeMap, panic::catch_unwind};

use chrono::{Duration, Utc};
use hmac::{digest::KeyInit, Hmac};
use jwt::SignWithKey;
use sha2::Sha256;

use crate::config::{JWT_PROPERY_EXP, JWT_PROPERY_USERNAME, JWT_PROPERY_UUID};

use super::{internal_objects::TokenGenerationModel, objects::JwtGeneratorError, JwtGenerator};

impl<'a> JwtGenerator<String> for TokenGenerationModel<'a> {
    async fn generate(&self) -> Result<String, JwtGeneratorError> {
        let days: Duration = catch_unwind(|| Duration::days(self.exp_days))
            .map_err(|_| JwtGeneratorError::DurationOutOfBound)?;

        let exp_time = Utc::now()
            .checked_add_signed(days)
            .ok_or(JwtGeneratorError::TimeCreationError)?
            .timestamp();

        let key: Hmac<Sha256> =
            Hmac::new_from_slice(self.env_secret).map_err(|_| JwtGeneratorError::CreateKey)?;

        let mut claims = BTreeMap::new();
        claims.insert(JWT_PROPERY_UUID, self.uuid.to_owned());
        claims.insert(JWT_PROPERY_USERNAME, self.username.to_owned());
        claims.insert(JWT_PROPERY_EXP, exp_time.to_string());

        claims
            .sign_with_key(&key)
            .map_err(|_| JwtGeneratorError::SignWithKey)
    }
}
