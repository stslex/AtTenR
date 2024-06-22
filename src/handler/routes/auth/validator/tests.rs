#[cfg(test)]
mod test_validators {
    use crate::handler::routes::auth::validator::{LoginValidatorError, LoginValidatorObject};

    use super::super::{FieldValidator, PasswordValidatorError, PasswordValidatorObject};

    #[tokio::test]
    async fn test_password_validator_success() {
        let password = "psswrd1234";
        let password_validator = PasswordValidatorObject { password };
        let result = password_validator.validate().await.unwrap();

        assert_eq!(result, password);
    }

    #[tokio::test]
    async fn test_password_validator_unsafe_password() {
        let password = "password";
        let password_validator = PasswordValidatorObject { password };
        let result = password_validator.validate().await;

        assert_eq!(result.unwrap_err(), PasswordValidatorError::NotSafe);
    }

    #[tokio::test]
    async fn test_password_validator_unsafe_qwerty() {
        let password = "qwerty1234";
        let password_validator = PasswordValidatorObject { password };
        let result = password_validator.validate().await;

        assert_eq!(result.unwrap_err(), PasswordValidatorError::NotSafe);
    }

    #[tokio::test]
    async fn test_password_validator_unsafe_1234546() {
        let password = "qwe12345678";
        let password_validator = PasswordValidatorObject { password };
        let result = password_validator.validate().await;

        assert_eq!(result.unwrap_err(), PasswordValidatorError::NotSafe);
    }

    #[tokio::test]
    async fn test_password_validator_unsafe_numbers() {
        let password = "1242526289";
        let password_validator = PasswordValidatorObject { password };
        let result = password_validator.validate().await;

        assert_eq!(result.unwrap_err(), PasswordValidatorError::NotSafeNumbers);
    }

    #[tokio::test]
    async fn test_password_validator_unsafe_chars() {
        let password = "psswdnotsafetest";
        let password_validator = PasswordValidatorObject { password };
        let result = password_validator.validate().await;

        assert_eq!(result.unwrap_err(), PasswordValidatorError::NotSafeChars);
    }

    #[tokio::test]
    async fn test_password_validator_too_short() {
        let password = "pss";
        let password_validator = PasswordValidatorObject { password };
        let result = password_validator.validate().await;

        assert_eq!(result.unwrap_err(), PasswordValidatorError::TooShort);
    }

    #[tokio::test]
    async fn test_password_validator_too_long() {
        let password = "too_long_password_to_test_validator_too_long_password_to_test_validator_too_long_password_to_test_validator_too_long_password_to_test_validator";
        let password_validator = PasswordValidatorObject { password };
        let result = password_validator.validate().await;

        assert_eq!(result.unwrap_err(), PasswordValidatorError::TooLong);
    }

    #[tokio::test]
    async fn test_login_validator_success() {
        let login = "log_valid_success";
        let login_validator = LoginValidatorObject { login };
        let result = login_validator.validate().await.unwrap();

        assert_eq!(result, login);
    }

    #[tokio::test]
    async fn test_login_validator_too_short() {
        let login = "log";
        let login_validator = LoginValidatorObject { login };
        let result = login_validator.validate().await;

        assert_eq!(result.unwrap_err(), LoginValidatorError::TooShort);
    }

    #[tokio::test]
    async fn test_login_validator_too_long() {
        let login = "too_long_login_to_test_validator_too_long_login_to_test_validator_too_long_login_to_test_validator_too_long_login_to_test_validator";
        let login_validator = LoginValidatorObject { login };
        let result = login_validator.validate().await;

        assert_eq!(result.unwrap_err(), LoginValidatorError::TooLong);
    }

    #[tokio::test]
    async fn test_login_validator_not_safe_admin() {
        let login = "admin";
        let login_validator = LoginValidatorObject { login };
        let result = login_validator.validate().await;

        assert_eq!(result.unwrap_err(), LoginValidatorError::NotSafe);
    }

    #[tokio::test]
    async fn test_login_validator_not_safe_login() {
        let login = "login";
        let login_validator = LoginValidatorObject { login };
        let result = login_validator.validate().await;

        assert_eq!(result.unwrap_err(), LoginValidatorError::NotSafe);
    }
}
