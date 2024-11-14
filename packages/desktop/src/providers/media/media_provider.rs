use std::{
  sync::{Arc, Mutex},
  time,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{sync::mpsc::Sender, task};
use tracing::{debug, error, info};
use windows::{
  Foundation::{EventRegistrationToken, TypedEventHandler},
  Media::Control::{
    GlobalSystemMediaTransportControlsSession as GsmtcSession,
    GlobalSystemMediaTransportControlsSessionManager as GsmtcManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as GsmtcPlaybackStatus,
  },
};

use crate::providers::{Provider, ProviderOutput, ProviderResult};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaOutput {
  pub session: Option<MediaSession>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSession {
  pub title: String,
  pub artist: String,
  pub album_title: String,
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
}

impl MediaProvider {
  pub fn new(config: MediaProviderConfig) -> MediaProvider {
    MediaProvider { _config: config }
  }

  fn emit_media_info(
    session: Option<&GsmtcSession>,
    emit_result_tx: Sender<ProviderResult>,
  ) {
    let res = match Self::media_output(session) {
      Ok(media_output) => emit_result_tx
        .blocking_send(Ok(ProviderOutput::Media(media_output)).into()),
      Err(err) => {
        error!("Error emitting media info: {:?}", err);
        emit_result_tx.blocking_send(Err(err).into())
      }
    };

    info!("Media info emitted: {:?}", res);
  }

  fn media_output(
    session: Option<&GsmtcSession>,
  ) -> anyhow::Result<MediaOutput> {
    Ok(MediaOutput {
      session: match session {
        Some(s) => Self::media_session(s)?,
        None => None,
      },
    })
  }

  fn media_session(
    session: &GsmtcSession,
  ) -> anyhow::Result<Option<MediaSession>> {
    let media_properties = session.TryGetMediaPropertiesAsync()?.get()?;
    let timeline_properties = session.GetTimelineProperties()?;
    let playback_info = session.GetPlaybackInfo()?;
    let title = media_properties.Title()?.to_string();

    // GSMTC can have a valid session, but return empty string for all
    // media properties. Check that we at least have a valid title.
    if title.is_empty() {
      return Ok(None);
    }

    let is_playing =
      playback_info.PlaybackStatus()? == GsmtcPlaybackStatus::Playing;
    let start_time =
      timeline_properties.StartTime()?.Duration as u64 / 10_000_000;
    let end_time =
      timeline_properties.EndTime()?.Duration as u64 / 10_000_000;
    let position =
      timeline_properties.Position()?.Duration as u64 / 10_000_000;

    Ok(Some(MediaSession {
      title: title.to_string(),
      artist: media_properties.Artist()?.to_string(),
      album_title: media_properties.AlbumTitle()?.to_string(),
      album_artist: media_properties.AlbumArtist()?.to_string(),
      track_number: media_properties.TrackNumber()? as u32,
      start_time,
      end_time,
      position,
      is_playing,
    }))
  }

  fn create_session_manager(
    emit_result_tx: Sender<ProviderResult>,
  ) -> anyhow::Result<()> {
    info!("Creating media session manager.");

    // Find the current GSMTC session & add listeners.
    let session_manager = GsmtcManager::RequestAsync()?.get()?;
    let current_session = session_manager.GetCurrentSession().ok();

    let event_tokens = match &current_session {
      Some(session) => Some(Self::add_session_listeners(
        session,
        emit_result_tx.clone(),
      )?),
      None => None,
    };

    // Emit initial media info.
    Self::emit_media_info(
      current_session.as_ref(),
      emit_result_tx.clone(),
    );

    let current_session = Arc::new(Mutex::new(current_session));
    let event_tokens = Arc::new(Mutex::new(event_tokens));

    // Clean up & rebind listeners when session changes.
    info!("asjdifoasjoi");
    let session_changed_handler = TypedEventHandler::new(
      move |session_manager: &Option<GsmtcManager>, _| {
        {
          let mut current_session = current_session.lock().unwrap();
          let mut event_tokens = event_tokens.lock().unwrap();

          // Remove listeners from the previous session.
          if let (Some(session), Some(token)) =
            (current_session.as_ref(), event_tokens.as_ref())
          {
            if let Err(err) =
              Self::remove_session_listeners(session, token)
            {
              error!("Failed to remove session listeners: {}", err);
            }
          }

          // Set up new session.
          let new_session =
            GsmtcManager::RequestAsync()?.get()?.GetCurrentSession()?;

          let tokens = Self::add_session_listeners(
            &new_session,
            emit_result_tx.clone(),
          )?;

          Self::emit_media_info(
            Some(&new_session),
            emit_result_tx.clone(),
          );

          *current_session = Some(new_session);
          *event_tokens = Some(tokens);
        }

        Ok(())
      },
    );

    session_manager.CurrentSessionChanged(&session_changed_handler)?;

    loop {
      std::thread::sleep(time::Duration::from_secs(1));
    }
  }

  fn remove_session_listeners(
    session: &GsmtcSession,
    tokens: &EventTokens,
  ) -> anyhow::Result<()> {
    session.RemoveMediaPropertiesChanged(
      tokens.media_properties_changed_token,
    )?;

    session
      .RemovePlaybackInfoChanged(tokens.playback_info_changed_token)?;

    session.RemoveTimelinePropertiesChanged(
      tokens.timeline_properties_changed_token,
    )?;

    Ok(())
  }

  fn add_session_listeners(
    session: &GsmtcSession,
    emit_result_tx: Sender<ProviderResult>,
  ) -> windows::core::Result<EventTokens> {
    info!("Adding session listeners.");

    let media_properties_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();

      TypedEventHandler::new(move |session: &Option<GsmtcSession>, _| {
        info!("Media properties changed event triggered.");

        if let Some(session) = session {
          Self::emit_media_info(Some(session), emit_result_tx.clone());
        }

        Ok(())
      })
    };

    let playback_info_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();

      TypedEventHandler::new(move |session: &Option<GsmtcSession>, _| {
        info!("Playback info changed event triggered.");

        if let Some(session) = session {
          Self::emit_media_info(Some(session), emit_result_tx.clone());
        }

        Ok(())
      })
    };

    let timeline_properties_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();

      TypedEventHandler::new(move |session: &Option<GsmtcSession>, _| {
        info!("Timeline properties changed event triggered.");

        if let Some(session) = session {
          Self::emit_media_info(Some(session), emit_result_tx.clone());
        }

        Ok(())
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
    task::spawn_blocking(move || {
      if let Err(err) =
        Self::create_session_manager(emit_result_tx.clone())
      {
        let _ = emit_result_tx.blocking_send(Err(err).into());
      }
    });
  }
}
