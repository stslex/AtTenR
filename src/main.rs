use db::{get_db, AppDb};
use routes::RouterSetup;
use std::sync::Arc;

mod db;
mod routes;

#[derive(Clone)]
struct AppState {
    db: AppDb,
}

#[tokio::main]
async fn main() {
    let app_db = get_db().await;
    let state = Arc::new(AppState { db: app_db });
    let router = state.create_router();

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(tcp_listener, router);
}
