use objects::{ToDoCreateError, ToDoDataCreateRequest, ToDoDataGetError, ToDoDataResponse};

pub mod objects;
mod repository;

pub trait ToDoRepository {
    async fn create<'a>(
        &self,
        todo: ToDoDataCreateRequest<'a>,
    ) -> Result<ToDoDataResponse, ToDoCreateError>;
    async fn get_todo_by_uuid<'a>(
        &self,
        uuid: &'a str,
    ) -> Result<ToDoDataResponse, ToDoDataGetError>;
    async fn remove_todo_by_uuid<'a>(&self, uuid: &'a str) -> Result<(), ToDoDataGetError>;
}
