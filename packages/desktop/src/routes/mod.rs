pub mod index;
pub mod service_worker;
pub mod widget_token;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
  routes![service_worker::serve, service_worker::init, index::serve,]
}
