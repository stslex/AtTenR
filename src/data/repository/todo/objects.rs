use uuid::Uuid;

use crate::data::database::{
    todo::objects::{ToDoEntity, TodoDatabaseError},
    user::objects::UserDatabaseError,
};

pub struct ToDoDataCreateRequest<'a> {
    pub user_uuid: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub expires_at: i64,
}

pub struct ToDoDataResponse {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub expires_at: i64,
}

impl Into<ToDoDataResponse> for ToDoEntity {
    fn into(self) -> ToDoDataResponse {
        ToDoDataResponse {
            uuid: self.uuid,
            user_uuid: self.user_uuid,
            title: self.title,
            description: self.description,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            expires_at: self.expires_at,
        }
    }
}

pub enum ToDoDataStatus {
    Created,
    InProgress,
    Completed,
    Expired,
}

impl Into<String> for ToDoDataStatus {
    fn into(self) -> String {
        match self {
            ToDoDataStatus::Created => "created".to_string(),
            ToDoDataStatus::InProgress => "in_progress".to_string(),
            ToDoDataStatus::Completed => "completed".to_string(),
            ToDoDataStatus::Expired => "expired".to_string(),
        }
    }
}

pub enum ToDoCreateError {
    UserUuidParseError,
    UserNotFound,
    ExpiresAtInPastError,
    UnresolvedError,
}

impl Into<ToDoCreateError> for UserDatabaseError {
    fn into(self) -> ToDoCreateError {
        match self {
            UserDatabaseError::UuidParseError => ToDoCreateError::UserUuidParseError,
            UserDatabaseError::UserNotFound => ToDoCreateError::UserNotFound,
            _ => ToDoCreateError::UnresolvedError,
        }
    }
}

impl Into<ToDoCreateError> for TodoDatabaseError {
    fn into(self) -> ToDoCreateError {
        match self {
            TodoDatabaseError::UuidParseError => ToDoCreateError::UserUuidParseError,
            _ => ToDoCreateError::UnresolvedError,
        }
    }
}

pub enum ToDoDataGetError {
    UuidParseError,
    NotFound,
    UnresolvedError,
}

impl Into<ToDoDataGetError> for TodoDatabaseError {
    fn into(self) -> ToDoDataGetError {
        match self {
            TodoDatabaseError::UuidParseError => ToDoDataGetError::UuidParseError,
            TodoDatabaseError::TodoNotFound => ToDoDataGetError::NotFound,
            _ => ToDoDataGetError::UnresolvedError,
        }
    }
}
