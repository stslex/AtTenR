use std::env;

use super::{
    internal_objects::{RefreshToken, TokenGenerationModel},
    objects::JwtGeneratorError,
    JwtGenerator,
};

impl<'a> JwtGenerator<String> for RefreshToken<'a> {
    async fn generate(&self) -> Result<String, JwtGeneratorError> {
        let secret = env::var("JWT_REFRESH_SECRET").map_err(|err| {
            println!("JWT_REFRESH_SECRET not found {}", err);
            JwtGeneratorError::InvalidEnvSecret
        });
        TokenGenerationModel {
            env_secret: secret?.as_bytes(),
            exp_days: REFRESH_EXP_TIME_DAYS,
            uuid: self.uuid,
            username: self.username,
        }
        .generate()
        .await
    }
}

const REFRESH_EXP_TIME_DAYS: i64 = 30;
