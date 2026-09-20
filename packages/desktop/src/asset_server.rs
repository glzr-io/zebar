use std::{
  collections::HashMap,
  io::Cursor,
  net::TcpListener,
  path::{Path, PathBuf},
  sync::{LazyLock, OnceLock},
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

use crate::common::{glob_util, PathExt};

/// Preferred port for the localhost asset server.
const DEFAULT_ASSET_SERVER_PORT: u16 = 6124;

/// Port that the localhost asset server is listening on.
///
/// Resolved at startup rather than fixed, since a port is machine-wide and
/// a second logged-in user would otherwise fail to bind.
static ASSET_SERVER_PORT: OnceLock<u16> = OnceLock::new();

/// Port of the running asset server.
pub fn asset_server_port() -> u16 {
  *ASSET_SERVER_PORT
    .get()
    .unwrap_or(&DEFAULT_ASSET_SERVER_PORT)
}

/// Map of tokens to their corresponding path and file patterns.
static ASSET_SERVER_TOKENS: LazyLock<Mutex<HashMap<String, TokenAccess>>> =
  LazyLock::new(|| Mutex::new(HashMap::new()));

/// Access information for a given token.
#[derive(Clone, Debug)]
struct TokenAccess {
  /// Base directory for the token.
  base_dir: PathBuf,

  /// File patterns for accessible files.
  file_patterns: Vec<String>,
}

/// Picks the port for the asset server.
///
/// Sticks to [`DEFAULT_ASSET_SERVER_PORT`] whenever it's free, and asks
/// the OS for any free port when it isn't, which is the case for every
/// logged-in user after the first.
///
/// Rocket needs the port up front, so the listener used to claim it is
/// dropped again before Rocket binds. Nothing else is expected to grab a
/// localhost port in that window.
fn pick_port() -> anyhow::Result<u16> {
  let listener =
    TcpListener::bind(("127.0.0.1", DEFAULT_ASSET_SERVER_PORT))
      .or_else(|_| TcpListener::bind("127.0.0.1:0"))?;

  Ok(listener.local_addr()?.port())
}

pub async fn setup_asset_server() -> anyhow::Result<()> {
  let port = pick_port()?;
  let _ = ASSET_SERVER_PORT.set(port);

  let rocket = rocket::build()
    .configure(rocket::Config::figment().merge(("port", port)))
    .mount("/", routes![sw_js, normalize_css, init, serve]);

  // Test if the server can start (this doesn't block).
  let rocket = rocket.ignite().await.map_err(|err| {
    anyhow::anyhow!("Asset server failed to initialize: {:?}", err)
  })?;

  // Now launch it in the background.
  task::spawn(async move {
    if let Err(err) = rocket.launch().await {
      error!("Asset server failed during runtime: {:?}", err);
    }
  });

  Ok(())
}

pub async fn create_init_url(
  parent_dir: &Path,
  html_path: &Path,
  file_patterns: Vec<String>,
) -> anyhow::Result<tauri::Url> {
  // Generate a unique token to identify requests from the widget to the
  // asset server.
  let token = upsert_or_get_token(parent_dir, file_patterns).await;

  let redirect = format!(
    "/{}",
    html_path.strip_prefix(parent_dir)?.to_unicode_string()
  );

  let url = tauri::Url::parse_with_params(
    &format!("http://127.0.0.1:{}/__zebar/init", asset_server_port()),
    &[("token", &token), ("redirect", &redirect)],
  )?;

  Ok(url)
}

/// Returns an asset server token for a given directory.
///
/// If the directory does not have an existing token, a new one is
/// generated and inserted.
async fn upsert_or_get_token(
  directory: &Path,
  file_patterns: Vec<String>,
) -> String {
  let mut asset_server_tokens = ASSET_SERVER_TOKENS.lock().await;

  // Find existing token for this path.
  let found_token = asset_server_tokens
    .iter()
    .find(|(_, token)| token.base_dir == directory)
    .map(|(token, _)| token.clone());

  if let Some(token) = found_token {
    // Update the file patterns for the existing token.
    if let Some(access) = asset_server_tokens.get_mut(&token) {
      access.file_patterns = file_patterns;
    }

    token
  } else {
    let new_token = Uuid::new_v4().to_string();

    asset_server_tokens.insert(
      new_token.clone(),
      TokenAccess {
        base_dir: directory.to_path_buf(),
        file_patterns,
      },
    );

    new_token
  }
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
  // Retrieve access information for the corresponding token.
  let token_access =
    { ASSET_SERVER_TOKENS.lock().await.get(&token.0).cloned() }?;

  let relative_path = path.unwrap_or("index.html".into());
  let absolute_path = token_access
    .base_dir
    .join(relative_path.clone())
    .canonicalize_pretty()
    .ok()?;

  // Allow access if:
  // - The asset path is within the base directory.
  // - The asset path matches any of the file patterns of the widget pack.
  if !absolute_path.starts_with(&token_access.base_dir)
    || !glob_util::is_match(&relative_path, &token_access.file_patterns)
      .ok()?
  {
    tracing::warn!(
      "Asset path {} is inaccessable with token {:?}.",
      absolute_path.display(),
      token_access
    );

    return None;
  }

  // Attempt to open and serve the requested file. Currently returns HTML
  // `Content-Type` if not found.
  NamedFile::open(absolute_path).await.ok()
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
