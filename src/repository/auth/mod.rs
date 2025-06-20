use actix_web::web;
use model::UserResponseModel;

use crate::{
    config::database::{model::ErrorResponseData, DataPoolConnection, DbPool},
    database::user::{
        model::{GoogleTokenInfo, UserDbModel},
        UserDatabase,
    },
};
pub mod model;

pub trait AuthRepository {
    async fn auth(self, info: GoogleTokenInfo) -> Result<UserResponseModel, ErrorResponseData>;
}

impl AuthRepository for web::Data<DbPool> {
    async fn auth(self, info: GoogleTokenInfo) -> Result<UserResponseModel, ErrorResponseData> {
        self.safely_run(|conn| match conn.find_user_by_google_id(&info.google_id) {
            Some(user) => Result::Ok(user),
            None => conn.insert_user(info),
        })
        .await
        .map(|user| user.into())
    }
}

impl Into<UserResponseModel> for UserDbModel {
    fn into(self) -> UserResponseModel {
        UserResponseModel {
            uuid: self.uuid,
            email: self.email,
            created_at: self.created_at.and_utc().timestamp(),
            updated_at: self.updated_at.and_utc().timestamp(),
        }
    }
}
