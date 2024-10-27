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

  async fn print_current_media_info(
    session: &MediaSession,
    emit_result_tx: Sender<ProviderResult>,
  ) {
    if let Ok(media_output) = Self::media_output(session) {
      info!("Title: {}", media_output.title);
      info!("Artist: {}", media_output.artist);
      info!("Album: {}", media_output.album);
      info!("Album Artist: {}", media_output.album_artist);
      emit_result_tx
        .send(Ok(ProviderOutput::Media(media_output)).into())
        .await;
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

  async fn create_session_manager(
    &mut self,
    emit_result_tx: Sender<ProviderResult>,
  ) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let session_manager = MediaManager::RequestAsync()?.get()?;
    println!("Session manager obtained.");
    let mut current_session = session_manager.GetCurrentSession()?;
    *self.current_session.lock().await = Some(current_session);

    // TODO - better way of handling error?
    match Self::add_session_listeners(
      &self.current_session.lock().await.as_ref().unwrap(),
      emit_result_tx.clone(),
    ) {
      Ok(tokens) => {
        *self.event_tokens.lock().await = Some(tokens);
      }
      Err(err) => {
        eprintln!("Error adding media session listeners: {:?}", err);
      }
    }

    let current_session = self.current_session.clone();
    let event_tokens = self.event_tokens.clone();
    let session_changed_handler = TypedEventHandler::new(
      move |session_manager: &Option<MediaManager>, _| {
        {
          // Remove listeners from the previous session.
          let current_session = current_session.clone();
          let event_tokens = event_tokens.clone();
          rt.block_on(async {
            let current_session = current_session.lock().await;
            let event_tokens = event_tokens.lock().await;
            if let Some(session) = &*current_session {
              if let Err(err) = Self::remove_session_listeners(
                session.clone(),
                event_tokens.clone(),
              ) {
                eprintln!(
                  "Error removing media session listeners: {:?}",
                  err
                );
              }
            }
          });

          let new_session =
            MediaManager::RequestAsync()?.get()?.GetCurrentSession()?;

          match Self::add_session_listeners(
            &new_session,
            emit_result_tx.clone(),
          ) {
            Ok(tokens) => {
              let mut event_tokens =
                rt.block_on(async { event_tokens.lock().await });
              *event_tokens = Some(tokens);
            }
            Err(err) => {
              eprintln!("Error adding media session listeners: {:?}", err);
            }
          }

          Self::print_current_media_info(
            &new_session,
            emit_result_tx.clone(),
          );
          rt.block_on(async {
            let mut current_session = current_session.lock().await;
            *current_session = Some(new_session);
          });
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
    session: MediaSession,
    mut event_tokens: Option<EventTokens>,
  ) -> anyhow::Result<()> {
    let token = event_tokens.expect("No event tokens available.");
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
  ) -> anyhow::Result<EventTokens> {
    let media_properties_changed_handler = {
      let emit_result_tx = emit_result_tx.clone();
      TypedEventHandler::new(move |session: &Option<MediaSession>, _| {
        println!("Media properties changed event triggered.");
        let session = session
          .as_ref()
          .expect("No session available on media properties change.");
        Self::print_current_media_info(session, emit_result_tx.clone());
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
        Self::print_current_media_info(session, emit_result_tx.clone());
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
        Self::print_current_media_info(session, emit_result_tx.clone());
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
  async fn run(&mut self, emit_result_tx: Sender<ProviderResult>) {
    if let Err(err) =
      self.create_session_manager(emit_result_tx.clone()).await
    {
      emit_result_tx.send(Err(err).into()).await;
    }
  }
}
