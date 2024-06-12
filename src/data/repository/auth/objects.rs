use crate::data::database::user::objects::{UserDatabaseError, UserEntityCreate};

pub struct UserRegistrationRequest<'a> {
    pub username: &'a str,
    pub login: &'a str,
    pub password: &'a str,
}

pub struct UserLoginRequest<'a> {
    pub login: &'a str,
    pub password: &'a str,
}

impl<'a> Into<UserEntityCreate> for UserRegistrationRequest<'a> {
    fn into(self) -> UserEntityCreate {
        UserEntityCreate {
            username: self.username.to_owned(),
            login: self.login.to_owned(),
            secret: self.password.to_owned(),
        }
    }
}

pub struct UserAuthResponse {
    pub uuid: uuid::Uuid,
    pub username: String,
    pub login: String,
    pub password: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub enum UserRegistrationError {
    LoginExists,
    UsernameExists,
    UserAlreadyExists,
    UserNotFound,
    UuidParseError,
    InternalError,
}

pub enum UserAuthError {
    UserNotFound,
    InvalidPassword,
    UuidParseError,
    InternalError,
}

impl Into<UserRegistrationError> for UserDatabaseError {
    fn into(self) -> UserRegistrationError {
        match self {
            UserDatabaseError::UserAlreadyExists => UserRegistrationError::UserAlreadyExists,
            UserDatabaseError::UserNotFound => UserRegistrationError::UserNotFound,
            UserDatabaseError::UuidParseError => UserRegistrationError::UuidParseError,
            UserDatabaseError::DatabaseError => UserRegistrationError::InternalError,
        }
    }
}
