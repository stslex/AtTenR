use actix_http::{body::MessageBody, Request, Response};
use actix_service::{IntoServiceFactory, ServiceFactory};
use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::fmt;

use actix_web::{dev::AppConfig, Error, HttpServer};

use super::ServiceSertConfig;

impl<F, I, S, B> ServiceSertConfig for HttpServer<F, I, S, B>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig> + 'static,
    S::Error: Into<Error>,
    S::InitError: fmt::Debug,
    S::Response: Into<Response<B>>,
    S::Service: 'static,
    B: MessageBody + 'static,
{
    fn bind_server_ssl(self) -> Self {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("/run/secrets/key.pem", SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file("/run/secrets/cert.pem")
            .unwrap();

        let addrs_host = std::env::var("ADDRESS").expect("PORT should be set");
        let addrs_port = std::env::var("PORT").expect("ADDRESS should be set");
        let addrs = format!("{}:{}", addrs_host, addrs_port);

        self.bind_openssl(addrs, builder)
            .expect("Failed to bind ssl to address")
    }

    fn bind_simple_server(self) -> Self {
        let addrs_host = std::env::var("ADDRESS").expect("PORT should be set");
        let addrs_port = std::env::var("PORT").expect("ADDRESS should be set");
        info!("Server running on: {}:{}", addrs_host, addrs_port);
        let addrs = format!("{}:{}", addrs_host, addrs_port);

        self.bind(addrs).expect("Failed to bind ssl to address")
    }
}
