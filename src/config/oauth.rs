use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    AuthUrl, Client, ClientId, ClientSecret, EmptyExtraTokenFields, EndpointNotSet, EndpointSet,
    RedirectUrl, RevocationErrorResponseType, RevocationUrl, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
};

fn create_oauth_client() -> Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
    EndpointSet,
> {
    let google_client_id =
        ClientId::new(dotenvy::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID."));
    let google_client_secret = ClientSecret::new(
        dotenvy::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET."),
    );
    let app_url = dotenvy::var("APP_URL").expect("Missing APP_URL.");

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string())
        .expect("Invalid token endpoint URL");

    let redirect_url = RedirectUrl::new(format!("{}/auth/google/callback", app_url))
        .expect("Invalid redirect URL");

    // Создаем OAuth2 клиент, который будет использоваться во всех хендлерах
    //  google_client_id,
    // Some(google_client_secret),
    // auth_url,
    // Some(token_url),
    // let oauth_client = BasicClient::new(google_client_id).set_redirect_uri(redirect_url);
    let revocation_url = RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
        .expect("Invalid revocation endpoint URL");

    BasicClient::new(google_client_id)
        .set_client_secret(google_client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url)
        .set_revocation_url(revocation_url)
}
