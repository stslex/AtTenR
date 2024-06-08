#[cfg(test)]
mod tests {
    use diesel::connection;

    use crate::data::database::{
        test::database_test_utls::run_migration_get_conn,
        user::{objects::UserEntityCreate, UserDatabase},
    };

    #[tokio::test]
    async fn test_create_user() {
        let connection = run_migration_get_conn().await.unwrap();
        let check_login = "login_check";
        let check_username = "username_check";
        let check_secret = "secret_check";
        let check_user = UserEntityCreate {
            login: check_login.to_owned(),
            username: check_username.to_owned(),
            secret: check_secret.to_owned(),
        };

        let insert_user_result = connection.create_user(check_user).await;
        assert!(insert_user_result.is_ok());

        let inserted_user = insert_user_result.unwrap();
        assert_eq!(inserted_user.login, check_login.to_owned());
        assert_eq!(inserted_user.username, check_username.to_owned());
        assert_eq!(inserted_user.secret, check_secret.to_owned());
    }

    #[tokio::test]
    async fn test_get_user_by_uuid() {
        let connection = run_migration_get_conn().await.unwrap();

        let check_login = "login";
        let check_username = "username";
        let check_secret = "secret";
        let check_user = UserEntityCreate {
            login: check_login.to_owned(),
            username: check_username.to_owned(),
            secret: check_secret.to_owned(),
        };

        let insert_user_result = connection.create_user(check_user).await;
        assert!(insert_user_result.is_ok());
        let inserted_user = insert_user_result.unwrap();

        let get_user_result = connection
            .get_user_by_uuid(&inserted_user.uuid.to_string())
            .await;
        assert!(get_user_result.is_ok());

        let get_user = get_user_result.unwrap();
        assert_eq!(get_user.login, check_login.to_owned());
        assert_eq!(get_user.username, check_username.to_owned());
        assert_eq!(get_user.secret, check_secret.to_owned());
    }

    #[tokio::test]
    async fn test_get_user_by_login() {
        let connection = run_migration_get_conn().await.unwrap();

        let check_login = "login";
        let check_username = "username";
        let check_secret = "secret";
        let check_user = UserEntityCreate {
            login: check_login.to_owned(),
            username: check_username.to_owned(),
            secret: check_secret.to_owned(),
        };

        let insert_user_result = connection.create_user(check_user).await;
        assert!(insert_user_result.is_ok());
        let inserted_user = insert_user_result.unwrap();

        let get_user_result = connection.get_user_by_login(&inserted_user.login).await;
        assert!(get_user_result.is_ok());

        let get_user = get_user_result.unwrap();
        assert_eq!(get_user.login, check_login.to_owned());
        assert_eq!(get_user.username, check_username.to_owned());
        assert_eq!(get_user.secret, check_secret.to_owned());
    }
}
