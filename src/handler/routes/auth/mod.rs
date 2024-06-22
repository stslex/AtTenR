mod auth;
mod request;
mod response;
mod validator;

pub trait AuthRoute {
    fn mount_auth_route(self, base_url: &str) -> Self;
}
