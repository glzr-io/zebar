use std::{
  collections::HashMap,
  io::Cursor,
  path::{Path, PathBuf},
  sync::LazyLock,
};

use rocket::{
  fs::NamedFile,
  http::{ContentType, Cookie, CookieJar, Header, SameSite, Status},
  request::{FromRequest, Outcome},
  response::{self, Redirect, Responder, Response},
  Request,
};
use tokio::{sync::Mutex, task};
use uuid::Uuid;

use crate::common::PathExt;

/// Port for the localhost asset server.
const ASSET_SERVER_PORT: u16 = 6124;

/// Map of tokens to their corresponding path.
static ASSET_SERVER_TOKENS: LazyLock<Mutex<HashMap<String, PathBuf>>> =
  LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn setup_asset_server() {
  task::spawn(async move {
    let rocket = rocket::build()
      .configure(
        rocket::Config::figment().merge(("port", ASSET_SERVER_PORT)),
      )
      .mount("/", routes![sw_js, normalize_css, init, serve]);

    if let Err(err) = rocket.launch().await {
      error!("Asset server failed to start: {:?}", err);
    }
  });
}

pub async fn create_init_url(
  parent_dir: &Path,
  html_path: &Path,
) -> anyhow::Result<tauri::Url> {
  // Generate a unique token to identify requests from the widget to the
  // asset server.
  let token = upsert_or_get_token(parent_dir).await;

  let redirect = format!(
    "/{}",
    html_path.strip_prefix(parent_dir)?.to_unicode_string()
  );

  let url = tauri::Url::parse_with_params(
    &format!("http://127.0.0.1:{}/__zebar/init", ASSET_SERVER_PORT),
    &[("token", &token), ("redirect", &redirect)],
  )?;

  Ok(url)
}

/// Returns an asset server token for a given directory.
///
/// If the directory does not have an existing token, a new one is
/// generated and inserted.
async fn upsert_or_get_token(directory: &Path) -> String {
  let mut asset_server_tokens = ASSET_SERVER_TOKENS.lock().await;

  // Find existing token for this path.
  let found_token = asset_server_tokens
    .iter()
    .find(|(_, path)| *path == directory)
    .map(|(token, _)| token.clone());

  found_token.unwrap_or_else(|| {
    let new_token = Uuid::new_v4().to_string();

    asset_server_tokens.insert(new_token.clone(), directory.to_path_buf());

    new_token
  })
}

#[get("/__zebar/init?<token>&<redirect>")]
pub fn init(
  token: String,
  redirect: String,
  cookies: &CookieJar<'_>,
) -> Redirect {
  // Create a http-only cookie with the widget's token.
  cookies.add(
    Cookie::build(("ZEBAR_TOKEN", token))
      .http_only(true)
      .same_site(SameSite::Strict)
      .path("/"),
  );

  Redirect::to(redirect)
}

#[get("/__zebar/sw.js")]
pub fn sw_js() -> SwResponse {
  SwResponse(include_str!("../resources/sw.js"))
}

#[derive(Debug)]
pub struct SwResponse(&'static str);

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for SwResponse {
  fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
    Response::build()
      .header(Header::new("Content-Type", "text/javascript"))
      .header(Header::new("Service-Worker-Allowed", "/"))
      .sized_body(self.0.len(), Cursor::new(self.0))
      .ok()
  }
}

#[get("/__zebar/normalize.css")]
pub fn normalize_css() -> (ContentType, &'static str) {
  (ContentType::CSS, include_str!("../resources/normalize.css"))
}

#[rocket::get("/<path..>", rank = 100)]
pub async fn serve(
  path: Option<PathBuf>,
  token: ServerToken,
) -> Option<NamedFile> {
  // Retrieve base directory for the corresponding token.
  let base_url =
    { ASSET_SERVER_TOKENS.lock().await.get(&token.0).cloned() }?;

  let asset_path = base_url
    .join(path.unwrap_or("index.html".into()))
    .canonicalize_pretty()
    .ok()?;

  // Prevent directory traversal outside of the base URL.
  if !asset_path.starts_with(&base_url) {
    return None;
  }

  // Attempt to open and serve the requested file. Currently returns HTML
  // `Content-Type` if not found.
  NamedFile::open(asset_path).await.ok()
}

/// Token for identifying which directory is being accessed.
#[derive(Debug)]
pub struct ServerToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ServerToken {
  type Error = anyhow::Error;

  async fn from_request(
    request: &'r Request<'_>,
  ) -> Outcome<Self, Self::Error> {
    let token = request.cookies().get("ZEBAR_TOKEN");

    match token {
      Some(token) => {
        Outcome::Success(ServerToken(token.value_trimmed().to_string()))
      }
      None => Outcome::Error((
        Status::Unauthorized,
        anyhow::anyhow!("Missing token for accessing directory."),
      )),
    }
  }
}
