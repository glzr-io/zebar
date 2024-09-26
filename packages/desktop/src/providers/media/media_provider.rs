use anyhow::Result;
use serde::{Deserialize, Serialize};
use windows::{
  Foundation::{EventRegistrationToken, TypedEventHandler},
  Media::Control::{
    GlobalSystemMediaTransportControlsSession,
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties,
  },
};

use crate::impl_interval_provider;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaOutput {
  pub title: String,
  pub artist: String,
  pub album: String,
  pub album_artist: String,
  //   pub duration: u64,
  //   pub position: u64,
  //   pub is_playing: bool,
}

pub struct MediaProvider {
  session_manager: GlobalSystemMediaTransportControlsSessionManager,
  current_session: GlobalSystemMediaTransportControlsSession,
}

impl MediaProvider {
  pub fn new() -> anyhow::Result<MediaProvider> {
    let session_manager =
      GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?
        .get()?;
    let current_session = session_manager.GetCurrentSession()?;

    let provider = MediaProvider {
      session_manager,
      current_session,
    };

    let media_props = provider.get_media_properties()?; 
    println!("{:?}", media_props.title);

    Ok(provider)
  }

  fn get_media_properties(&self) -> Result<MediaOutput> {
    let media_properties =
      self.current_session.TryGetMediaPropertiesAsync()?.get()?;
    Ok(MediaOutput {
      title: media_properties.Title()?.to_string(),
      artist: media_properties.Artist()?.to_string(),
      album: media_properties.AlbumTitle()?.to_string(),
      album_artist: media_properties.AlbumArtist()?.to_string(),
    })
  }
}

// impl_interval_provider!(MediaProvider, true);
