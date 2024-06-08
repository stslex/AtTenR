use objects::{UserDatabaseError, UserEntity, UserEntityCreate};

mod database;
pub mod objects;

trait UserDatabase {
    async fn create_user<'a>(
        &self,
        user: UserEntityCreate<'a>,
    ) -> Result<UserEntity, UserDatabaseError>;
}
