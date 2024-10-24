use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  time,
};
use windows::{
  Foundation::{EventRegistrationToken, TypedEventHandler},
  Media::Control::{
    GlobalSystemMediaTransportControlsSession as MediaSession,
    GlobalSystemMediaTransportControlsSessionManager as MediaManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as MediaProperties,
    GlobalSystemMediaTransportControlsSessionPlaybackInfo as PlaybackInfo,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as PlaybackStatus,
    GlobalSystemMediaTransportControlsSessionTimelineProperties as TimelineProperties,
    SessionsChangedEventArgs,
  },
};
use windows::Media::Control::{MediaPropertiesChangedEventArgs, PlaybackInfoChangedEventArgs};
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
  current_session:
    Arc<Mutex<Option<MediaSession>>>,
  session_changed_event_handler: TypedEventHandler<
    MediaManager,
    SessionsChangedEventArgs,
  >,
  media_properties_event_handler: TypedEventHandler<
    MediaSession,
    MediaPropertiesChangedEventArgs,
  >,
  playback_info_event_handler: TypedEventHandler<
    MediaSession,
    PlaybackInfoChangedEventArgs,
  >,
}

impl MediaProvider {
  pub fn new(config: MediaProviderConfig) -> MediaProvider {
    MediaProvider {
      _config: config,
      current_session: Arc::new(Mutex::new(None)),
      session_changed_event_handler: TypedEventHandler::new(
        MediaProvider::current_session_changed,
      ),
      media_properties_event_handler: TypedEventHandler::new(
        MediaProvider::media_properties_changed,
      ),
      playback_info_event_handler: TypedEventHandler::new(
        MediaProvider::playback_info_changed,
      ),
    }
  }

  fn playback_info_changed(
    session: &Option<MediaSession>,
    _args: &Option<PlaybackInfoChangedEventArgs>,
  ) -> windows::core::Result<()> {
    windows::core::Result::Ok(())
  }

  fn media_properties_changed(
    session: &Option<MediaSession>,
    _args: &Option<MediaPropertiesChangedEventArgs>,
  ) -> windows::core::Result<()> {
    windows::core::Result::Ok(())
  }

  fn current_session_changed(
    session_manager: &Option<
      MediaManager,
    >,
    _args: &Option<SessionsChangedEventArgs>,
  ) -> windows::core::Result<()> {
    if let Some(session_manager) = session_manager {
      let session = session_manager.GetCurrentSession()?;
      let mut current_session = self.current_session.lock().unwrap();
      *current_session = Some(session);
    }
    windows::core::Result::Ok(())
  }

  fn print_current_media_info(&self) -> anyhow::Result<()> {
    let media_properties = &self
      .current_session
      .lock()
      .unwrap()
      .TryGetMediaPropertiesAsync()?
      .get()?;
    println!("Title: {}", media_properties.Title()?);
    println!("Artist: {}", media_properties.Artist()?);
    println!("Album: {}", media_properties.AlbumTitle()?);
    println!("Album Artist: {}", media_properties.AlbumArtist()?);
    anyhow::Ok(())
  }

  fn create_session_manager(&self) -> anyhow::Result<()> {
    // SESSION MANAGER -------
    let session_manager =
      MediaManager::RequestAsync()
        .context("Failed to aquire media session manager.")?
        .get()
        .context("Failed to aquire media session manager.")?;
    println!("Session manager obtained.");

    let current_session = session_manager
      .GetCurrentSession()
      .context("Failed to aquire initial media session")?;
    println!("Initial current session obtained.");

    self.print_current_media_info()?;

    current_session
      .MediaPropertiesChanged(&self.media_properties_event_handler)?;

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

// impl_interval_provider!(MediaProvider, true);
