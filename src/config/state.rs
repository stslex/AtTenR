use actix_service::ServiceFactory;
use actix_web::{dev::ServiceRequest, web, App};

use super::{database::pool::create_db_pool, AppState, AppStateConfig};

impl<T> AppStateConfig for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn bind_app_state(self) -> Self {
        self.app_data(web::Data::new(create_db_pool().clone()))
            .app_data(web::Data::new(AppState {
                app_name: "Cinematch",
            }))
    }
    #[cfg(test)]
    fn bind_app_state_for_tests(self) -> Self {
        use super::database::pool::create_test_db_pool;

        self.app_data(web::Data::new(create_test_db_pool().clone()))
            .app_data(web::Data::new(AppState {
                app_name: "Cinematch_test",
            }))
    }
}
