use std::sync::Arc;

use rocket::{
  http::{ContentType, Cookie, CookieJar, Status},
  response::Redirect,
  State,
};
use tokio::fs;

use crate::config::Config;

#[get("/__zebar/init?<asset_id>")]
pub fn init(asset_id: String, cookies: &CookieJar<'_>) -> Redirect {
  println!("====Serving init route");

  // Create a http-only cookie with the asset ID.
  cookies.add(
    Cookie::build(("ZEBAR_ASSET_ID", asset_id))
      .http_only(true)
      .path("/"),
  );

  Redirect::to("/vanilla.html")
}

#[get("/__zebar/sw.js")]
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
