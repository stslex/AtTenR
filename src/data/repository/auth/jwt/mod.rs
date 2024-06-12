use objects::JwtGeneratorError;

mod access_generator;
mod generator;
mod internal_objects;
pub mod objects;
mod refresh_generator;
mod token_generator;

pub trait JwtGenerator<T> {
    async fn generate(&self) -> Result<T, JwtGeneratorError>;
}
