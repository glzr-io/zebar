use rocket::{
  http::Status,
  request::{FromRequest, Outcome, Request},
};

/// Asset ID for identifying which widget is being accessed.
#[derive(Debug)]
pub struct WidgetAssetId(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WidgetAssetId {
  type Error = anyhow::Error;

  async fn from_request(
    request: &'r Request<'_>,
  ) -> Outcome<Self, Self::Error> {
    let asset_id = request.cookies().get("ZEBAR_ASSET_ID");

    match asset_id {
      Some(asset_id) => Outcome::Success(WidgetAssetId(
        asset_id.value_trimmed().to_string(),
      )),
      None => Outcome::Error((
        Status::Unauthorized,
        anyhow::anyhow!("Missing asset ID for widget."),
      )),
    }
  }
}
