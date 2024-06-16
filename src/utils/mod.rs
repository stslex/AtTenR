mod hasher;
mod test;

#[async_trait]
pub trait AppHasher {
    async fn hash(&self) -> String;
}
