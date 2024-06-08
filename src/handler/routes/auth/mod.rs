mod auth;
mod request;
mod response;

pub trait AuthRoute {
    fn mount_auth_route(self, base_url: &str) -> Self;
}
