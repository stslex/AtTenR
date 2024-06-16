#[cfg(test)]
mod test_decoder {
    use std::{collections::BTreeMap, env};

    use hmac::{digest::KeyInit, Hmac};
    use jwt::SignWithKey;
    use sha2::Sha256;

    use crate::{
        config::{JWT_ACCESS_SECRET, JWT_REFRESH_SECRET},
        handler::routes::validators::jwt::{DecodeAccessToken, JwtDecoder},
    };

    const EXPECTED_UUID: &str = "expected_uuid";
    const EXPECTED_USERNAME: &str = "expected_username";
    const SECRET_TEST: &str = "secret_test";

    #[test]
    fn test_decode_access() {
        env::set_var(JWT_ACCESS_SECRET, SECRET_TEST);
        let result = DecodeAccessToken {
            token: &get_test_token(),
        }
        .decode();

        assert!(result.is_ok());
        let decoder_result = result.unwrap();
        assert_eq!(decoder_result.uuid, EXPECTED_UUID);
        assert_eq!(decoder_result.username, EXPECTED_USERNAME);
    }

    #[test]
    fn test_decode_refresh() {
        env::set_var(JWT_REFRESH_SECRET, SECRET_TEST);

        let result = DecodeAccessToken {
            token: &get_test_token(),
        }
        .decode();

        assert!(result.is_ok());
        let decoder_result = result.unwrap();
        assert_eq!(decoder_result.uuid, EXPECTED_UUID);
        assert_eq!(decoder_result.username, EXPECTED_USERNAME);
    }

    fn get_test_token() -> String {
        let exp_time = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .expect("Failed to add days")
            .timestamp();

        let key: Hmac<Sha256> =
            Hmac::new_from_slice(SECRET_TEST.as_bytes()).expect("Failed to create key");
        let mut claims = BTreeMap::new();
        claims.insert("uuid", EXPECTED_UUID.to_string());
        claims.insert("username", EXPECTED_USERNAME.to_string());
        claims.insert("exp_time", exp_time.to_string());

        claims.sign_with_key(&key).ok().unwrap().to_owned()
    }
}
