use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "username")]
    pub username: String,
}
