use std::env;

use crate::data::repository::auth::jwt::objects::JwtGeneratorError;

use super::{
    internal_objects::{AccessToken, TokenGenerationModel},
    JwtGenerator,
};

impl<'a> JwtGenerator<String> for AccessToken<'a> {
    async fn generate(&self) -> Result<String, super::objects::JwtGeneratorError> {
        let secret = env::var("JWT_ACCESS_SECRET").map_err(|err| {
            println!("JWT_ACCESS_SECRET not found {}", err);
            JwtGeneratorError::InvalidEnvSecret
        });
        TokenGenerationModel {
            env_secret: secret?.as_bytes(),
            exp_days: ACCESS_EXP_TIME_DAYS,
            uuid: self.uuid,
            username: self.username,
        }
        .generate()
        .await
    }
}

const ACCESS_EXP_TIME_DAYS: i64 = 7;
