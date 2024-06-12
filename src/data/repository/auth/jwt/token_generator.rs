use std::collections::BTreeMap;

use chrono::Duration;
use hmac::{digest::KeyInit, Hmac};
use jwt::SignWithKey;
use sha2::Sha256;

use super::{internal_objects::TokenGenerationModel, objects::JwtGeneratorError, JwtGenerator};

impl<'a> JwtGenerator<String> for TokenGenerationModel<'a> {
    async fn generate(&self) -> Result<String, JwtGeneratorError> {
        println!("Generating token for user: {}", self.username);

        let days: Duration = match std::panic::catch_unwind(|| Duration::days(self.exp_days)) {
            Ok(result) => result,
            Err(_) => {
                println!("Failed to create duration / out of bound");
                return Err(JwtGeneratorError::DurationOutOfBound);
            }
        };
        let exp_time = match chrono::Utc::now().checked_add_signed(days) {
            Some(result) => result,
            None => {
                println!("Failed to add days");
                return Err(JwtGeneratorError::TimeCreationError);
            }
        }
        .timestamp();

        println!("exp_time: {}", exp_time);

        let key: Hmac<Sha256> = match Hmac::new_from_slice(self.env_secret) {
            Ok(result) => result,
            Err(_) => {
                println!("Failed to create key");
                return Err(JwtGeneratorError::CreateKey);
            }
        };

        let mut claims = BTreeMap::new();
        claims.insert("uuid", self.uuid.to_owned());
        claims.insert("username", self.username.to_owned());
        claims.insert("exp_time", exp_time.to_string());

        match claims.sign_with_key(&key) {
            Ok(result) => Ok(result),
            Err(_) => {
                println!("Failed to sign with key");
                Err(JwtGeneratorError::SignWithKey)
            }
        }
    }
}
