use super::objects::{ToDoCreateError, ToDoDataCreateRequest, ToDoDataGetError, ToDoDataResponse};
use super::ToDoRepository;
use crate::data::database::todo::{objects::TodoEntityCreate, ToDoDatabase};
use crate::data::database::user::UserDatabase;
use crate::data::repository::todo::objects::ToDoDataStatus;
use crate::Conn;

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

    async fn remove_todo_by_uuid<'a>(
        &self,
        user_uuid: &'a str,
        uuid: &'a str,
    ) -> Result<(), ToDoDataGetError> {
        let entity = self.get_by_uuid(uuid).await.map_err(|err| err.into())?;
        if entity.user_uuid.to_string() != user_uuid {
            return Err(ToDoDataGetError::UserNotMatchError);
        };
        self.remove_by_uuid(uuid)
            .await
            .map_err(|err| err.into())
            .map(|_| ())
    }

    async fn get_todo_by_uuid<'a>(
        &self,
        user_uuid: &'a str,
        uuid: &'a str,
    ) -> Result<ToDoDataResponse, ToDoDataGetError> {
        match self.get_by_uuid(uuid).await {
            Ok(entity) => {
                if entity.user_uuid.to_string() != user_uuid {
                    Err(ToDoDataGetError::UserNotMatchError)
                } else {
                    Ok(entity.into())
                }
            }
            Err(err) => Err(err.into()),
        }
    }
}
