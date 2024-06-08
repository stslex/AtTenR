use crate::schemas::user;
use uuid::Uuid;

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct UserEntity {
    #[diesel(column_name = "uuid")]
    pub uuid: Uuid,
    #[diesel(column_name = "login")]
    pub login: String,
    #[diesel(column_name = "secret")]
    pub secret: String,
    #[diesel(column_name = "username")]
    pub username: String,
}

#[derive(Queryable, Insertable, PartialEq, Debug)]
#[diesel(table_name = user)]
pub struct UserEntityCreate {
    #[diesel(column_name = "login")]
    pub login: String,
    #[diesel(column_name = "secret")]
    pub secret: String,
    #[diesel(column_name = "username")]
    pub username: String,
}

#[derive(Debug)]
pub enum UserDatabaseError {
    UserAlreadyExists,
    UserNotFound,
    UuidParseError,
    DatabaseError,
}
