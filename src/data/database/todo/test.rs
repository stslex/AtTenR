#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::data::database::{
        test::database_test_utls::run_migration_get_conn,
        todo::{
            objects::{TodoDatabaseError, TodoEntityCreate},
            ToDoDatabase,
        },
    };

    #[tokio::test]
    async fn test_create_todo() {
        let connection = run_migration_get_conn().await.unwrap();
        let created = chrono::Utc::now().timestamp_millis();
        let expires = created + 1000;
        let entity_check = TodoEntityCreate {
            user_uuid: Uuid::new_v4().to_owned(),
            title: "title test".to_string(),
            description: "description test".to_string(),
            status: "test status pending".to_string(),
            expires_at: expires,
            created_at: created,
            updated_at: created,
        };
        let entity_create = entity_check.clone();
        let result = connection.create_todo(entity_create).await;
        assert!(result.is_ok());

        let inserted = result.unwrap().to_owned();
        assert_eq!(entity_check.user_uuid, inserted.user_uuid);
        assert_eq!(entity_check.title, inserted.title);
        assert_eq!(entity_check.description, inserted.description);
        assert_eq!(entity_check.status, inserted.status);
        assert_eq!(entity_check.created_at, inserted.created_at);
        assert_eq!(entity_check.expires_at, inserted.expires_at);
        assert_eq!(entity_check.updated_at, inserted.updated_at);
    }

    #[tokio::test]
    async fn test_get_todo_by_uuid() {
        let connection = run_migration_get_conn().await.unwrap();
        let created = chrono::Utc::now().timestamp_millis();
        let expires = created + 1000;
        let entity_check = TodoEntityCreate {
            user_uuid: Uuid::new_v4().to_owned(),
            title: "title test".to_string(),
            description: "description test".to_string(),
            status: "test status pending".to_string(),
            expires_at: expires,
            created_at: created,
            updated_at: created,
        };
        let entity_create = entity_check.clone();
        let result = connection.create_todo(entity_create).await;
        assert!(result.is_ok());
        let inserted = result.unwrap().to_owned();

        let get_result = connection.get_by_uuid(&inserted.uuid.to_string()).await;
        assert!(get_result.is_ok());
        let getted_user = get_result.unwrap().to_owned();

        assert_eq!(entity_check.user_uuid, getted_user.user_uuid);
        assert_eq!(entity_check.title, getted_user.title);
        assert_eq!(entity_check.description, getted_user.description);
        assert_eq!(entity_check.status, getted_user.status);
        assert_eq!(entity_check.created_at, getted_user.created_at);
        assert_eq!(entity_check.expires_at, getted_user.expires_at);
        assert_eq!(entity_check.updated_at, getted_user.updated_at);

        assert_eq!(inserted.uuid, getted_user.uuid);
    }

    #[tokio::test]
    async fn test_get_todo_by_uuid_not_found() {
        let connection = run_migration_get_conn().await.unwrap();
        let get_result = connection.get_by_uuid(&Uuid::new_v4().to_string()).await;
        assert!(get_result.is_err());
        assert_eq!(get_result.err().unwrap(), TodoDatabaseError::TodoNotFound)
    }

    #[tokio::test]
    async fn test_remove_todo() {
        let connection = run_migration_get_conn().await.unwrap();
        let created = chrono::Utc::now().timestamp_millis();
        let expires = created + 1000;
        let entity_check = TodoEntityCreate {
            user_uuid: Uuid::new_v4().to_owned(),
            title: "title test".to_string(),
            description: "description test".to_string(),
            status: "test status pending".to_string(),
            expires_at: expires,
            created_at: created,
            updated_at: created,
        };
        let entity_create = entity_check.clone();
        let result = connection.create_todo(entity_create).await;
        assert!(result.is_ok());
        let inserted = result.unwrap().to_owned();

        let remove_result = connection.remove_by_uuid(&inserted.uuid.to_string()).await;
        assert!(remove_result.is_ok());

        let get_result = connection.get_by_uuid(&inserted.uuid.to_string()).await;
        assert!(get_result.is_err());
        assert_eq!(get_result.unwrap_err(), TodoDatabaseError::TodoNotFound)
    }

    #[tokio::test]
    async fn test_remove_todo_not_found() {
        let connection = run_migration_get_conn().await.unwrap();
        let remove_result = connection.remove_by_uuid(&Uuid::new_v4().to_string()).await;
        assert!(remove_result.is_err());
        assert_eq!(remove_result.unwrap_err(), TodoDatabaseError::TodoNotFound)
    }
}
