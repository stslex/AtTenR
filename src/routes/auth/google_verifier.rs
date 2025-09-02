use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

use crate::database::user::model::GoogleTokenInfo;

pub async fn verify_google_id_token<'a>(
    id_token: &'a str,
) -> Result<GoogleTokenInfo, Box<dyn Error>> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
    let header = decode_header(id_token)?;
    let kid = header.kid.ok_or("Missing kid")?;

    let jwks_url = "https://www.googleapis.com/oauth2/v3/certs";
    let jwks: Jwks = Client::new().get(jwks_url).send().await?.json().await?;

    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.kid == kid)
        .ok_or("JWK not found")?;
    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[client_id]);
    validation.set_issuer(&["https://accounts.google.com", "accounts.google.com"]);

    let token_data = decode::<Claims>(id_token, &decoding_key, &validation)?.claims;

    // 3. Доп. проверки
    if token_data.email_verified != Some(true) {
        return Err("Email not verified".into());
    }
    let email_ = token_data.email.ok_or("Email claim missing")?;

    Ok(GoogleTokenInfo {
        google_id: token_data.sub,
        email: email_,
    })
}

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    email: Option<String>,
    email_verified: Option<bool>,
    aud: String,
    iss: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
    kty: String,
    alg: String,
    #[serde(rename = "use")]
    use_: String,
}
