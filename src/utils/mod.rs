use crate::handler::routes::response::ErrorResponse;

mod hasher;
mod test;

#[async_trait]
pub trait AppHasher {
    async fn hash(&self) -> String;
}

#[async_trait]
pub trait AsyncInto<T> {
    async fn async_into(&self) -> T;
}

pub trait ErrorParser {
    fn parse_error(&self) -> &'static ErrorResponse<'static>;
}
