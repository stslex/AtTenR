use objects::{UserAuthError, UserAuthRequest, UserAuthResposne};

pub mod objects;
mod repository;

trait AuthRepository {
    async fn create_user<'a>(
        &self,
        user: UserAuthRequest<'a>,
    ) -> Result<UserAuthResposne, UserAuthError>;
}
