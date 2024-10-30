pub mod index;
pub mod service_worker;
pub mod user_agent;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
  routes![service_worker::serve, index::serve]
}
