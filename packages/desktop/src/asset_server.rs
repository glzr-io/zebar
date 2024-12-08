use std::{io::Cursor, path::PathBuf, sync::Arc};

use rocket::{
  fs::NamedFile,
  http::{ContentType, Cookie, CookieJar, Header, SameSite, Status},
  request::{FromRequest, Outcome},
  response::{self, Redirect, Responder, Response},
  tokio::task,
  Request, State,
};

use crate::{
  common::PathExt, config::Config, widget_factory::WidgetFactory,
};

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
  token: WidgetToken,
  widget_factory: &State<Arc<WidgetFactory>>,
) -> Option<NamedFile> {
  // Retrieve the widget state for the corresponding token.
  let widget_state = widget_factory.state_by_token(&token.0).await?;

  // Determine the final path to serve.
  let base_url = widget_state.html_path.parent().map(PathBuf::from)?;
  let asset_path = base_url
    .join(path.unwrap_or("index.html".into()))
    .to_absolute()
    .ok()?;

  // Prevent directory traversal outside of the base URL.
  if !asset_path.starts_with(&base_url) {
    return None;
  }

  // Attempt to open and serve the requested file. Currently returns HTML
  // `Content-Type` if not found.
  NamedFile::open(asset_path).await.ok()
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
