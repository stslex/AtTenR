use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoResponse {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub expired_at: i64,
    pub status: String,
}
