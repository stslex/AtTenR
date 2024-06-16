use super::{
    internal_objects::{AccessToken, RefreshToken},
    objects::{JwtGeneratorError, JwtObject, JwtResult},
    JwtGenerator,
};

impl<'a> JwtGenerator<JwtResult> for JwtObject<'a> {
    async fn generate(&self) -> Result<JwtResult, JwtGeneratorError> {
        let access_token: AccessToken = Into::into(*self);
        let refresh_token: RefreshToken = Into::into(*self);

        let access_await = access_token.generate();
        let refresh_await = refresh_token.generate();

        let result = JwtResult {
            access_token: access_await.await?,
            refresh_token: refresh_await.await?,
        };
        Ok(result)
    }
}
