use uuid::Uuid;

use crate::schemas::todo;

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct ToDoEntity {
    #[diesel(column_name = "uuid")]
    pub uuid: Uuid,
    #[diesel(column_name = "user_uuid")]
    pub user_uuid: Uuid,
    #[diesel(column_name = "title")]
    pub title: String,
    #[diesel(column_name = "description")]
    pub description: String,
    #[diesel(column_name = "status")]
    pub status: String,
    #[diesel(column_name = "created_at")]
    pub created_at: i64,
    #[diesel(column_name = "updated_at")]
    pub updated_at: i64,
    #[diesel(column_name = "expires_at")]
    pub expires_at: i64,
}

#[derive(Queryable, Insertable, PartialEq, Debug, Clone)]
#[diesel(table_name = todo)]
pub struct TodoEntityCreate {
    #[diesel(column_name = "user_uuid")]
    pub user_uuid: Uuid,
    #[diesel(column_name = "title")]
    pub title: String,
    #[diesel(column_name = "description")]
    pub description: String,
    #[diesel(column_name = "status")]
    pub status: String,
    #[diesel(column_name = "created_at")]
    pub created_at: i64,
    #[diesel(column_name = "updated_at")]
    pub updated_at: i64,
    #[diesel(column_name = "expires_at")]
    pub expires_at: i64,
}

#[derive(Debug, PartialEq)]
pub enum TodoDatabaseError {
    TodoNotFound,
    UuidParseError,
    DatabaseError,
}
