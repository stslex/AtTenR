use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use uuid::Uuid;

use crate::{schemas::todo, Conn};

use super::{
    objects::{ToDoEntity, TodoDatabaseError, TodoEntityCreate},
    ToDoDatabase,
};

impl ToDoDatabase for Conn {
    async fn create_todo(&self, todo: TodoEntityCreate) -> Result<ToDoEntity, TodoDatabaseError> {
        self.0
            .run(move |db| {
                diesel::insert_into(todo::table)
                    .values(&todo)
                    .get_result::<ToDoEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error todo creating: {}", err);
                        TodoDatabaseError::DatabaseError
                    })
            })
            .await
    }

    async fn get_by_uuid<'a>(&self, uuid: &'a str) -> Result<ToDoEntity, TodoDatabaseError> {
        let uuid = Uuid::parse_str(uuid).map_err(|_| TodoDatabaseError::UuidParseError)?;
        self.0
            .run(move |db| {
                todo::table
                    .filter(todo::uuid.eq(uuid))
                    .first::<ToDoEntity>(db)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => TodoDatabaseError::TodoNotFound,
                        _ => {
                            eprintln!("Error todo get_by_uuid: {}", err);
                            TodoDatabaseError::DatabaseError
                        }
                    })
            })
            .await
    }

    async fn remove_by_uuid<'a>(&self, uuid: &'a str) -> Result<(), TodoDatabaseError> {
        let uuid = Uuid::parse_str(uuid).map_err(|_| TodoDatabaseError::UuidParseError)?;
        match self
            .0
            .run(move |db| {
                diesel::delete(todo::table.filter(todo::uuid.eq(uuid)))
                    .execute(db)
                    .map_err(|err| {
                        eprintln!("Error todo remove_by_uuid: {}", err);
                        TodoDatabaseError::DatabaseError
                    })
            })
            .await?
        {
            0 => Err(TodoDatabaseError::TodoNotFound),
            _ => Ok(()),
        }
    }
}
