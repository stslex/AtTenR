use error::ErrorResponse;
use serde::Deserialize;

pub mod error;
pub mod response;

pub trait ModelValidator<'a, D>
where
    D: Deserialize<'a>,
{
    fn validate(self) -> Result<Box<D>, &'static ErrorResponse<'static>>
    where
        Self: Sized;
}
