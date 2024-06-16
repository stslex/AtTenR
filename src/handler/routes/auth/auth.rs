use rocket::futures::{FutureExt, TryFutureExt};
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use crate::data::repository::auth::objects::{UserAuthError, UserLoginRequest};
use crate::data::repository::auth::AuthRepository;
use crate::handler::routes::auth::request::LoginRequest;
use crate::utils::{AppHasher, ErrorParser};
use crate::Conn;

use super::response::LoginOk;
use super::AuthRoute;
use crate::handler::routes::response::{ApiResponse, ERROR_INVALID_REQUEST};

impl AuthRoute for Rocket<Build> {
    fn mount_auth_route(self, base_url: &str) -> Self {
        let path = format!("{}/auth", base_url);
        self.mount(path, routes![login, register])
    }
}

#[post("/login", format = "json", data = "<login_request>")]
async fn login<'a>(
    login_request: Option<Json<LoginRequest<'a>>>,
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

#[post("/register")]
async fn register() -> &'static str {
    "Register"
}
