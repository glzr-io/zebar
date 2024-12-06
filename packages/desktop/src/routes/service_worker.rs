use std::sync::Arc;

use rocket::{
  http::{ContentType, Cookie, CookieJar, Status},
  response::Redirect,
  State,
};
use tokio::fs;

use crate::config::Config;

#[get("/init/<widget_token>")]
pub fn init(widget_token: String, cookies: &CookieJar<'_>) -> Redirect {
  println!("====Serving init route");

  // Create a secure, HttpOnly cookie with the widget token
  let cookie = Cookie::build(("widget_token", widget_token))
    .http_only(true)
    .secure(false)
    .path("/")
    .finish();

  cookies.add(cookie);

  Redirect::to("/vanilla.html")
}

#[get("/__zebar-sw.js")]
pub async fn serve(
  config: &State<Arc<Config>>,
) -> Result<(ContentType, Vec<u8>), Status> {
  println!("====Serving service worker");
  let sw_path = match &config.service_worker_path {
    Some(path) => path,
    None => {
      return Err(Status::NotFound);
    }
  };

  match fs::read(&sw_path).await {
    Ok(content) => Ok((ContentType::JavaScript, content)),
    Err(_) => Err(Status::NotFound),
  }
}
