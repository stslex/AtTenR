use std::{future::Ready, rc::Rc};

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error,
};

use crate::config::AppState;

use super::ApiKeyParcer;

pub struct CheckApiMiddleware {
    state: web::Data<AppState<'static>>,
}

impl CheckApiMiddleware {
    pub fn new(state: web::Data<AppState<'static>>) -> Self {
        CheckApiMiddleware { state }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CheckApiMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckApMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Ok(CheckApMiddlewareInner {
            service: Rc::new(service),
            state: self.state.clone(),
        })
    }
}

pub struct CheckApMiddlewareInner<S> {
    service: Rc<S>,
    state: web::Data<AppState<'static>>,
}

impl<S, B> Service<ServiceRequest> for CheckApMiddlewareInner<S>
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

            match http_request.get_api().await {
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
