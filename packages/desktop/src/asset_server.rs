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

use crate::{
  common::{glob_util, PathExt},
  widget_pack::AssetMimeType,
};

/// Port for the localhost asset server.
const ASSET_SERVER_PORT: u16 = 6124;

/// MIME types that Rocket's extension table does not consistently identify as browser JavaScript.
const DEFAULT_MIME_TYPES: &[(&str, &str)] = &[
  ("cjs", "text/javascript"),
  ("cts", "text/javascript"),
  ("jsx", "text/javascript"),
  ("mjs", "text/javascript"),
  ("mts", "text/javascript"),
  ("ts", "text/javascript"),
  ("tsx", "text/javascript"),
  ("wasm", "application/wasm"),
  ("webmanifest", "application/manifest+json"),
];

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

  /// Custom MIME types configured for the widget pack.
  mime_types: Vec<AssetMimeType>,
}

pub async fn setup_asset_server() -> anyhow::Result<()> {
  // Disable Rocket's built-in signal handling (`ctrlc` and Unix
  // `signals`). By default Rocket listens for SIGTERM/SIGINT and
  // performs a graceful HTTP-server shutdown — but because the signal
  // handler *replaces* the OS default, the rest of the process (Tauri)
  // stays alive. This leaves a zombie process that holds the
  // single-instance lock, preventing a fresh Zebar from starting.
  //
  // With signal handling disabled, SIGTERM/SIGINT fall through to the
  // OS default (process termination), which is the desired behavior
  // when Zebar is managed by an external process (e.g. GlazeWM).
  let figment = rocket::Config::figment()
    .merge(("port", ASSET_SERVER_PORT))
    .merge(("shutdown.ctrlc", false));

  #[cfg(unix)]
  let figment = figment.merge(("shutdown.signals", Vec::<String>::new()));

  let rocket = rocket::build()
    .configure(figment)
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
  mime_types: Vec<AssetMimeType>,
) -> anyhow::Result<tauri::Url> {
  // Generate a unique token to identify requests from the widget to the
  // asset server.
  let token =
    upsert_or_get_token(parent_dir, file_patterns, mime_types).await;

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
async fn upsert_or_get_token(
  directory: &Path,
  file_patterns: Vec<String>,
  mime_types: Vec<AssetMimeType>,
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
      access.mime_types = mime_types;
    }

    token
  } else {
    let new_token = Uuid::new_v4().to_string();

    asset_server_tokens.insert(
      new_token.clone(),
      TokenAccess {
        base_dir: directory.to_path_buf(),
        file_patterns,
        mime_types,
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
) -> Option<AssetResponse> {
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

  let content_type =
    content_type_for_asset(&relative_path, &token_access.mime_types);

  // NamedFile supplies the standard extension-based type. Override it only when this project
  // includes a predefined browser type or an explicit mapping in its widget-pack config.
  let file = NamedFile::open(absolute_path).await.ok()?;
  Some(AssetResponse { file, content_type })
}

fn content_type_for_asset(
  path: &Path,
  mime_types: &[AssetMimeType],
) -> Option<ContentType> {
  let extension = path.extension()?.to_str()?.to_ascii_lowercase();
  let configured = mime_types.iter().rev().find(|entry| {
    entry
      .extension
      .trim_start_matches('.')
      .eq_ignore_ascii_case(&extension)
  });
  let mime_type = configured
    .map(|entry| entry.content_type.as_str())
    .or_else(|| {
      DEFAULT_MIME_TYPES
        .iter()
        .find(|(known_extension, _)| *known_extension == extension)
        .map(|(_, mime_type)| *mime_type)
    })?;

  ContentType::parse_flexible(mime_type)
}

pub struct AssetResponse {
  file: NamedFile,
  content_type: Option<ContentType>,
}

impl<'r> Responder<'r, 'static> for AssetResponse {
  fn respond_to(
    self,
    request: &'r Request<'_>,
  ) -> response::Result<'static> {
    let mut response = self.file.respond_to(request)?;
    if let Some(content_type) = self.content_type {
      response.set_header(content_type);
    }
    Ok(response)
  }
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

#[cfg(test)]
mod tests {
  use super::*;
  use rocket::{http::Cookie, local::asynchronous::Client};

  #[test]
  fn adds_browser_mime_types_for_script_and_webassembly_assets() {
    let empty = [];

    assert_eq!(
      content_type_for_asset(Path::new("widget.TSX"), &empty)
        .map(|content_type| content_type.to_string())
        .as_deref(),
      Some("text/javascript")
    );
    assert_eq!(
      content_type_for_asset(Path::new("widget.wasm"), &empty)
        .map(|content_type| content_type.to_string())
        .as_deref(),
      Some("application/wasm")
    );
  }

  #[test]
  fn custom_mime_types_override_defaults_and_accept_extensions_with_a_dot()
  {
    let mime_types = vec![AssetMimeType {
      extension: ".TS".to_string(),
      content_type: "application/x-typescript".to_string(),
    }];

    assert_eq!(
      content_type_for_asset(Path::new("widget.ts"), &mime_types)
        .map(|content_type| content_type.to_string())
        .as_deref(),
      Some("application/x-typescript")
    );
  }

  #[tokio::test]
  async fn serves_configured_content_type_for_custom_file_extensions() {
    let base_dir = std::env::temp_dir()
      .join(format!("zebar-asset-mime-{}", Uuid::new_v4()));
    tokio::fs::create_dir_all(&base_dir).await.unwrap();
    tokio::fs::write(base_dir.join("widget.custom"), "asset body")
      .await
      .unwrap();

    let token = Uuid::new_v4().to_string();
    ASSET_SERVER_TOKENS.lock().await.insert(
      token.clone(),
      TokenAccess {
        base_dir: base_dir.clone(),
        file_patterns: vec!["widget.custom".to_string()],
        mime_types: vec![AssetMimeType {
          extension: "custom".to_string(),
          content_type: "text/x-widget".to_string(),
        }],
      },
    );

    let client =
      Client::tracked(rocket::build().mount("/", routes![serve]))
        .await
        .unwrap();
    let response = client
      .get("/widget.custom")
      .cookie(Cookie::new("ZEBAR_TOKEN", token.clone()))
      .dispatch()
      .await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
      response.content_type().unwrap().to_string(),
      "text/x-widget"
    );
    assert_eq!(
      response.into_string().await.as_deref(),
      Some("asset body")
    );

    ASSET_SERVER_TOKENS.lock().await.remove(&token);
    tokio::fs::remove_dir_all(base_dir).await.unwrap();
  }
}
