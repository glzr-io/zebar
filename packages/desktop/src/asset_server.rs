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

use crate::common::{glob_util, PathExt};

/// Port for the localhost asset server.
const ASSET_SERVER_PORT: u16 = 6124;

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

  mime_types: HashMap<String, ContentType>,
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
  mime_types: &HashMap<String, String>,
) -> anyhow::Result<tauri::Url> {
  // Generate a unique token to identify requests from the widget to the
  // asset server.
  let mime_types = parse_mime_types(mime_types)?;
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
  mime_types: HashMap<String, ContentType>,
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
) -> Option<(ContentType, NamedFile)> {
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

  let extension = relative_path
    .extension()
    .and_then(|extension| extension.to_str())
    .unwrap_or_default()
    .to_ascii_lowercase();
  let content_type = token_access
    .mime_types
    .get(&extension)
    .cloned()
    .unwrap_or_else(|| match extension.as_str() {
      "ts" => ContentType::new("text", "typescript"),
      "tsx" => ContentType::new("text", "tsx"),
      "jsx" => ContentType::new("text", "jsx"),
      _ => ContentType::from_extension(&extension)
        .unwrap_or(ContentType::Binary),
    });
  NamedFile::open(absolute_path)
    .await
    .ok()
    .map(|file| (content_type, file))
}

fn parse_mime_types(
  mime_types: &HashMap<String, String>,
) -> anyhow::Result<HashMap<String, ContentType>> {
  let mut parsed = HashMap::new();
  for (extension, value) in mime_types {
    anyhow::ensure!(
      !extension.is_empty()
        && extension.bytes().all(|byte| {
          byte.is_ascii_lowercase() || byte.is_ascii_digit()
        }),
      "Invalid MIME extension '{extension}': use lowercase letters and digits without a dot."
    );
    let content_type = value.parse::<ContentType>().map_err(|err| {
      anyhow::anyhow!("Invalid MIME type for '{extension}': {err}")
    })?;
    anyhow::ensure!(
      content_type.top() != "*" && content_type.sub() != "*",
      "MIME type for '{extension}' must not contain wildcards."
    );
    parsed.insert(extension.clone(), content_type);
  }
  Ok(parsed)
}

/// Token for identifying which directory is being accessed.
#[derive(Debug)]
pub struct ServerToken(pub String);

#[cfg(test)]
mod tests {
  use rocket::local::asynchronous::Client;

  use super::*;

  #[rocket::async_test]
  async fn serves_types_without_changing_file_access() {
    let directory = std::env::temp_dir().join(Uuid::new_v4().to_string());
    std::fs::create_dir(&directory).unwrap();
    for name in [
      "test.jsx",
      "test.TSX",
      "test.ts",
      "test.js",
      "test.css",
      "test.vue",
      "test.unknown",
      "denied.txt",
    ] {
      std::fs::write(directory.join(name), "fixture body").unwrap();
    }
    let directory = directory.canonicalize_pretty().unwrap();
    let patterns = vec![
      "*.jsx",
      "*.TSX",
      "*.ts",
      "*.js",
      "*.css",
      "*.vue",
      "*.unknown",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let overrides = parse_mime_types(&HashMap::from([
      ("vue".into(), "text/plain".into()),
      ("js".into(), "application/javascript".into()),
    ]))
    .unwrap();
    let token = upsert_or_get_token(&directory, patterns, overrides).await;
    let client =
      Client::tracked(rocket::build().mount("/", routes![serve]))
        .await
        .unwrap();
    for (path, expected) in [
      ("/test.jsx", "text/jsx"),
      ("/test.TSX", "text/tsx"),
      ("/test.ts", "text/typescript"),
      ("/test.js", "application/javascript"),
      ("/test.css", "text/css; charset=utf-8"),
      ("/test.vue", "text/plain"),
      ("/test.unknown", "application/octet-stream"),
    ] {
      let response = client
        .get(path)
        .cookie(Cookie::new("ZEBAR_TOKEN", token.clone()))
        .dispatch()
        .await;
      assert_eq!(response.status(), Status::Ok, "{path}");
      assert_eq!(
        response.content_type(),
        Some(expected.parse().unwrap()),
        "{path}"
      );
      assert_eq!(response.into_string().await.unwrap(), "fixture body");
    }
    for path in ["/missing.jsx", "/denied.txt"] {
      let response = client
        .get(path)
        .cookie(Cookie::new("ZEBAR_TOKEN", token.clone()))
        .dispatch()
        .await;
      assert_eq!(response.status(), Status::NotFound, "{path}");
    }
    let other_directory = directory.join("other-pack");
    std::fs::create_dir(&other_directory).unwrap();
    std::fs::write(other_directory.join("test.jsx"), "other pack")
      .unwrap();
    let other_token = upsert_or_get_token(
      &other_directory,
      vec!["*.jsx".into()],
      HashMap::from([("jsx".into(), ContentType::Plain)]),
    )
    .await;
    let response = client
      .get("/test.jsx")
      .cookie(Cookie::new("ZEBAR_TOKEN", other_token.clone()))
      .dispatch()
      .await;
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.into_string().await.unwrap(), "other pack");
    assert!(serve(
      Some(PathBuf::from("../test.jsx")),
      ServerToken(other_token.clone())
    )
    .await
    .is_none());
    ASSET_SERVER_TOKENS.lock().await.remove(&other_token);
    assert_eq!(
      client.get("/test.jsx").dispatch().await.status(),
      Status::Unauthorized
    );
    assert_eq!(
      client
        .get("/test.jsx")
        .cookie(Cookie::new("ZEBAR_TOKEN", "invalid"))
        .dispatch()
        .await
        .status(),
      Status::NotFound
    );

    let refreshed = upsert_or_get_token(
      &directory,
      vec!["*.jsx".into()],
      HashMap::from([("jsx".into(), ContentType::Plain)]),
    )
    .await;
    assert_eq!(token, refreshed);
    let response = client
      .get("/test.jsx")
      .cookie(Cookie::new("ZEBAR_TOKEN", token.clone()))
      .dispatch()
      .await;
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(
      client
        .get("/test.js")
        .cookie(Cookie::new("ZEBAR_TOKEN", token.clone()))
        .dispatch()
        .await
        .status(),
      Status::NotFound
    );
    ASSET_SERVER_TOKENS.lock().await.remove(&token);
    std::fs::remove_dir_all(&directory).unwrap();
  }

  #[test]
  fn rejects_invalid_mime_configuration() {
    for (extension, value) in [
      (".jsx", "text/jsx"),
      ("JSX", "text/jsx"),
      ("", "text/plain"),
      ("jsx", "not a mime type"),
      ("jsx", "text/*"),
      ("jsx", "text/plain\r\nX-Injected: yes"),
    ] {
      assert!(
        parse_mime_types(&HashMap::from([(
          extension.into(),
          value.into()
        )]))
        .is_err(),
        "{extension}: {value}"
      );
    }
  }

  #[test]
  fn pack_mime_configuration_is_optional_and_preserved() {
    use crate::widget_pack::WidgetPackConfig;
    let mut value = serde_json::json!({"name":"test", "version":"1.0.0"});
    let pack: WidgetPackConfig =
      serde_json::from_value(value.clone()).unwrap();
    assert!(pack.mime_types.is_empty());
    value["mimeTypes"] = serde_json::json!({"vue":"text/plain"});
    let mut pack: WidgetPackConfig =
      serde_json::from_value(value).unwrap();
    pack.name = "renamed".into();
    let saved = serde_json::to_value(pack).unwrap();
    assert_eq!(saved["mimeTypes"]["vue"], "text/plain");
  }
}

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
