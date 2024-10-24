use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  time,
};
use tracing::info;
use windows::{
  Foundation::{EventRegistrationToken, TypedEventHandler},
  Media::Control::{
    CurrentSessionChangedEventArgs,
    GlobalSystemMediaTransportControlsSession as MediaSession,
    GlobalSystemMediaTransportControlsSessionManager as MediaManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as MediaProperties,
    GlobalSystemMediaTransportControlsSessionPlaybackInfo as PlaybackInfo,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as PlaybackStatus,
    GlobalSystemMediaTransportControlsSessionTimelineProperties as TimelineProperties,
    MediaPropertiesChangedEventArgs, PlaybackInfoChangedEventArgs,
  },
};

use crate::{
  impl_interval_provider,
  providers::{Provider, ProviderOutput, ProviderResult},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaProviderConfig {}

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
  _config: MediaProviderConfig,
  current_session: Arc<Mutex<Option<MediaSession>>>,
}

impl MediaProvider {
  pub fn new(config: MediaProviderConfig) -> MediaProvider {
    MediaProvider {
      _config: config,
      current_session: Arc::new(Mutex::new(None)),
    }
  }

  fn print_current_media_info(session: &MediaSession) {
    if let Ok(media_output) = Self::media_output(session) {
      info!("Title: {}", media_output.title);
      info!("Artist: {}", media_output.artist);
      info!("Album: {}", media_output.album);
      info!("Album Artist: {}", media_output.album_artist);
    }

    // TODO: Emit to frontend client via channel.
  }

  fn media_output(session: &MediaSession) -> anyhow::Result<MediaOutput> {
    let media_properties = session.TryGetMediaPropertiesAsync()?.get()?;

    Ok(MediaOutput {
      title: media_properties.Title()?.to_string(),
      artist: media_properties.Artist()?.to_string(),
      album: media_properties.AlbumTitle()?.to_string(),
      album_artist: media_properties.AlbumArtist()?.to_string(),
    })
  }

  // OnSessionChanged -> (fires when switching focus from netflix to spot//
  //   Set MediaPropertiesHandler (fires when song changes) <- interact
  // with this to go next song/prev song   Set PlaybackInfoHandler
  // (fires on tick basically) <- interact with this to scrub
  // forward/backwards
  fn create_session_manager(&self) -> anyhow::Result<()> {
    let session_manager = MediaManager::RequestAsync()?.get()?;
    println!("Session manager obtained.");

    let mut current_session = session_manager.GetCurrentSession()?;
    Self::add_session_listeners(&current_session);

    let session_changed_handler = TypedEventHandler::new(
      |session_manager: &Option<MediaManager>, _| {
        let current_session =
          MediaManager::RequestAsync()?.get()?.GetCurrentSession()?;

        Self::add_session_listeners(&current_session);

        // self.current_session = Arccurrent_session;
        MediaProvider::print_current_media_info(&current_session);

        windows::core::Result::Ok(())
      },
    );

    session_manager.CurrentSessionChanged(&session_changed_handler)?;

    loop {
      std::thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
  }

  fn add_session_listeners(session: &MediaSession) -> anyhow::Result<()> {
    let media_properties_changed_handler =
      TypedEventHandler::new(move |session: &Option<MediaSession>, _| {
        println!("Media properties changed event triggered.");
        let session = session
          .as_ref()
          .expect("No session available on media properties change.");
        MediaProvider::print_current_media_info(session);
        windows::core::Result::Ok(())
      });

    let playback_info_changed_handler =
      TypedEventHandler::new(move |session: &Option<MediaSession>, _| {
        println!("Playback info changed event triggered.");
        let session = session
          .as_ref()
          .expect("No session available on playback info change.");
        MediaProvider::print_current_media_info(session);
        windows::core::Result::Ok(())
      });

      let timeline_properties_changed_handler = TypedEventHandler::new(
        move |session: &Option<MediaSession>, _| {
          println!("Timeline properties changed event triggered.");
          let session = session
            .as_ref()
            .expect("No session available on timeline properties change.");
          MediaProvider::print_current_media_info(session);
          windows::core::Result::Ok(())
        },
      );

    session.TimelinePropertiesChanged(&timeline_properties_changed_handler)?;
    session.PlaybackInfoChanged(&playback_info_changed_handler)?;
    session.MediaPropertiesChanged(&media_properties_changed_handler)?;

    Ok(())
  }
}

#[async_trait]
impl Provider for MediaProvider {
  async fn run(&self, emit_result_tx: Sender<ProviderResult>) {
    if let Err(err) = self.create_session_manager() {
      emit_result_tx.send(Err(err).into()).await;
    }
  }
}
