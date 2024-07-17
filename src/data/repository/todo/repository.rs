use crate::{
    data::{
        database::{
            todo::{objects::TodoEntityCreate, ToDoDatabase},
            user::UserDatabase,
        },
        repository::todo::objects::ToDoDataStatus,
    },
    Conn,
};

use super::{
    objects::{ToDoCreateError, ToDoDataCreateRequest, ToDoDataGetError, ToDoDataResponse},
    ToDoRepository,
};

impl ToDoRepository for Conn {
    async fn create<'a>(
        &self,
        todo: ToDoDataCreateRequest<'a>,
    ) -> Result<ToDoDataResponse, ToDoCreateError> {
        let created_at_ms = chrono::Utc::now().timestamp_millis();
        if todo.expires_at < created_at_ms {
            return Err(ToDoCreateError::ExpiresAtInPastError);
        }
        let user = self
            .get_user_by_uuid(&todo.user_uuid)
            .await
            .map_err(|err| err.into())?;
        let entity_create = TodoEntityCreate {
            user_uuid: user.uuid,
            title: todo.title.to_string(),
            description: todo.description.to_string(),
            status: ToDoDataStatus::Created.into(),
            created_at: created_at_ms,
            updated_at: created_at_ms,
            expires_at: todo.expires_at,
        };
        self.create_todo(entity_create)
            .await
            .map(|entity| entity.into())
            .map_err(|err| err.into())
    }
    async fn get_todo_by_uuid<'a>(
        &self,
        uuid: &'a str,
    ) -> Result<ToDoDataResponse, ToDoDataGetError> {
        self.get_by_uuid(uuid)
            .await
            .map(|entity| entity.into())
            .map_err(|err| err.into())
    }
    async fn remove_todo_by_uuid<'a>(&self, uuid: &'a str) -> Result<(), ToDoDataGetError> {
        self.remove_by_uuid(uuid)
            .await
            .map_err(|err| err.into())
            .map(|_| ())
    }
}
