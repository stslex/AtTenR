use std::{future::Ready, rc::Rc};

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error,
};

use crate::config::AppState;

use super::jwt::DecodeRefreshToken;

struct RefreshTokenMiddleware {
    state: web::Data<AppState<'static>>,
}

impl RefreshTokenMiddleware {
    pub fn new(state: web::Data<AppState<'static>>) -> Self {
        RefreshTokenMiddleware { state }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RefreshTokenMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RefreshTokenMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Ok(RefreshTokenMiddlewareInner {
            service: Rc::new(service),
            state: self.state.clone(),
        })
    }
}

pub struct RefreshTokenMiddlewareInner<S> {
    service: Rc<S>,
    state: web::Data<AppState<'static>>,
}

impl<S, B> Service<ServiceRequest> for RefreshTokenMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let state = self.state.clone();
        let (http_request, payload) = req.into_parts();

        let fut = async move {
            // Note: verify_jwt function needs to be implemented or imported
            // For now, we'll assume it exists in scope

            let token = match http_request.get_token() {
                Some(token) => token,
                None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
            };

            let decode_access_token = DecodeRefreshToken { token: &token };

            match decode_access_token.decode() {
                Ok(claims) => {
                    let req = ServiceRequest::from_parts(http_request, payload);
                    req.extensions_mut().insert(std::sync::Arc::new(claims));
                    service.call(req).await
                }
                Err(err) => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
            }
        };

        Box::pin(fut)
    }
}
