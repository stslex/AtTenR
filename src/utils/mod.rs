use serde::Deserialize;

mod hasher;
mod test;

pub trait AppHasher {
    async fn hash(&self) -> String;
}

pub trait AsyncInto<T> {
    async fn async_into(&self) -> T;
}

pub trait Validator<'a, D, E>
where
    D: Deserialize<'a>,
{
    fn validate(self) -> Result<Box<D>, &'static E>
    where
        Self: Sized;
}
