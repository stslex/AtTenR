use super::RouterSetup;
use crate::AppState;
use axum::{extract::State, Router};
use sqlx::Executor;
use std::sync::Arc;

impl RouterSetup for Arc<AppState> {
    fn create_router(&self) -> Router {
        Router::new()
            .route("/hello", axum::routing::get(route_hello))
            .route("/user", axum::routing::get(route_get_user))
            .with_state(self.to_owned())
    }
}

async fn route_hello() -> &'static str {
    "Hello, World!"
}

async fn route_get_user(State(state): State<Arc<AppState>>) -> &'static str {
    let pg_query_result = state.db.pool.execute("SELECT * FROM users").await;
    "Hello, user!"
}
