pub mod asset_id;
pub mod index;
pub mod service_worker;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
  routes![service_worker::serve, service_worker::init, index::serve,]
}
