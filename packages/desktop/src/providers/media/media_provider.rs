use std::{
  sync::{Arc, Mutex},
  time,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use windows::{
  Foundation::{EventRegistrationToken, TypedEventHandler},
  Media::Control::{
    GlobalSystemMediaTransportControlsSession as MediaSession,
    GlobalSystemMediaTransportControlsSessionManager as MediaManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as MediaPlaybackStatus,
    // GlobalSystemMediaTransportControlsSessionMediaProperties as
    // MediaProperties,
    // GlobalSystemMediaTransportControlsSessionPlaybackInfo as
    // PlaybackInfo,
    // GlobalSystemMediaTransportControlsSessionTimelineProperties as
    // TimelineProperties, MediaPropertiesChangedEventArgs,
    // PlaybackInfoChangedEventArgs,
  },
};

use crate::providers::{Provider, ProviderOutput, ProviderResult};

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
  pub track_number: u32,
  pub start_time: u64,
  pub end_time: u64,
  pub position: u64,
  pub is_playing: bool,
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
}

impl MediaProvider {
  pub fn new(config: MediaProviderConfig) -> MediaProvider {
    MediaProvider {
      _config: config,
      current_session: Arc::new(Mutex::new(None)),
      event_tokens: Arc::new(Mutex::new(None)),
    }
  }

  fn emit_media_info(
    session: &MediaSession,
    emit_result_tx: Sender<ProviderResult>,
  ) {
    if let Ok(media_output) = Self::media_output(session) {
      println!("media_output: {:?}", media_output);
      emit_result_tx
        .try_send(Ok(ProviderOutput::Media(media_output)).into())
        .expect("Errror emitting media info.");
    }
  }

  fn media_output(session: &MediaSession) -> anyhow::Result<MediaOutput> {
    let media_properties = session.TryGetMediaPropertiesAsync()?.get()?;
    let timeline_properties = session.GetTimelineProperties()?;
    let playback_info = session.GetPlaybackInfo()?;
    let is_playing = matches!(
      playback_info.PlaybackStatus(),
      Ok(MediaPlaybackStatus::Playing)
    );
    let start_time =
      timeline_properties.StartTime()?.Duration as u64 / 10_000_000;
    let end_time =
      timeline_properties.EndTime()?.Duration as u64 / 10_000_000;
    let position =
      timeline_properties.Position()?.Duration as u64 / 10_000_000;
    Ok(MediaOutput {
      title: media_properties.Title()?.to_string(),
      artist: media_properties.Artist()?.to_string(),
      album: media_properties.AlbumTitle()?.to_string(),
      album_artist: media_properties.AlbumArtist()?.to_string(),
      track_number: media_properties.TrackNumber()? as u32,
      start_time,
      end_time,
      position,
      is_playing,
    })
  }

  fn create_session_manager(
    &self,
    emit_result_tx: Sender<ProviderResult>,
  ) -> anyhow::Result<()> {
    // Find the current GSMTC session & add listeners.
    let session_manager = MediaManager::RequestAsync()?.get()?;
    println!("Media Session manager obtained.");
    *self.current_session.lock().unwrap() =
      session_manager.GetCurrentSession().ok();

    match Self::add_session_listeners(
      &self.current_session.lock().unwrap().as_ref().unwrap(),
      emit_result_tx.clone(),
    ) {
      Ok(tokens) => {
        *self.event_tokens.lock().unwrap() = Some(tokens);
      }
      Err(err) => {
        eprintln!("Error adding media session listeners: {:?}", err);
      }
    }

    // Clean up & rebind listeners when session changes.
    let current_session = self.current_session.clone();
    let event_tokens = self.event_tokens.clone();
    let session_changed_handler = TypedEventHandler::new(
      move |session_manager: &Option<MediaManager>, _| {
        {
          // Remove listeners from the previous session.
          let mut current_session = current_session.lock().unwrap();
          let mut event_tokens = event_tokens.lock().unwrap();
          if let Err(err) = Self::remove_session_listeners(
            &current_session.as_ref().unwrap(),
            event_tokens.as_ref(),
          ) {
            eprintln!("Error removing media session listeners: {:?}", err);
          }

          // Set up new session.
          let new_session =
            MediaManager::RequestAsync()?.get()?.GetCurrentSession()?;

          match Self::add_session_listeners(
            &new_session,
            emit_result_tx.clone(),
          ) {
            Ok(tokens) => {
              *event_tokens = Some(tokens);
            }
            Err(err) => {
              eprintln!("Error adding media session listeners: {:?}", err);
            }
          }

          Self::emit_media_info(&new_session, emit_result_tx.clone());
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

  fn remove_session_listeners(
    session: &MediaSession,
    event_tokens: Option<&EventTokens>,
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
    Ok(())
  }

  fn add_session_listeners(
    session: &MediaSession,
    emit_result_tx: Sender<ProviderResult>,
  ) -> anyhow::Result<EventTokens> {
    let media_properties_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();
      TypedEventHandler::new(move |session: &Option<MediaSession>, _| {
        println!("Media properties changed event triggered.");
        let session = session
          .as_ref()
          .expect("No session available on media properties change.");
        Self::emit_media_info(session, emit_result_tx.clone());
        windows::core::Result::Ok(())
      })
    };

    let playback_info_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();
      TypedEventHandler::new(move |session: &Option<MediaSession>, _| {
        println!("Playback info changed event triggered.");
        let session = session
          .as_ref()
          .expect("No session available on playback info change.");
        Self::emit_media_info(session, emit_result_tx.clone());
        windows::core::Result::Ok(())
      })
    };
    let timeline_properties_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();
      TypedEventHandler::new(move |session: &Option<MediaSession>, _| {
        println!("Timeline properties changed event triggered.");
        let session = session
          .as_ref()
          .expect("No session available on timeline properties change.");
        Self::emit_media_info(session, emit_result_tx.clone());
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
        .expect("Erroring emitting media provider err.");
    }
  }
}
