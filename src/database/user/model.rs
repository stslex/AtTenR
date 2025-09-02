use diesel::prelude::{Insertable, Queryable};

use crate::schemas::users;

#[derive(Queryable, Insertable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct UserDbModel {
    pub uuid: uuid::Uuid,
    pub google_id: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GoogleTokenInfo {
    pub google_id: String,
    pub email: String,
}
