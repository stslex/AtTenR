use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserResponseModel {
    pub uuid: uuid::Uuid,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}
