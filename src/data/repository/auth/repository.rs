use crate::Conn;

use super::{
    objects::{UserAuthError, UserAuthRequest, UserAuthResposne},
    AuthRepository,
};

impl AuthRepository for Conn {
    async fn create_user<'a>(
        &self,
        user: UserAuthRequest<'a>,
    ) -> Result<UserAuthResposne, UserAuthError> {
        todo!("Implement create_user")
    }
}
