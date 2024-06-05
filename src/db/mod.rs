use sqlx::PgPool;

#[derive(Clone)]
pub struct AppDb {
    pub pool: PgPool,
}

pub async fn get_db() -> AppDb {
    AppDb {
        pool: PgPool::connect("postgres://username:password@localhost/database_name")
            .await
            .unwrap(),
    }
}
