use actix_web::web;

use crate::routes::auth::login::login;

pub fn auth_api_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth").service(login),
        //     .route(
        //     "/login",
        //     web::post()
        //         .to(login)
        //         .wrap(CheckApiMiddleware::new(cfg.clone())),
        // ),
    );
}
