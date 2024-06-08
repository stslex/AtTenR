use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

#[database("diesel_postgres_pool")]
pub struct DbConn(pub diesel::PgConnection);

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    println!("Running database migrations");
    let conn = DbConn::get_one(&rocket).await.expect("database connection");
    conn.run(|conn| {
        // Run pending migrations
        match conn.run_pending_migrations(MIGRATIONS) {
            Ok(m_version) => {
                println!("Database migrations ran successfully: {:?}", m_version);
                Ok(rocket)
            }
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        }
    })
    .await
    .unwrap()
}
