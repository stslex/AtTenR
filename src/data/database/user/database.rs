use diesel::ExpressionMethods;
use uuid::Uuid;

use super::{
    objects::{UserDatabaseError, UserEntity, UserEntityCreate},
    UserDatabase,
};
use crate::diesel::query_dsl::methods::FilterDsl;
use crate::diesel::RunQueryDsl;
use crate::{schemas::user, Conn};

#[async_trait]
impl UserDatabase for Conn {
    async fn create_user(&self, user: UserEntityCreate) -> Result<UserEntity, UserDatabaseError> {
        self.0
            .run(move |db| {
                diesel::insert_into(user::table)
                    .values(&user)
                    .get_result::<UserEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error creating user: {}", err);
                        UserDatabaseError::DatabaseError
                    })
            })
            .await
    }

    async fn get_user_by_uuid<'a>(&self, uuid: &'a str) -> Result<UserEntity, UserDatabaseError> {
        let uuid = Uuid::parse_str(uuid).map_err(|_| UserDatabaseError::UuidParseError)?;
        self.0
            .run(move |db| {
                user::table
                    .filter(user::uuid.eq(uuid))
                    .first::<UserEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error getting user: {}", err);
                        UserDatabaseError::DatabaseError
                    })
            })
            .await
    }

    async fn get_user_by_login<'a>(&self, login: &'a str) -> Result<UserEntity, UserDatabaseError> {
        let login = login.to_owned();
        self.0
            .run(move |db| {
                user::table
                    .filter(user::login.eq(login))
                    .first::<UserEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error getting user: {}", err);
                        UserDatabaseError::DatabaseError
                    })
            })
            .await
    }
}
