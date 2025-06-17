use error::ErrorResponse;
use serde::Deserialize;

use crate::utils::Validator;

pub mod error;
pub mod response;

pub trait ModelValidator<'a, D>: Validator<'a, D, ErrorResponse<'static>>
where
    D: Deserialize<'a>,
{
    fn validate(self) -> Result<Box<D>, &'static ErrorResponse<'static>>
    where
        Self: Sized;
}

impl<'a, D, T: ModelValidator<'a, D> + ?Sized> Validator<'a, D, ErrorResponse<'static>> for T
where
    D: Deserialize<'a>,
{
    fn validate(self) -> Result<Box<D>, &'static ErrorResponse<'static>>
    where
        Self: Sized,
    {
        todo!()
    }
}
