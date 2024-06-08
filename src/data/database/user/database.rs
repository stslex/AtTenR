use crate::db::DbConn;

use super::{
    objects::{UserDatabaseError, UserEntity, UserEntityCreate},
    UserDatabase,
};

impl UserDatabase for DbConn {
    async fn create_user<'a>(
        &self,
        user: UserEntityCreate<'a>,
    ) -> Result<UserEntity, UserDatabaseError> {
        todo!("Implement create_user")
    }
}
