use diesel::{
    r2d2::{self, ConnectionManager, PooledConnection, R2D2Connection},
    PgConnection,
};

mod migration;
pub mod model;
pub mod pool;
mod utils;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

trait DbMigration {
    fn run_migrations(&self) -> Self;
}

pub trait DataPool<C, E>
where
    C: R2D2Connection + 'static,
{
    fn safe_get(self) -> Result<PooledConnection<ConnectionManager<C>>, E>;
}

pub trait DataPoolConnection<P, R, E>
where
    P: R2D2Connection + 'static,
    R: std::marker::Send + 'static,
{
    async fn safely_run<A>(self, action: A) -> Result<R, E>
    where
        A: FnOnce(&mut P) -> Result<R, E> + Send + 'static;
}
