// Currently windows support only
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use windows::Media::Control::{
  GlobalSystemMediaTransportControlsSessionManager,
  GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

use crate::{impl_interval_provider, providers::ProviderOutput};
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaOutput {
  pub title: String,
  pub sub_title: String,
  pub track_number: i32,
  pub artist: String,
  pub album_title: String,
  pub play_status: bool,
}

impl Default for MediaOutput {
  fn default() -> Self {
    MediaOutput {
      title: "None Playing".to_string(),
      sub_title: "None Playing".to_string(),
      track_number: 0,
      artist: "None Playing".to_string(),
      album_title: "None Playing".to_string(),
      play_status: false,
    }
  }
}

pub struct MediaProvider {
  config: MediaProviderConfig,
}

impl MediaProvider {
  pub fn new(config: MediaProviderConfig) -> MediaProvider {
    MediaProvider { config }
  }

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval
  }

  async fn run_interval(&self) -> anyhow::Result<ProviderOutput> {
    let media =
      match GlobalSystemMediaTransportControlsSessionManager::RequestAsync(
      ) {
        Ok(async_op) => match async_op.get() {
          Ok(manager) => manager,
          Err(_) => {
            return Ok(ProviderOutput::Media(MediaOutput::default()))
          }
        },
        Err(_) => {
          return Ok(ProviderOutput::Media(MediaOutput::default()))
        }
      };
    if let Ok(current_session) = media.GetCurrentSession() {
      if let Ok(media_properties) =
        current_session.TryGetMediaPropertiesAsync()?.get()
      {
        let title = media_properties
          .Title()
          .map(|hs| hs.to_string())
          .unwrap_or_else(|_| "Unknown Title".to_string());
        let sub_title = media_properties
          .Subtitle()
          .map(|hs| hs.to_string())
          .unwrap_or_else(|_| "Unknown Subtitle".to_string());
        let track_number = media_properties
          .TrackNumber()
          .map(|i| i as i32)
          .unwrap_or(0);
        let album_title = media_properties
          .AlbumTitle()
          .map(|hs| hs.to_string())
          .unwrap_or_else(|_| "Unknown Album".to_string());
        let artist = media_properties
          .Artist()
          .map(|hs| hs.to_string())
          .unwrap_or_else(|_| "Unknown Artist".to_string());

        let playback_info = current_session.GetPlaybackInfo()?;
        let play_status = matches!(
                playback_info.PlaybackStatus(),
                Ok(GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing)
            );

        return Ok(ProviderOutput::Media(MediaOutput {
          title,
          sub_title,
          track_number,
          artist,
          album_title,
          play_status,
        }));
      }
    }

    Ok(ProviderOutput::Media(MediaOutput::default()))
  }
}

impl_interval_provider!(MediaProvider, true);
