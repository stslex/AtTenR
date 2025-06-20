use actix_web::web;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use log::error;

use super::{model::ErrorResponseData, DataPool, DataPoolConnection, DbPool};

impl DataPool<PgConnection, ErrorResponseData> for web::Data<DbPool> {
    fn safe_get(
        self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ErrorResponseData> {
        self.get().map_err(|e| {
            error!("Failed to get connection from pool: {}", e);
            ErrorResponseData::InternalServerError
        })
    }
}

impl<R> DataPoolConnection<PgConnection, R, ErrorResponseData> for web::Data<DbPool>
where
    R: std::marker::Send + 'static,
{
    async fn safely_run<A>(self, action: A) -> Result<R, ErrorResponseData>
    where
        A: FnOnce(&mut PgConnection) -> Result<R, ErrorResponseData> + Send + 'static,
    {
        web::block(move || {
            let mut conn = self.safe_get()?;
            action(&mut conn)
        })
        .await
        .map_err(|e| {
            error!("Blocking error: {}", e);
            ErrorResponseData::BlockingError
        })?
    }
}
