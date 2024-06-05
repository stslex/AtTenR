use axum::Router;

mod router;

pub trait RouterSetup {
    fn create_router(&self) -> Router;
}
