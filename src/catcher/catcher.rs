use rocket::{catch, catchers, Build, Rocket};

use crate::routes::response::{ErrorResponse, ERROR_UNAUTHORIZED};

use super::AppCatcher;

impl AppCatcher for Rocket<Build> {
    fn mount_catcher(self) -> Self {
        self.register("/", catchers![catcher_unauthorized])
    }
}

#[catch(401)]
fn catcher_unauthorized() -> ErrorResponse<'static> {
    *ERROR_UNAUTHORIZED
}
