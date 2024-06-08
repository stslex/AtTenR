use objects::{UserDatabaseError, UserEntity, UserEntityCreate};
use rocket::async_trait;

mod database;
pub mod objects;
mod test;

#[async_trait]
trait UserDatabase {
    async fn create_user(&self, user: UserEntityCreate) -> Result<UserEntity, UserDatabaseError>;
    async fn get_user_by_uuid<'a>(&self, uuid: &'a str) -> Result<UserEntity, UserDatabaseError>;
    async fn get_user_by_login<'a>(&self, login: &'a str) -> Result<UserEntity, UserDatabaseError>;
}
