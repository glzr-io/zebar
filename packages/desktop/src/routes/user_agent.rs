use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct UserAgent<'r> {
  pub value: Option<&'r str>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent<'r> {
  type Error = ();

  async fn from_request(
    request: &'r Request<'_>,
  ) -> Outcome<Self, Self::Error> {
    let value = request.headers().get_one("User-Agent").and_then(|ua| {
      if ua.starts_with("zebar-widget=") {
        ua.split_once('=').map(|(_, value)| value)
      } else {
        None
      }
    });

    Outcome::Success(UserAgent { value })
  }
}
