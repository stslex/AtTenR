use rocket::futures::TryFutureExt;

use crate::{
    data::database::user::{objects::UserDatabaseError, UserDatabase},
    Conn,
};

use super::{
    jwt::{objects::JwtObject, JwtGenerator},
    objects::{
        UserAuthError, UserAuthResponse, UserLoginRequest, UserRegistrationError,
        UserRegistrationRequest,
    },
    AuthRepository,
};

impl AuthRepository for Conn {
    async fn registrarion<'a>(
        &self,
        user: UserRegistrationRequest<'a>,
    ) -> Result<UserAuthResponse, UserRegistrationError> {
        let user_by_login = self.get_user_by_login(user.login).await;
        match user_by_login {
            Ok(_) => return Err(UserRegistrationError::UsernameExists),
            Err(UserDatabaseError::UserNotFound) => (),
            Err(err) => return Err(err.into()),
        }

        let user_by_username = self.get_user_by_username(user.username).await;
        match user_by_username {
            Ok(_) => return Err(UserRegistrationError::UsernameExists),
            Err(UserDatabaseError::UserNotFound) => (),
            Err(err) => return Err(err.into()),
        }

        let user_created = self.create_user(user.into()).await.map_err(|err| {
            print!("Error in create user: {:?}", err);
            err.into()
        })?;

        JwtObject {
            uuid: &user_created.uuid.to_string(),
            username: &user_created.username,
        }
        .generate()
        .await
        .map(|jwt| UserAuthResponse {
            uuid: user_created.uuid,
            username: user_created.username,
            login: user_created.login,
            password: user_created.secret,
            access_token: jwt.access_token,
            refresh_token: jwt.refresh_token,
        })
        .map_err(|err| {
            println!("Error in generate jwt: {:?}", err);
            UserRegistrationError::InternalError
        })
    }

    async fn login<'a>(
        &self,
        user: UserLoginRequest<'a>,
    ) -> Result<UserAuthResponse, UserAuthError> {
        let check_user = self
            .get_user_by_login(user.login)
            .map_err(|err| {
                print!("Error in get user by login: {:?}", err);
                match err {
                    UserDatabaseError::UserNotFound => return UserAuthError::UserNotFound,
                    _ => UserAuthError::InternalError,
                }
            })
            .await?;

        if check_user.uuid.is_nil() {
            return Err(UserAuthError::UserNotFound);
        }

        if check_user.secret != user.password {
            return Err(UserAuthError::InvalidPassword);
        }

        JwtObject {
            uuid: &check_user.uuid.to_string(),
            username: &check_user.username,
        }
        .generate()
        .await
        .map(|jwt| UserAuthResponse {
            uuid: check_user.uuid,
            username: check_user.username,
            login: check_user.login,
            password: check_user.secret,
            access_token: jwt.access_token,
            refresh_token: jwt.refresh_token,
        })
        .map_err(|err| {
            println!("Error in generate jwt: {:?}", err);
            UserAuthError::InternalError
        })
    }
}
