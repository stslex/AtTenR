use objects::{UserDatabaseError, UserEntity, UserEntityCreate};

mod database;
pub mod objects;
mod test;

pub trait UserDatabase {
    async fn create_user(&self, user: UserEntityCreate) -> Result<UserEntity, UserDatabaseError>;
    async fn get_user_by_uuid<'a>(&self, uuid: &'a str) -> Result<UserEntity, UserDatabaseError>;
    async fn get_user_by_login<'a>(&self, login: &'a str) -> Result<UserEntity, UserDatabaseError>;
    async fn get_user_by_username<'a>(
        &self,
        username: &'a str,
    ) -> Result<UserEntity, UserDatabaseError>;
}
