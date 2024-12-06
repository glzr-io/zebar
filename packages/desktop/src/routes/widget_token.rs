use rocket::{
  http::Status,
  request::{FromRequest, Outcome, Request},
};

/// Token for identifying which widget is being accessed.
#[derive(Debug)]
pub struct WidgetToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WidgetToken {
  type Error = anyhow::Error;

  async fn from_request(
    request: &'r Request<'_>,
  ) -> Outcome<Self, Self::Error> {
    let token = request.headers().get_one("X-ZEBAR-TOKEN");

    match token {
      Some(token) => Outcome::Success(WidgetToken(token.to_string())),
      None => Outcome::Error((
        Status::Unauthorized,
        anyhow::anyhow!("Missing X-ZEBAR-TOKEN header."),
      )),
    }
  }
}
