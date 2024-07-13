use super::{FieldValidator, PasswordValidatorError, PasswordValidatorObject};

#[async_trait]
impl<'a> FieldValidator<String, PasswordValidatorError> for PasswordValidatorObject<'a> {
    async fn validate(&self) -> Result<String, PasswordValidatorError> {
        // Password must be at least 8 characters long
        if self.password.len() < 8 {
            return Err(PasswordValidatorError::TooShort);
        }

        // Password must be at most 64 characters long
        if self.password.len() > 64 {
            return Err(PasswordValidatorError::TooLong);
        }

        // Password must not contain "password", "123456" or "qwerty"
        if self.password.contains("password")
            || self.password.contains("123456")
            || self.password.contains("qwerty")
        {
            return Err(PasswordValidatorError::NotSafe);
        }

        // Password must contain at least one number
        if self.password.chars().all(char::is_numeric) {
            return Err(PasswordValidatorError::NotSafeNumbers);
        }

        // Password must contain at least one special character
        if self.password.chars().all(char::is_alphabetic) {
            return Err(PasswordValidatorError::NotSafeChars);
        }
        Ok(self.password.to_owned())
    }
}
