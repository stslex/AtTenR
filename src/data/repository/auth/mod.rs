use objects::{
    UserAuthError, UserAuthResponse, UserLoginRequest, UserRegistrationError,
    UserRegistrationRequest,
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
}
