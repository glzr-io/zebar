use std::{path::PathBuf, sync::Arc};

use rocket::{
  fs::NamedFile,
  http::{ContentType, Cookie, CookieJar, SameSite, Status},
  request::{FromRequest, Outcome},
  response::Redirect,
  tokio::task,
  Request, State,
};

use crate::{config::Config, widget_factory::WidgetFactory};

pub fn setup_asset_server(
  config: Arc<Config>,
  widget_factory: Arc<WidgetFactory>,
) {
  task::spawn(async move {
    let rocket = rocket::build()
      .configure(rocket::Config::figment().merge(("port", 6124)))
      .manage(config)
      .manage(widget_factory)
      .mount("/", routes![sw_js, normalize_css, init, serve]);

    if let Err(err) = rocket.launch().await {
      error!("Asset server failed to start: {:?}", err);
    }
  });
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
pub fn sw_js() -> (ContentType, String) {
  let sw_path = include_str!("../resources/sw.js");
  (ContentType::JavaScript, sw_path.to_string())
}

#[get("/__zebar/normalize.css")]
pub fn normalize_css() -> (ContentType, String) {
  let normalize_css = include_str!("../resources/normalize.css");
  (ContentType::CSS, normalize_css.to_string())
}

#[rocket::get("/<path..>", rank = 100)]
pub async fn serve(
  path: Option<PathBuf>,
  token: WidgetToken,
  widget_factory: &State<Arc<WidgetFactory>>,
) -> Result<NamedFile, Status> {
  // Retrieve the widget state for the corresponding token.
  let widget = match widget_factory.widget_state_by_token(&token.0).await {
    Some(widget) => widget,
    None => return Err(Status::NotFound),
  };

  // Prevent directory traversal with "..".
  if let Some(ref p) = path {
    if p
      .components()
      .any(|component| component.as_os_str() == "..")
    {
      return Err(Status::Forbidden);
    }
  }

  // Get the widget's HTML path and determine its root directory.
  let widget_html_path = widget.html_path.clone();
  let root = widget_html_path
    .parent()
    .map(PathBuf::from)
    .unwrap_or_else(|| widget.html_path.clone());

  // Determine the final path to serve.
  let final_path = if let Some(ref p) = path {
    if p.as_os_str().is_empty() {
      // If the path is empty, use the widget's default HTML or fallback to
      // `index.html`. Did this cause vite
      let widget_filename = widget_html_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("index.html");
      root.join(widget_filename)
    } else {
      // If a path is provided serve it from the root directory.
      root.join(p)
    }
  } else {
    // Handle the case where the path is `None` explicitly, serving the
    // widget's HTML.
    let widget_filename = widget_html_path
      .file_name()
      .and_then(|name| name.to_str())
      .unwrap_or("index.html");
    root.join(widget_filename)
  };

  println!("Root: {:?} ---- Final path: {:?}", root, final_path);

  // Attempt to open and serve the requested file, currently returns html
  // `Content-Type` if not found
  NamedFile::open(final_path)
    .await
    .map_err(|_| Status::NotFound)
}

/// Token for identifying which widget is being accessed.
#[derive(Debug)]
pub struct WidgetToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WidgetToken {
  type Error = anyhow::Error;

  async fn from_request(
    request: &'r Request<'_>,
  ) -> Outcome<Self, Self::Error> {
    let token = request.cookies().get("ZEBAR_TOKEN");

    match token {
      Some(token) => {
        Outcome::Success(WidgetToken(token.value_trimmed().to_string()))
      }
      None => Outcome::Error((
        Status::Unauthorized,
        anyhow::anyhow!("Missing token for widget."),
      )),
    }
  }
}
