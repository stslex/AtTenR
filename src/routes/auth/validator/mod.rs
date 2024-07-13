mod login;
mod password;
mod tests;

#[async_trait]
pub trait FieldValidator<T, Error> {
    async fn validate(&self) -> Result<T, Error>;
}

pub struct PasswordValidatorObject<'a> {
    pub password: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum PasswordValidatorError {
    TooShort,
    TooLong,
    NotSafe,
    NotSafeChars,
    NotSafeNumbers,
}

pub struct LoginValidatorObject<'a> {
    pub login: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum LoginValidatorError {
    TooShort,
    TooLong,
    NotSafe,
}
