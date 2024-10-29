use std::{
  sync::{Arc, Mutex},
  time,
};

use anyhow::Context;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
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

#[derive(Clone, Debug)]
struct EventTokens {
  playback_info_changed_token: EventRegistrationToken,
  media_properties_changed_token: EventRegistrationToken,
  timeline_properties_changed_token: EventRegistrationToken,
}

pub struct MediaProvider {
  _config: MediaProviderConfig,
  current_session: Arc<Mutex<Option<MediaSession>>>,
  event_tokens: Arc<Mutex<Option<EventTokens>>>,
  media_output: Arc<Mutex<Option<MediaOutput>>>,
}

impl MediaProvider {
  pub fn new(config: MediaProviderConfig) -> MediaProvider {
    MediaProvider {
      _config: config,
      current_session: Arc::new(Mutex::new(None)),
      event_tokens: Arc::new(Mutex::new(None)),
      media_output: Arc::new(Mutex::new(None)),
    }
  }

  fn emit_media_info(
    session: &MediaSession,
    emit_result_tx: Sender<ProviderResult>,
    media_output: &mut MediaOutput,
  ) {
    println!("Emitting media info.");
    if let Ok(new_media_output) = Self::media_output(session) {
      info!("Title: {}", media_output.title);
      info!("Artist: {}", media_output.artist);
      info!("Album: {}", media_output.album);
      info!("Album Artist: {}", media_output.album_artist);
      if new_media_output != *media_output {
        *media_output = new_media_output;
        emit_result_tx
          .try_send(Ok(ProviderOutput::Media(media_output.clone())).into())
          .expect("Media output provider failed to send.");
      }
    }
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

  fn create_session_manager(
    &self,
    emit_result_tx: Sender<ProviderResult>,
  ) -> anyhow::Result<()> {
    let session_manager = MediaManager::RequestAsync()?.get()?;
    *self.current_session.lock().unwrap() =
      session_manager.GetCurrentSession().ok();

    let tokens = Self::add_session_listeners(
      &self.current_session.lock().unwrap().as_ref().unwrap(),
      emit_result_tx.clone(),
      self.media_output.clone(),
    );
    *self.event_tokens.lock().unwrap() = tokens.ok();

    let current_session = self.current_session.clone();
    let event_tokens = self.event_tokens.clone();
    let media_output = self.media_output.clone();
    let session_changed_handler = TypedEventHandler::new(
      move |session_manager: &Option<MediaManager>, _| {
        {
          println!("Current session changed event triggered.");
          let mut current_session = current_session.lock().unwrap();
          let mut event_tokens = event_tokens.lock().unwrap();
          // let mut media_output = media_output.lock().unwrap();

          // Remove listeners from the previous session.
          Self::remove_session_listeners(
            &current_session.as_ref().unwrap(),
            event_tokens.as_ref(),
          )
          .expect("Error removing session listeners.");

          let new_session =
            MediaManager::RequestAsync()?.get()?.GetCurrentSession()?;

          let tokens = Self::add_session_listeners(
            &current_session.as_ref().unwrap(),
            emit_result_tx.clone(),
            media_output.clone(),
          );
          *event_tokens = tokens.ok();
          *current_session = Some(new_session);
        }

        windows::core::Result::Ok(())
      },
    );

    session_manager.CurrentSessionChanged(&session_changed_handler)?;

    loop {
      std::thread::sleep(time::Duration::from_secs(1));
    }
  }

  // TODO - is it better to have arc<mutex> as the params?
  fn remove_session_listeners(
    session: &MediaSession,
    mut event_tokens: Option<&EventTokens>,
  ) -> anyhow::Result<()> {
    let token = event_tokens.unwrap();
    session.RemoveMediaPropertiesChanged(
      token.media_properties_changed_token,
    )?;
    session
      .RemovePlaybackInfoChanged(token.playback_info_changed_token)?;
    session.RemoveTimelinePropertiesChanged(
      token.timeline_properties_changed_token,
    )?;
    event_tokens = None;
    Ok(())
  }

  fn add_session_listeners(
    session: &MediaSession,
    emit_result_tx: Sender<ProviderResult>,
    media_output: Arc<Mutex<Option<MediaOutput>>>,
  ) -> anyhow::Result<EventTokens> {
    let media_properties_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();
      let media_output = media_output.clone();
      TypedEventHandler::new(move |session: &Option<MediaSession>, _| {
        println!("Media properties changed event triggered.");
        let session = session
          .as_ref()
          .expect("No session available on media properties change.");
        let mut media_output = media_output.lock().unwrap();
        Self::emit_media_info(
          session,
          emit_result_tx.clone(),
          media_output.as_mut().unwrap(),
        );
        windows::core::Result::Ok(())
      })
    };

    let playback_info_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();
      let media_output = media_output.clone();
      TypedEventHandler::new(move |session: &Option<MediaSession>, _| {
        println!("Playback info changed event triggered.");
        let session = session
          .as_ref()
          .expect("No session available on playback info change.");
        let mut media_output = media_output.lock().unwrap();
        Self::emit_media_info(
          session,
          emit_result_tx.clone(),
          &mut media_output.as_mut().unwrap(),
        );
        windows::core::Result::Ok(())
      })
    };
    let timeline_properties_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();
      let media_output = media_output.clone();
      TypedEventHandler::new(move |session: &Option<MediaSession>, _| {
        println!("Timeline properties changed event triggered.");
        let session = session
          .as_ref()
          .expect("No session available on timeline properties change.");
        let mut media_output = media_output.lock().unwrap();
        Self::emit_media_info(
          session,
          emit_result_tx.clone(),
          &mut media_output.as_mut().unwrap(),
        );
        windows::core::Result::Ok(())
      })
    };

    let timeline_token = session
      .TimelinePropertiesChanged(&timeline_properties_changed_handler)?;
    let playback_token =
      session.PlaybackInfoChanged(&playback_info_changed_handler)?;
    let media_token =
      session.MediaPropertiesChanged(&media_properties_changed_handler)?;

    Ok({
      EventTokens {
        playback_info_changed_token: playback_token,
        media_properties_changed_token: media_token,
        timeline_properties_changed_token: timeline_token,
      }
    })
  }
}

#[async_trait]
impl Provider for MediaProvider {
  async fn run(&self, emit_result_tx: Sender<ProviderResult>) {
    if let Err(err) = self.create_session_manager(emit_result_tx.clone()) {
      emit_result_tx
        .send(Err(err).into())
        .await
        .expect("Error emitting media provider to frontend.");
    }
  }
}
