use rocket::futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use crate::data::repository::auth::objects::{UserLoginRequest, UserRegistrationRequest};
use crate::data::repository::auth::AuthRepository;
use crate::routes::auth::request::{LoginRequest, RegistrationRequest};
use crate::routes::validators::ApiKey;
use crate::routes::validators::RefreshToken;
use crate::utils::{AppHasher, ErrorParser};
use crate::Conn;

use super::response::LoginOk;
use super::validator::{FieldValidator, LoginValidatorObject, PasswordValidatorObject};
use super::AuthRoute;
use crate::routes::response::{ApiResponse, ERROR_INVALID_REQUEST};

impl AuthRoute for Rocket<Build> {
    fn mount_auth_route(self, base_url: &str) -> Self {
        let path = format!("{}/auth", base_url);
        self.mount(path, routes![login, register, refresh_token])
    }
}

#[post("/login", format = "json", data = "<login_request>")]
async fn login<'a>(
    login_request: Option<Json<LoginRequest<'a>>>,
    _api_key_validator: ApiKey,
    db: Conn,
) -> ApiResponse<'static, Json<LoginOk>> {
    let login_request = if login_request.is_none() {
        return ApiResponse::Err(ERROR_INVALID_REQUEST);
    } else {
        login_request.unwrap()
    };

    let login_hash = login_request.login.hash();
    let password_hash = login_request.password.hash();
    let user = UserLoginRequest {
        login: &login_hash.await,
        password: &password_hash.await,
    };
    db.login(user)
        .map_ok(|user| Json::<LoginOk>(user.into()))
        .map_err(|err| err.parse_error())
        .await
        .into()
}

#[post("/register", format = "json", data = "<reg_request>")]
async fn register<'a>(
    reg_request: Option<Json<RegistrationRequest<'a>>>,
    _api_key_validator: ApiKey,
    db: Conn,
) -> ApiResponse<'static, Json<LoginOk>> {
    let reg_request = if reg_request.is_none() {
        return ApiResponse::Err(ERROR_INVALID_REQUEST);
    } else {
        reg_request.unwrap()
    };

    let password_validator = PasswordValidatorObject {
        password: reg_request.password,
    };
    let password = match password_validator.validate().await {
        Ok(password) => password,
        Err(err) => return ApiResponse::Err(err.parse_error()),
    };

    let login_validator = LoginValidatorObject {
        login: reg_request.login,
    };
    let login = match login_validator.validate().await {
        Ok(login) => login,
        Err(err) => return ApiResponse::Err(err.parse_error()),
    };

    let login_hash = login.hash();
    let password_hash = password.hash();

    let user = UserRegistrationRequest {
        username: reg_request.username,
        login: &login_hash.await,
        password: &password_hash.await,
    };
    db.registrarion(user)
        .map_ok(|user| Json::<LoginOk>(user.into()))
        .map_err(|err| err.parse_error())
        .await
        .into()
}

#[post("/refresh")]
async fn refresh_token<'a>(
    refresh_token: RefreshToken,
    db: Conn,
) -> ApiResponse<'static, Json<LoginOk>> {
    let uuid = refresh_token.uuid;
    db.refresh_token(&uuid)
        .map_ok(|user| Json::<LoginOk>(user.into()))
        .map_err(|err| err.parse_error())
        .await
        .into()
}
