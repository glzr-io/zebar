use std::{path::PathBuf, sync::Arc};

use rocket::{
  fs::NamedFile,
  http::{ContentType, Status},
  request::{FromRequest, Outcome, Request},
  State,
};
use tokio::fs;

use super::user_agent::UserAgent;
use crate::widget_factory::WidgetFactory;

#[rocket::get("/<path..>", rank = 100)]
pub async fn serve(
  path: Option<PathBuf>, // Optional path from the route.
  user_agent: UserAgent<'_>,
  widget_factory: &State<Arc<WidgetFactory>>,
) -> Result<NamedFile, Status> {
  // Validate the User-Agent header.
  if user_agent.value.is_none() {
    println!("Bad User-Agent: {:?}", user_agent.value);
    return Err(Status::NotFound);
  }

  // Retrieve the widget state using the User-Agent.
  let widget = match widget_factory
    .widget_state_by_id(&user_agent.value.unwrap())
    .await
  {
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
