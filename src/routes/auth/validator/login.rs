use super::{FieldValidator, LoginValidatorError, LoginValidatorObject};

#[async_trait]
impl<'a> FieldValidator<String, LoginValidatorError> for LoginValidatorObject<'a> {
    async fn validate(&self) -> Result<String, LoginValidatorError> {
        // Login must be at least 4 characters long
        if self.login.len() < 4 {
            return Err(LoginValidatorError::TooShort);
        }

        // Login must be at most 32 characters long
        if self.login.len() > 32 {
            return Err(LoginValidatorError::TooLong);
        }

        // Login must not contain "admin"
        if self.login.contains("admin") || self.login.contains("login") {
            return Err(LoginValidatorError::NotSafe);
        }
        Ok(self.login.to_owned())
    }
}
