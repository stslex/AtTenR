use objects::{
    UserAuthError, UserAuthResponse, UserLoginRequest, UserRefreshTokenError,
    UserRefreshTokenResponse, UserRegistrationError, UserRegistrationRequest,
};

mod jwt;
pub mod objects;
mod repository;

pub trait AuthRepository {
    async fn registrarion<'a>(
        &self,
        user: UserRegistrationRequest<'a>,
    ) -> Result<UserAuthResponse, UserRegistrationError>;
    async fn login<'a>(
        &self,
        user: UserLoginRequest<'a>,
    ) -> Result<UserAuthResponse, UserAuthError>;
    async fn refresh_token<'a>(
        &self,
        uuid: &'a str,
    ) -> Result<UserRefreshTokenResponse, UserRefreshTokenError>;
}
