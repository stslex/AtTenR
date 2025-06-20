use actix_web::{post, web, HttpResponse};

use crate::{
    config::database::{model::ErrorResponseData, DbPool},
    repository::auth::AuthRepository,
    routes::auth::{google_verifier::verify_google_id_token, models::GoogleAuthRequest},
};

#[post("/auth/google")]
pub async fn google_auth(
    pool: web::Data<DbPool>,
    data: web::Json<GoogleAuthRequest>,
) -> actix_web::Result<HttpResponse> {
    let id_token = data.id_token.clone();

    let token_info = match verify_google_id_token(&id_token).await {
        Ok(model) => model,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };

    match pool.auth(token_info).await {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(err) => match err {
            ErrorResponseData::InternalServerError | ErrorResponseData::NotFound => {
                Ok(HttpResponse::InternalServerError().finish())
            }
            _ => Ok(HttpResponse::Unauthorized().finish()),
        },
    }
}
