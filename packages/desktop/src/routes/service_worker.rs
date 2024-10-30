use std::sync::Arc;

use rocket::{
  http::{ContentType, Status},
  State,
};
use tokio::fs;

use crate::config::Config;

#[get("/__zebar-sw.js")]
pub async fn serve(
  config: &State<Arc<Config>>,
) -> Result<(ContentType, Vec<u8>), Status> {
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
