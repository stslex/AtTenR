use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use log::error;
use model::{GoogleTokenInfo, UserDbModel};
use uuid::Uuid;

use crate::{config::database::model::ErrorResponseData, schemas::users};

pub mod model;

pub trait UserDatabase {
    fn find_user_by_google_id(self, google_id: &str) -> Option<UserDbModel>;
    fn insert_user(self, info: GoogleTokenInfo) -> Result<UserDbModel, ErrorResponseData>;
}

impl UserDatabase for &mut PgConnection {
    fn find_user_by_google_id(self, google_id: &str) -> Option<UserDbModel> {
        users::table
            .filter(users::google_id.eq(google_id))
            .first::<UserDbModel>(self)
            .optional()
            .unwrap_or_else(|e| {
                error!("Failed to find user by google id: {}", e);
                None
            })
    }

    fn insert_user(self, info: GoogleTokenInfo) -> Result<UserDbModel, ErrorResponseData> {
        let new_user = UserDbModel {
            uuid: Uuid::new_v4(),
            google_id: info.google_id.clone(),
            email: info.email.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<UserDbModel>(self)
            .map_err(|e| {
                error!("Failed to get user by username: {}", e);
                match e {
                    diesel::result::Error::NotFound => ErrorResponseData::NotFound,
                    _ => ErrorResponseData::InternalServerError,
                }
            })
    }
}
