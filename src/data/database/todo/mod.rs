use objects::{ToDoEntity, TodoDatabaseError, TodoEntityCreate};

mod database;
pub mod objects;
mod test;

pub trait ToDoDatabase {
    async fn create_todo(&self, todo: TodoEntityCreate) -> Result<ToDoEntity, TodoDatabaseError>;
    async fn get_by_uuid<'a>(&self, uuid: &'a str) -> Result<ToDoEntity, TodoDatabaseError>;
    async fn remove_by_uuid<'a>(&self, uuid: &'a str) -> Result<(), TodoDatabaseError>;
}
